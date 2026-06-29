# Spec: Logistics Workflow Desktop

> Rebuild from workbook1.xlsx business process. Single-window 4-step document workflow.

## Objective

Build a Rust+Qt6 desktop application that exactly mirrors the 30-field business process defined in `workbook1.xlsx`. Users create shipments and progress them through 4 sequential phases: CREATE → DRAFT DOCUMENTATION → CUSTOMS CLEARANCE → DOCUMENT CHECKLIST → TELEX RELEASE.

### User Stories

- Admin creates new shipment with SC/PO, booking, carrier, port, warehouse info
- Manager fills draft documentation (shipper, consignee, invoice, bill of lading)
- Accounting records customs clearance details
- All roles track document checklist (BL, CO, Phyto, charges, payment, originals)
- Manager finalizes with telex release
- Export completed shipments to Excel in workbook1 format

### Success Criteria

- [ ] User can create, read, update all 30 workbook1 fields
- [ ] Shipment progresses through 5 statuses sequentially
- [ ] Each of the 4 workflow steps has its own form with role-colored header
- [ ] Document checklist has 11 interactive boolean/date/number fields
- [ ] Telex release marks shipment as terminal state
- [ ] Excel export produces workbook1-format 30-column report
- [ ] All Rust functions tested (unit + integration)
- [ ] Zero clippy warnings
- [ ] Offscreen QML loads without errors (QT_QPA_PLATFORM=offscreen)

## Tech Stack

| Component | Technology | Version |
|---|---|---|
| Language | Rust | 2021 edition |
| GUI | Qt6 QML via cxx-qt | 0.6 |
| Database | PostgreSQL via sqlx | 0.8 |
| Async | tokio | 1 |
| Excel | rust_xlsxwriter | 0.80 |
| Serialization | serde + serde_json | 1 |
| Dates | chrono | 0.4 |
| Logging | log + env_logger | 0.4 / 0.11 |
| Errors | anyhow | 1 |

## Commands

```
Build:    cargo build
Release:  cargo build --release
Run:      QT_QPA_PLATFORM=xcb cargo run
Test:     cargo test
Test (offscreen): QT_QPA_PLATFORM=offscreen cargo test
Lint:     cargo clippy -- -D warnings
Clean:    cargo clean
```

## Project Structure

```
~/logistics-workflow/
├── Cargo.toml
├── build.rs
├── resources/
│   └── qml.qrc
├── qml/
│   ├── MainWindow.qml          # Split: sidebar + workflow canvas
│   ├── ShipmentSidebar.qml      # Shipment list, new button, search
│   ├── WorkflowProgress.qml     # 4-step indicator bar
│   ├── Step1Create.qml          # ADMIN: SC/PO fields (red)
│   ├── Step2Draft.qml           # MANAGER: invoice, BL fields (blue)
│   ├── Step3Customs.qml         # ACCOUNTING: customs fields (orange)
│   ├── Step4Checklist.qml       # ALL: document checklist + telex (multi)
│   └── Theme.qml                # Colors, fonts, role colors
├── src/
│   ├── main.rs                  # App entry: tokio init → AppData → Qt loop
│   ├── lib.rs                   # Re-exports
│   ├── config.rs                # TOML config loader
│   ├── db/
│   │   ├── mod.rs               # Global OnceLock<PgPool>, init_pool()
│   │   ├── schema.rs            # CREATE TABLE statements
│   │   ├── queries.rs           # CRUD operations
│   │   └── seed.rs              # workbook1 5-sample seed data
│   └── bridge.rs                # AppData.qml generation + Excel export
├── tests/
│   ├── db_tests.rs
│   └── integration_tests.rs
├── config.toml                  # DB connection params (gitignored)
├── config.toml.example          # Template for new setups
├── run.sh                       # Launcher script
├── logistics-workflow.desktop   # Desktop entry
└── workbook1.xlsx               # Reference document
```

## Code Style

```rust
// main.rs — entry point
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
use std::sync::OnceLock;

static TOKIO_RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn main() {
    env_logger::init();
    log::info!("Starting Logistics Workflow");

    // Init DB pool
    let rt = tokio::runtime::Runtime::new().unwrap();
    let config = config::load().expect("config.toml");
    rt.block_on(async {
        db::init_pool(&config.db_url()).await.expect("DB connection");
        db::schema::ensure_schema().await.expect("Schema");
        db::seed::seed_if_empty().await.expect("Seed data");
    });
    TOKIO_RT.set(rt).ok();

    // Generate AppData before Qt starts
    bridge::regenerate_appdata();

    // Launch Qt
    let mut app = QGuiApplication::new();
    app.pin_mut().set_application_name(&QString::from("Logistics Workflow"));

    let mut engine = QQmlApplicationEngine::new();
    let qml_path = std::env::current_dir()
        .unwrap_or_default()
        .join("qml")
        .join("MainWindow.qml");
    engine.pin_mut().load(&QUrl::from_local_file(
        &QString::from(qml_path.to_str().unwrap())
    ));

    let exit_code = app.pin_mut().exec();
    std::process::exit(exit_code);
}
```

```rust
// db/queries.rs — Runtime query pattern (no macros)
pub async fn list_shipments(pool: &PgPool) -> Result<Vec<Shipment>, sqlx::Error> {
    sqlx::query_as::<_, Shipment>(
        "SELECT * FROM shipments ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
}
```

Naming: `snake_case` Rust, `camelCase` QML. Structs: `Shipment` (row mapping), `ShipmentForm` (create/update input).

## Testing Strategy

- **Framework:** `cargo test` with `#[tokio::test]` for async DB tests
- **Unit tests:** Every query function, config loader, bridge function
- **Integration tests:** Full CRUD cycle, status progression, Excel export
- **QML tests:** Offscreen loading verifies QML parsing, module resolution
- **Coverage:** All query paths, all status transitions, edge cases (empty DB, invalid input)
- **Pattern:** Each test creates its own pool, runs independently, rolls back mutations

## Database Schema

### Table: shipments (30 fields)

```sql
CREATE TABLE shipments (
    id              SERIAL PRIMARY KEY,
    shipment_ref    VARCHAR(30) UNIQUE NOT NULL,
    status          VARCHAR(20) NOT NULL DEFAULT 'DRAFT',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Phase 1: CREATE (ADMIN)
    sc_po_id        VARCHAR(30),
    sc_po_date      DATE,
    sc_po_by        VARCHAR(60),
    buyer_name      VARCHAR(80),
    booking_number  VARCHAR(30),
    shipping_line   VARCHAR(50),
    origin_port     VARCHAR(80),
    warehouse_loc   VARCHAR(120),
    loading_plan    TEXT,

    -- Phase 2: DRAFT (MANAGER)
    shipper_name    VARCHAR(100),
    consignee_name  VARCHAR(100),
    etd             DATE,
    invoice_number  VARCHAR(30),
    invoice_date    DATE,
    total_value_usd NUMERIC(12,2),
    drafts_date     DATE,
    bill_of_lading  VARCHAR(30),

    -- Phase 3: CUSTOMS (ACCOUNTING)
    customs_date    DATE,
    customs_number  VARCHAR(30),
    customs_status  VARCHAR(10),  -- red/yellow/green

    -- Phase 4: CHECKLIST (ALL ROLES)
    bl_received     BOOLEAN DEFAULT false,
    charges_paid    BOOLEAN DEFAULT false,
    co_received     BOOLEAN DEFAULT false,
    phyto_received  BOOLEAN DEFAULT false,
    docs_confirmed  BOOLEAN DEFAULT false,
    prepayment_date DATE,
    prepayment_amt  NUMERIC(12,2),
    remaining_amt   NUMERIC(12,2),
    originals_status VARCHAR(20),
    originals_sent  DATE,
    telex_released  BOOLEAN DEFAULT false
);
```

## Boundaries

### Always Do
- Run tests before marking task complete
- Use whitelist validation for dynamic SQL column names
- Write structured logs (log::info!/error!) for all DB operations
- Regenerate AppData.qml after every mutation
- Escape JSON properly for QML single-quoted strings (escape_json function)
- Build and verify offscreen before testing on display

### Ask First
- Adding new dependencies to Cargo.toml
- Changing database schema after initial creation
- Modifying the workflow step count/order
- Adding role authentication or user management

### Never Do
- Commit config.toml with real passwords
- Use sqlx::query_as!() macro (breaks on NUMERIC)
- Call block_on inside #[tokio::test]
- Interpolate user input directly into SQL column names
- Merge the 4 workflow steps into a single form
- Drop the sequential workflow constraint

## Open Questions

- None — all resolved during planning phase.
