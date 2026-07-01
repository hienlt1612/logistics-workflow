use crate::db;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Global API token for write-protection (set at startup from config).
static API_TOKEN: OnceLock<Option<String>> = OnceLock::new();

/// Store the API token from config. None = auth disabled (public).
pub fn set_api_token(token: Option<String>) {
    let _ = API_TOKEN.set(token);
}

/// Check Authorization header for write requests. Returns Ok(()) if allowed.
fn check_auth(method: &str, raw: &str) -> Result<(), String> {
    // Only check write methods
    if method != "POST" && method != "PATCH" && method != "DELETE" {
        return Ok(());
    }
    let required = API_TOKEN.get().and_then(|t| t.as_deref());
    match required {
        None | Some("") => Ok(()), // auth disabled or empty token
        Some(token) => {
            // Extract Authorization: Bearer *** from headers
            let auth_header = raw.lines()
                .find(|l| l.to_lowercase().starts_with("authorization:"))
                .unwrap_or("");
            let provided = auth_header
                .trim_start_matches(|c: char| c != ' ' && c != ':')
                .trim_start_matches(':')
                .trim()
                .trim_start_matches("Bearer ")
                .trim();
            if provided == token {
                Ok(())
            } else {
                Err(format!("Invalid or missing API token. Use Authorization: Bearer *** header for write operations."))
            }
        }
    }
}

/// Extract user role from X-User-Role header. Returns "user" if not found.
fn extract_role(raw: &str) -> String {
    raw.lines()
        .find(|l| l.to_lowercase().starts_with("x-user-role:"))
        .and_then(|l| l.split_once(':').map(|(_, v)| v.trim().to_string()))
        .unwrap_or_else(|| "user".into())
}

/// Check if the request is from an admin user.
fn is_admin(raw: &str) -> bool {
    extract_role(raw).eq_ignore_ascii_case("admin")
}

/// Regenerate the AppData.qml singleton with fresh shipment data.
/// Called at startup and after every mutation.
pub fn regenerate_appdata() {
    let rt = crate::TOKIO_RT.get().expect("Tokio runtime not initialized");
    let pool = db::pool();

    let shipments = rt.block_on(async {
        db::queries::list_shipments(pool).await
            .unwrap_or_else(|e| {
                log::error!("Failed to list shipments for AppData: {}", e);
                vec![]
            })
    });

    let json = serde_json::to_string(&shipments).unwrap_or_else(|e| {
        log::error!("Failed to serialize shipments: {}", e);
        "[]".into()
    });

    // Compute summary stats for dashboard
    let mut draft = 0u32; let mut docs = 0u32; let mut customs = 0u32;
    let mut checklist = 0u32; let mut complete = 0u32; let mut telex = 0u32;
    for s in &shipments {
        match s.status.as_str() {
            "DRAFT" => draft += 1,
            "DOCUMENTS_READY" => docs += 1,
            "CUSTOMS_CLEARED" => customs += 1,
            "CHECKLIST_IN_PROGRESS" => checklist += 1,
            "COMPLETE" => complete += 1,
            "TELEX_RELEASED" => telex += 1,
            _ => {}
        }
    }
    let summary = serde_json::json!({
        "total": shipments.len(),
        "draft": draft,
        "documents": docs,
        "customs": customs,
        "checklist": checklist + complete,
        "telex": telex,
    });
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    let appdata_qml = format!(
        r#"pragma Singleton
import QtQml

QtObject {{
    property string allShipmentsJson: '{0}'
    property string summaryJson: '{1}'
}}
"#,
        escape_json(&json),
        escape_json(&summary_json)
    );

    let qml_dir = qml_dir();
    fs::create_dir_all(&qml_dir).unwrap_or_else(|e| {
        log::error!("Failed to create qml dir: {}", e)
    });

    fs::write(qml_dir.join("AppData.qml"), &appdata_qml)
        .unwrap_or_else(|e| log::error!("Failed to write AppData.qml: {}", e));

    fs::write(
        qml_dir.join("qmldir"),
        "singleton AppData 1.0 AppData.qml\nsingleton Theme 1.0 Theme.qml\n"
    )
    .unwrap_or_else(|e| log::error!("Failed to write qmldir: {}", e));

    log::info!("AppData.qml regenerated ({} shipments)", shipments.len());
}

/// Create a new shipment from JSON input, then regenerate AppData.
pub fn create_shipment(json_str: &str) -> Result<String, String> {
    let rt = crate::TOKIO_RT.get().ok_or("Tokio runtime not initialized")?;
    let pool = db::pool();

    let input: db::queries::CreateShipmentInput = serde_json::from_str(json_str)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    let shipment = rt.block_on(async {
        db::queries::create_shipment(pool, &input).await
    })
    .map_err(|e| format!("DB error: {}", e))?;

    regenerate_appdata();
    Ok(serde_json::to_string(&shipment).unwrap_or_default())
}

/// Update shipment fields from JSON, then regenerate AppData.
pub fn update_shipment(id: i64, json_str: &str) -> Result<String, String> {
    let rt = crate::TOKIO_RT.get().ok_or("Tokio runtime not initialized")?;
    let pool = db::pool();

    let fields: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json_str)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    let shipment = rt.block_on(async {
        db::queries::update_shipment_fields(pool, id, &fields).await
    })
    .map_err(|e| format!("DB error: {}", e))?;

    regenerate_appdata();
    Ok(serde_json::to_string(&shipment).unwrap_or_default())
}

/// Toggle a boolean checklist field, then regenerate AppData.
pub fn toggle_checklist(id: i64, field: &str, value: bool) -> Result<String, String> {
    let rt = crate::TOKIO_RT.get().ok_or("Tokio runtime not initialized")?;
    let pool = db::pool();

    let shipment = rt.block_on(async {
        db::queries::set_checklist_bool(pool, id, field, value).await
    })
    .map_err(|e| format!("DB error: {}", e))?;

    regenerate_appdata();
    Ok(serde_json::to_string(&shipment).unwrap_or_default())
}

/// Export all shipments to workbook1-format Excel file.
pub async fn export_workbook1_async(pool: &sqlx::PgPool) -> Result<Vec<u8>, String> {
    let shipments = db::queries::list_shipments(pool).await
        .map_err(|e| format!("DB error: {e}"))?;

    let bytes = write_workbook1_xlsx_binary(&shipments)
        .map_err(|e| format!("Excel write error: {e}"))?;

    log::info!("Exported {} shipments to XLSX ({:.1} KB)", shipments.len(), bytes.len() as f64 / 1024.0);
    Ok(bytes)
}

fn write_workbook1_xlsx_binary(
    shipments: &[db::queries::Shipment],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use rust_xlsxwriter::{Workbook, Format, Color, FormatBorder};

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name("Shipments")?;

    let headers = [
        "Shipment Ref", "Status",
        "SC/PO ID", "Date of SC/PO", "SC/PO made by", "For Buyer",
        "Booking Number", "Shipping Line", "Port of Loading", "Warehouse",
        "Loading Plan", "Shipper/Exporter", "Consignee", "ETD",
        "Invoice #", "Invoice Date", "Total Value USD", "Drafts Date",
        "Bill of Lading #", "Customs Date", "Customs #", "Customs Status",
        "BL Received", "Charges Paid", "CO Received", "Phyto Received",
        "Docs Confirmed", "Payment Date", "Prepayment", "Remaining",
        "Payment Received", "Originals Status", "Originals Sent",
        "Contact & Sending", "Telex Released", "Created At", "Updated At",
    ];

    let hdr = Format::new().set_bold().set_background_color(Color::RGB(0x2C3E50)).set_font_color(Color::RGB(0xFFFFFF)).set_border(FormatBorder::Thin);
    let data = Format::new().set_border(FormatBorder::Thin);
    let ok_fmt = Format::new().set_border(FormatBorder::Thin).set_font_color(Color::RGB(0x27AE60)).set_bold();
    let no_fmt = Format::new().set_border(FormatBorder::Thin).set_font_color(Color::RGB(0x95A5A6));

    for (i, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, i as u16, *h, &hdr)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    for (row, s) in shipments.iter().enumerate() {
        let r = (row + 1) as u32;
        let mut c: u16 = 0;

        macro_rules! w { ($v:expr) => {{ let x = c; c += 1; sheet.write_string_with_format(r, x, $v, &data)?; }}; }
        macro_rules! wb { ($v:expr) => {{ let x = c; c += 1; let f = if $v { &ok_fmt } else { &no_fmt }; sheet.write_string_with_format(r, x, if $v { "Yes" } else { "No" }, f)?; }}; }
        macro_rules! wd { ($v:expr) => {{ let x = c; c += 1; let s = $v.map(|d: chrono::NaiveDate| d.to_string()).unwrap_or_default(); sheet.write_string_with_format(r, x, &s, &data)?; }}; }
        macro_rules! wc { ($v:expr) => {{ let x = c; c += 1; if let Some(val) = $v { let f = val.to_string().parse::<f64>().unwrap_or(0.0); sheet.write_number_with_format(r, x, f, &data)?; } else { sheet.write_string_with_format(r, x, "", &data)?; } }}; }

        w!(&s.shipment_ref);
        w!(&s.status);
        w!(s.sc_po_id.as_deref().unwrap_or(""));
        wd!(&s.sc_po_date);
        w!(s.sc_po_by.as_deref().unwrap_or(""));
        w!(s.buyer_name.as_deref().unwrap_or(""));
        w!(s.booking_number.as_deref().unwrap_or(""));
        w!(s.shipping_line.as_deref().unwrap_or(""));
        w!(s.origin_port.as_deref().unwrap_or(""));
        w!(s.warehouse_loc.as_deref().unwrap_or(""));
        w!(s.loading_plan.as_deref().unwrap_or(""));
        w!(s.shipper_name.as_deref().unwrap_or(""));
        w!(s.consignee_name.as_deref().unwrap_or(""));
        wd!(&s.etd);
        w!(s.invoice_number.as_deref().unwrap_or(""));
        wd!(&s.invoice_date);
        wc!(&s.total_value_usd);
        wd!(&s.drafts_date);
        w!(s.bill_of_lading.as_deref().unwrap_or(""));
        wd!(&s.customs_date);
        w!(s.customs_number.as_deref().unwrap_or(""));
        w!(s.customs_status.as_deref().unwrap_or(""));
        wb!(s.bl_received);
        wb!(s.charges_paid);
        wb!(s.co_received);
        wb!(s.phyto_received);
        wb!(s.docs_confirmed);
        wd!(&s.prepayment_date);
        wc!(&s.prepayment_amt);
        wc!(&s.remaining_amt);
        wb!(s.payment_received);
        w!(s.originals_status.as_deref().unwrap_or(""));
        wd!(&s.originals_sent);
        w!(s.originals_description.as_deref().unwrap_or(""));
        wb!(s.telex_released);
        w!(&s.created_at.to_string());
        w!(&s.updated_at.to_string());
    }

    for col in 0..headers.len() as u16 {
        sheet.set_column_width(col, 16)?;
    }

    let buf = workbook.save_to_buffer()?;
    Ok(buf)
}


fn dirs_documents() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h).join("Documents"))
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

/// Get the qml directory path.
fn qml_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("qml")
}

/// Escape JSON string for embedding in QML single-quoted strings.
/// MUST escape: \\ ' \n \r \t \b \f
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace('\u{0008}', "\\b")
        .replace('\u{000C}', "\\f")
}

// ── REST API HTTP server ──

const API_PORT: u16 = 19876;

/// Start the REST API HTTP server with CORS support.
/// Routes:
///   GET    /api/dashboard           → summary stats
///   GET    /api/shipments           → list all (?status= filter)
///   GET    /api/shipments/:id       → get one
///   POST   /api/shipments           → create
///   PATCH  /api/shipments/:id       → update fields
///   PATCH  /api/shipments/:id/checklist → toggle boolean
///   GET    /api/export/all          → download workbook1 xlsx (base64-encoded JSON)
///   OPTIONS *                       → CORS preflight
pub fn start_command_server() {
    let rt = crate::TOKIO_RT.get().expect("Tokio runtime not initialized");
    let bind_host = std::env::var("LW_BIND_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let listener = std::net::TcpListener::bind(format!("{bind_host}:{API_PORT}"))
        .expect("Failed to bind API server");
    listener.set_nonblocking(false).ok();
    log::info!("REST API server listening on http://{bind_host}:{API_PORT}");

    rt.spawn(async move {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    use std::io::{Read, Write};
                    let mut buf = [0u8; 16384];
                    if let Ok(n) = stream.read(&mut buf) {
                        let raw = String::from_utf8_lossy(&buf[..n]).to_string();
                        let (status, content_type, body) = route_request(&raw).await;
                        let resp = format!(
                            "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PATCH, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization, X-User-Role\r\nContent-Length: {}\r\n\r\n",
                            body.as_bytes().len()
                        );
                        let _ = stream.write_all(resp.as_bytes());
                        let _ = stream.write_all(body.as_bytes());
                    }
                }
                Err(e) => log::error!("API stream error: {e}"),
            }
        }
    });
}

/// Parse HTTP request and dispatch to handler.
/// Returns (status_line, content_type, body).
async fn route_request(raw: &str) -> (&'static str, &'static str, String) {
    // Parse request line: METHOD /path HTTP/1.1
    let first_line = raw.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        return ("400 Bad Request", "application/json", json_error("BAD_REQUEST", "Malformed request"));
    }
    let method = parts[0];
    let full_path = parts[1];

    // Split path from query string
    let (path, query) = match full_path.find('?') {
        Some(i) => (&full_path[..i], &full_path[i+1..]),
        None => (full_path, ""),
    };

    // Extract body after \r\n\r\n
    let body = match raw.find("\r\n\r\n") {
        Some(pos) => raw[pos + 4..].trim().to_string(),
        None => String::new(),
    };

    // Parse query params
    let query_params = parse_query(query);
    let pool = db::pool();

    // ── CORS preflight ──
    if method == "OPTIONS" {
        return ("204 No Content", "text/plain", String::new());
    }

    // ── Auth check for write operations ──
    // Skip auth for login endpoint + health
    if path != "/api/login" && path != "/health" {
        if let Err(msg) = check_auth(method, raw) {
            return ("401 Unauthorized", "application/json", json_error("UNAUTHORIZED", &msg));
        }
    }

    // ── Route matching ──
    // GET /health — Docker healthcheck
    if method == "GET" && path == "/health" {
        return ("200 OK", "application/json",
            serde_json::json!({ "status": "healthy", "version": "0.2.0" }).to_string());
    }

    // POST /api/login
    if method == "POST" && path == "/api/login" {
        return handle_login(pool, &body).await;
    }

    // GET /api/dashboard
    if method == "GET" && path == "/api/dashboard" {
        return handle_dashboard(pool).await;
    }

    // GET /api/export/all
    if method == "GET" && path == "/api/export/all" {
        return handle_export_all().await;
    }

    // GET /api/shipments (list)
    if method == "GET" && path == "/api/shipments" {
        let status_filter = query_params.get("status").map(|s| s.as_str());
        let page: i64 = query_params.get("page").and_then(|s| s.parse().ok()).unwrap_or(1);
        let page_size: i64 = query_params.get("pageSize").and_then(|s| s.parse().ok()).unwrap_or(20);
        return handle_list_shipments(pool, status_filter, page, page_size).await;
    }

    // GET /api/shipments/:id/containers (must be before generic :id)
    if method == "GET" && path.starts_with("/api/shipments/") && path.ends_with("/containers") {
        let rest = &path["/api/shipments/".len()..];
        let id_str = rest.trim_end_matches("/containers");
        if let Ok(id) = id_str.parse::<i64>() {
            return handle_list_containers(pool, id).await;
        }
        return ("400 Bad Request", "application/json", json_error("BAD_REQUEST", "Invalid shipment ID"));
    }

    // GET /api/shipments/:id
    if method == "GET" && path.starts_with("/api/shipments/") {
        let id_str = &path["/api/shipments/".len()..];
        if let Ok(id) = id_str.parse::<i64>() {
            return handle_get_shipment(pool, id).await;
        }
        return ("400 Bad Request", "application/json", json_error("BAD_REQUEST", "Invalid shipment ID"));
    }

    // POST /api/shipments (create)
    if method == "POST" && path == "/api/shipments" {
        return handle_create_shipment(pool, &body).await;
    }

    // PATCH /api/shipments/batch (bulk status advance)
    if method == "PATCH" && path == "/api/shipments/batch" {
        return handle_batch_update(pool, &body).await;
    }

    // PATCH /api/shipments/:id/checklist
    if method == "PATCH" && path.ends_with("/checklist") {
        let rest = &path["/api/shipments/".len()..];
        let id_str = rest.trim_end_matches("/checklist");
        if let Ok(id) = id_str.parse::<i64>() {
            return handle_toggle_checklist(pool, id, &body).await;
        }
        return ("400 Bad Request", "application/json", json_error("BAD_REQUEST", "Invalid shipment ID"));
    }

    // DELETE /api/shipments/:id
    if method == "DELETE" && path.starts_with("/api/shipments/") {
        let id_str = &path["/api/shipments/".len()..];
        if let Ok(id) = id_str.parse::<i64>() {
            return handle_delete_shipment(pool, id, raw).await;
        }
        return ("400 Bad Request", "application/json", json_error("BAD_REQUEST", "Invalid shipment ID"));
    }

    // ── Shipping Calls ──
    // ponytail: reuse same handler pattern as shipments.

    // GET /api/shipping-calls
    if method == "GET" && path == "/api/shipping-calls" {
        return handle_list_calls(pool).await;
    }

    // GET /api/shipping-calls/:id/warehouses (must be before generic :id)
    if method == "GET" && path.starts_with("/api/shipping-calls/") && path.ends_with("/warehouses") {
        let rest = &path["/api/shipping-calls/".len()..];
        let id_str = rest.trim_end_matches("/warehouses");
        if let Ok(id) = id_str.parse::<i64>() {
            return handle_list_call_warehouses(pool, id).await;
        }
        return ("400 Bad Request", "application/json", json_error("BAD_REQUEST", "Invalid call ID"));
    }

    // POST /api/shipping-calls
    if method == "POST" && path == "/api/shipping-calls" {
        return handle_create_call(pool, &body).await;
    }

    // GET /api/shipping-calls/:id
    if method == "GET" && path.starts_with("/api/shipping-calls/") {
        let id_str = &path["/api/shipping-calls/".len()..];
        if let Ok(id) = id_str.parse::<i64>() {
            return handle_get_call(pool, id).await;
        }
        return ("400 Bad Request", "application/json", json_error("BAD_REQUEST", "Invalid call ID"));
    }

    // POST /api/containers
    if method == "POST" && path == "/api/containers" {
        return handle_create_container(pool, &body).await;
    }

    // PATCH /api/shipments/:id
    if method == "PATCH" && path.starts_with("/api/shipments/") {
        let id_str = &path["/api/shipments/".len()..];
        if let Ok(id) = id_str.parse::<i64>() {
            return handle_update_shipment(pool, id, &body, raw).await;
        }
        return ("400 Bad Request", "application/json", json_error("BAD_REQUEST", "Invalid shipment ID"));
    }

    // 404
    ("404 Not Found", "application/json", json_error("NOT_FOUND", &format!("No route for {method} {path}")))
}

// ── Query string parser ──

fn parse_query(query: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            map.insert(
                url_decode(k),
                url_decode(v),
            );
        }
    }
    map
}

fn url_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                out.push(byte as char);
            }
        } else if c == '+' {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

// ── Route handlers ──

// ── Handler: POST /api/login ──
async fn handle_login(pool: &sqlx::PgPool, body: &str) -> (&'static str, &'static str, String) {
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct LoginInput { username: String, password: String }

    let input: LoginInput = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &format!("Invalid JSON: {e}"))),
    };

    match db::queries::authenticate_user(pool, &input.username, &input.password).await {
        Ok(Some(user)) => {
            // Return the real API token from config — client stores it for write auth
            let api_token = API_TOKEN.get()
                .and_then(|t| t.as_deref().map(|s| s.to_string()))
                .unwrap_or_default();
            let resp = serde_json::json!({
                "token": api_token,
                "username": user.username,
                "role": user.role,
            });
            ("200 OK", "application/json", serde_json::to_string(&resp).unwrap_or_default())
        }
        Ok(None) => ("401 Unauthorized", "application/json", json_error("UNAUTHORIZED", "Invalid username or password")),
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_dashboard(pool: &sqlx::PgPool) -> (&'static str, &'static str, String) {
    match db::queries::list_shipments(pool).await {
        Ok(shipments) => {
            let mut draft = 0u32; let mut docs = 0u32; let mut customs = 0u32;
            let mut checklist = 0u32; let mut complete = 0u32; let mut telex = 0u32;
            for s in &shipments {
                match s.status.as_str() {
                    "DRAFT" => draft += 1,
                    "DOCUMENTS_READY" => docs += 1,
                    "CUSTOMS_CLEARED" => customs += 1,
                    "CHECKLIST_IN_PROGRESS" => checklist += 1,
                    "COMPLETE" => complete += 1,
                    "TELEX_RELEASED" => telex += 1,
                    _ => {}
                }
            }
            let summary = serde_json::json!({
                "total": shipments.len(),
                "draft": draft,
                "documents": docs,
                "customs": customs,
                "checklist": checklist + complete,
                "telex": telex,
            });
            let body = serde_json::to_string(&summary).unwrap_or_default();
            regenerate_appdata_inline(pool).await;
            ("200 OK", "application/json", body)
        }
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_list_shipments(pool: &sqlx::PgPool, status_filter: Option<&str>, page: i64, page_size: i64) -> (&'static str, &'static str, String) {
    match db::queries::list_shipments_paginated(pool, status_filter, page, page_size).await {
        Ok((rows, total)) => {
            let total_pages = ((total as f64) / (page_size as f64)).ceil() as i64;
            let body = serde_json::json!({
                "data": rows,
                "pagination": {
                    "page": page,
                    "pageSize": page_size,
                    "totalItems": total,
                    "totalPages": total_pages
                }
            });
            ("200 OK", "application/json", serde_json::to_string(&body).unwrap_or_default())
        }
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_get_shipment(pool: &sqlx::PgPool, id: i64) -> (&'static str, &'static str, String) {
    match db::queries::get_shipment(pool, id).await {
        Ok(Some(s)) => {
            let body = serde_json::to_string(&s).unwrap_or_default();
            ("200 OK", "application/json", body)
        }
        Ok(None) => ("404 Not Found", "application/json", json_error("NOT_FOUND", &format!("Shipment {id} not found"))),
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_create_shipment(pool: &sqlx::PgPool, body: &str) -> (&'static str, &'static str, String) {
    let input: db::queries::CreateShipmentInput = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &format!("Invalid JSON: {e}"))),
    };
    match db::queries::create_shipment(pool, &input).await {
        Ok(s) => {
            let json = serde_json::to_string(&s).unwrap_or_default();
            regenerate_appdata_inline(pool).await;
            ("201 Created", "application/json", json)
        }
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_update_shipment(pool: &sqlx::PgPool, id: i64, body: &str, raw: &str) -> (&'static str, &'static str, String) {
    let fields: serde_json::Map<String, serde_json::Value> = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &format!("Invalid JSON: {e}"))),
    };

    // Check: reverting from TELEX_RELEASED requires admin
    if let Some(new_status) = fields.get("status").and_then(|v| v.as_str()) {
        if new_status != "TELEX_RELEASED" {
            // User is trying to change status away from TELEX_RELEASED
            // Fetch current shipment to check its current status
            match db::queries::get_shipment(pool, id).await {
                Ok(Some(current)) => {
                    if current.status == "TELEX_RELEASED" && !is_admin(raw) {
                        return ("403 Forbidden", "application/json",
                            json_error("FORBIDDEN", "Only admin can revert a TELEX_RELEASED shipment"));
                    }
                }
                Ok(None) => return ("404 Not Found", "application/json",
                    json_error("NOT_FOUND", &format!("Shipment {id} not found"))),
                Err(e) => return ("500 Internal Server Error", "application/json",
                    json_error("DB_ERROR", &e.to_string())),
            }
        }
    }

    match db::queries::update_shipment_fields(pool, id, &fields).await {
        Ok(s) => {
            let json = serde_json::to_string(&s).unwrap_or_default();
            regenerate_appdata_inline(pool).await;
            ("200 OK", "application/json", json)
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("Invalid field") {
                ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &msg))
            } else {
                ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &msg))
            }
        }
    }
}

async fn handle_toggle_checklist(pool: &sqlx::PgPool, id: i64, body: &str) -> (&'static str, &'static str, String) {
    #[derive(serde::Deserialize)]
    struct ChecklistToggle { field: String, value: bool }

    let toggle: ChecklistToggle = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &format!("Invalid JSON: {e}"))),
    };
    match db::queries::set_checklist_bool(pool, id, &toggle.field, toggle.value).await {
        Ok(s) => {
            let json = serde_json::to_string(&s).unwrap_or_default();
            regenerate_appdata_inline(pool).await;
            ("200 OK", "application/json", json)
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("Invalid") {
                ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &msg))
            } else {
                ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &msg))
            }
        }
    }
}

async fn handle_delete_shipment(pool: &sqlx::PgPool, id: i64, raw: &str) -> (&'static str, &'static str, String) {
    // Only admin can delete shipments
    if !is_admin(raw) {
        return ("403 Forbidden", "application/json", json_error("FORBIDDEN", "Only admin can delete shipments"));
    }
    match db::queries::delete_shipment(pool, id).await {
        Ok(true) => {
            regenerate_appdata_inline(pool).await;
            ("200 OK", "application/json", r#"{"ok":true}"#.into())
        }
        Ok(false) => ("404 Not Found", "application/json", json_error("NOT_FOUND", &format!("Shipment {id} not found"))),
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_batch_update(pool: &sqlx::PgPool, body: &str) -> (&'static str, &'static str, String) {
    #[derive(serde::Deserialize)]
    struct BatchInput { ids: Vec<i64>, status: String }

    let input: BatchInput = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &format!("Invalid JSON: {e}"))),
    };
    match db::queries::batch_advance_status(pool, &input.ids, &input.status).await {
        Ok(count) => {
            regenerate_appdata_inline(pool).await;
            let body = format!(r#"{{"updated":{count}}}"#);
            ("200 OK", "application/json", body)
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("Invalid") {
                ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &msg))
            } else {
                ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &msg))
            }
        }
    }
}

/// Handle GET /api/export/all — returns base64-encoded XLSX as JSON
async fn handle_export_all() -> (&'static str, &'static str, String) {
    let pool = db::pool();
    match crate::bridge::export_workbook1_async(pool).await {
        Ok(bytes) => {
            use base64::Engine;
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            let json = serde_json::json!({
                "data": b64,
                "filename": "workbook1_export.xlsx",
                "contentType": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            });
            ("200 OK", "application/json", serde_json::to_string(&json).unwrap_or_default())
        }
        Err(e) => ("500 Internal Server Error", "application/json", json_error("EXPORT_ERROR", &e)),
    }
}

/// Inline AppData regeneration (no block_on needed — we're in async context).
async fn regenerate_appdata_inline(pool: &sqlx::PgPool) {
    let shipments = db::queries::list_shipments(pool).await.unwrap_or_default();
    let json = serde_json::to_string(&shipments).unwrap_or_default();

    let mut draft = 0u32; let mut docs = 0u32; let mut customs = 0u32;
    let mut checklist = 0u32; let mut complete = 0u32; let mut telex = 0u32;
    for s in &shipments {
        match s.status.as_str() {
            "DRAFT" => draft += 1, "DOCUMENTS_READY" => docs += 1,
            "CUSTOMS_CLEARED" => customs += 1, "CHECKLIST_IN_PROGRESS" => checklist += 1,
            "COMPLETE" => complete += 1, "TELEX_RELEASED" => telex += 1, _ => {}
        }
    }
    let summary = serde_json::json!({"total":shipments.len(),"draft":draft,"documents":docs,"customs":customs,"checklist":checklist+complete,"telex":telex});
    let summary_json = serde_json::to_string(&summary).unwrap_or_default();

    let qml = format!(r#"pragma Singleton
import QtQml

QtObject {{
    property string allShipmentsJson: '{0}'
    property string summaryJson: '{1}'
}}
"#, escape_json(&json), escape_json(&summary_json));
    let qd = qml_dir();
    let _ = fs::write(qd.join("AppData.qml"), &qml);
    let _ = fs::write(qd.join("qmldir"), "singleton AppData 1.0 AppData.qml\nsingleton Theme 1.0 Theme.qml\n");
}

// ── Shipping Call handlers ──
// ponytail: reuse handle_* pattern from shipments.

async fn handle_list_calls(pool: &sqlx::PgPool) -> (&'static str, &'static str, String) {
    match db::queries::list_shipping_calls(pool).await {
        Ok(calls) => ("200 OK", "application/json", serde_json::to_string(&calls).unwrap_or_default()),
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_get_call(pool: &sqlx::PgPool, id: i64) -> (&'static str, &'static str, String) {
    match db::queries::get_shipping_call(pool, id).await {
        Ok(Some(c)) => ("200 OK", "application/json", serde_json::to_string(&c).unwrap_or_default()),
        Ok(None) => ("404 Not Found", "application/json", json_error("NOT_FOUND", &format!("Call {id} not found"))),
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_create_call(pool: &sqlx::PgPool, body: &str) -> (&'static str, &'static str, String) {
    let input: db::queries::CreateShippingCallInput = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &format!("Invalid JSON: {e}"))),
    };
    match db::queries::create_shipping_call(pool, &input).await {
        Ok(call) => {
            // ponytail: create warehouses inline if provided in same POST
            if let Some(ref warehouses) = input.warehouses {
                for w in warehouses {
                    let _ = db::queries::create_call_warehouse(pool, &db::queries::CreateWarehouseInput {
                        shipping_call_id: call.id,
                        warehouse_name: w.warehouse_name.clone(),
                        planned_containers: w.planned_containers,
                    }).await;
                }
            }
            ("201 Created", "application/json", serde_json::to_string(&call).unwrap_or_default())
        }
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_list_call_warehouses(pool: &sqlx::PgPool, call_id: i64) -> (&'static str, &'static str, String) {
    match db::queries::list_call_warehouses(pool, call_id).await {
        Ok(warehouses) => ("200 OK", "application/json", serde_json::to_string(&warehouses).unwrap_or_default()),
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_create_container(pool: &sqlx::PgPool, body: &str) -> (&'static str, &'static str, String) {
    let input: db::queries::CreateContainerInput = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => return ("400 Bad Request", "application/json", json_error("VALIDATION_ERROR", &format!("Invalid JSON: {e}"))),
    };
    match db::queries::create_container(pool, &input).await {
        Ok(c) => ("201 Created", "application/json", serde_json::to_string(&c).unwrap_or_default()),
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

async fn handle_list_containers(pool: &sqlx::PgPool, shipment_id: i64) -> (&'static str, &'static str, String) {
    match db::queries::list_containers(pool, shipment_id).await {
        Ok(containers) => ("200 OK", "application/json", serde_json::to_string(&containers).unwrap_or_default()),
        Err(e) => ("500 Internal Server Error", "application/json", json_error("DB_ERROR", &e.to_string())),
    }
}

// ── JSON helpers ──

/// Build a standard API error JSON string.
fn json_error(code: &str, message: &str) -> String {
    let msg = message.replace('\\', "\\\\").replace('"', "\\\"");
    format!(r#"{{"error":{{"code":"{code}","message":"{msg}"}}}}"#)
}
