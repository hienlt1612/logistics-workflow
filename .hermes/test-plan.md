# Logistics Workflow v0.2.0 — Comprehensive Test Plan

Generated: 2026-06-30 | Source: agent-skills (planning-and-task-breakdown)
=======================================================================

## Inventory Summary

| Layer | Modules | Public Functions | Components |
|-------|---------|-----------------|------------|
| Rust DB Queries | queries.rs, schema.rs, seed.rs | 12 | — |
| Rust Bridge (API) | bridge.rs | 7 | — |
| Rust Config | config.rs | 2 | — |
| Frontend Stores | auth, dashboard, shipments | 16 | — |
| Frontend API Client | client.ts | 9 | — |
| Frontend Views | 5 views | — | Dashboard, Workflow, Detail, Export, Login |
| Frontend Components | 12 components | — | Steps 1-4, Layout, Toast, DateInput, etc. |
| **TOTAL** | | **46 functions** | **17 components** |

---

## MODULE 1: Database Layer (`src/db/`)

### 1.1 `ensure_schema(pool) → Result<()>`
**Source:** `src/db/schema.rs:3`
- [ ] Test: creates `shipments` table with all 37 columns
- [ ] Test: creates `users` table with 4 columns
- [ ] Test: idempotent — calling twice doesn't fail
- [ ] Test: correct column types (BIGSERIAL, VARCHAR, DATE, NUMERIC, BOOLEAN, TIMESTAMPTZ)

### 1.2 `init_pool(url) → Result<()>`
**Source:** `src/db/mod.rs:11`
- [ ] Test: connects successfully with valid URL
- [ ] Test: returns error with invalid URL
- [ ] Test: pool is reusable across queries

### 1.3 `seed_if_empty(pool) → Result<()>`
**Source:** `src/db/seed.rs:4`
- [ ] Test: inserts 5 seed shipments when table is empty
- [ ] Test: does NOT insert when shipments exist (idempotent)
- [ ] Test: seed data has all required fields filled

### 1.4 `list_shipments(pool) → Result<Vec<Shipment>>`
**Source:** `src/db/queries.rs:70`
- [ ] Test: returns all shipments (unbounded)
- [ ] Test: returns empty vec for empty table
- [ ] Test: all 37 fields populated in returned struct
- [ ] Test: order is consistent (by id)

### 1.5 `list_shipments_paginated(pool, status, page, page_size) → Result<(Vec<Shipment>, i64)>`
**Source:** `src/db/queries.rs:75`
- [ ] Test: returns correct page of results
- [ ] Test: returns correct total count
- [ ] Test: page=1, pageSize=20 returns first 20
- [ ] Test: page beyond total returns empty data
- [ ] Test: status filter works (DRAFT, DOCUMENTS_READY, etc.)
- [ ] Test: NULL status filter returns all

### 1.6 `get_shipment(pool, id) → Result<Option<Shipment>>`
**Source:** `src/db/queries.rs:107`
- [ ] Test: returns correct shipment by id
- [ ] Test: returns None for non-existent id
- [ ] Test: all fields match database

### 1.7 `next_shipment_ref(pool) → Result<String>`
**Source:** `src/db/queries.rs:112`
- [ ] Test: returns SHP-YYYY-NNN format
- [ ] Test: increments for each call
- [ ] Test: works when table is empty

### 1.8 `create_shipment(pool, input) → Result<Shipment>`
**Source:** `src/db/queries.rs:122`
- [ ] Test: creates shipment with all fields
- [ ] Test: auto-generates shipment_ref
- [ ] Test: sets created_at and updated_at
- [ ] Test: validates required fields (NOT NULL)

### 1.9 `update_shipment_fields(pool, id, fields) → Result<Shipment>`
**Source:** `src/db/queries.rs:147`
- [ ] Test: updates specified fields only
- [ ] Test: rejects invalid field names
- [ ] Test: updates `updated_at` timestamp
- [ ] Test: BigDecimal fields accept string input
- [ ] Test: BigDecimal fields accept number input (JSON number)
- [ ] Test: Date fields accept yyyy-MM-dd
- [ ] Test: Boolean fields accept true/false
- [ ] Test: `telex_released: true` auto-sets status to TELEX_RELEASED
- [ ] Test: status field works

### 1.10 `advance_status(pool, id, new_status) → Result<Shipment>`
**Source:** `src/db/queries.rs:205`
- [ ] Test: advances to valid status
- [ ] Test: rejects invalid status values
- [ ] Test: updated_at changes

### 1.11 `batch_advance_status(pool, ids, new_status) → Result<u64>`
**Source:** `src/db/queries.rs:215`
- [ ] Test: updates multiple shipments at once
- [ ] Test: returns correct count of updated rows
- [ ] Test: invalid status returns error
- [ ] Test: empty ids array returns 0

### 1.12 `delete_shipment(pool, id) → Result<bool>`
**Source:** `src/db/queries.rs:237`
- [ ] Test: deletes existing shipment → returns true
- [ ] Test: non-existent id → returns false

### 1.13 `set_checklist_bool(pool, id, field, value) → Result<Shipment>`
**Source:** `src/db/queries.rs:254`
- [ ] Test: toggles bl_received, charges_paid, co_received, phyto_received, docs_confirmed
- [ ] Test: toggles payment_received
- [ ] Test: toggles telex_released (auto-sets status)
- [ ] Test: rejects invalid field names
- [ ] Test: returns updated shipment

### 1.14 `authenticate_user(pool, username, password) → Result<Option<User>>`
**Source:** `src/db/queries.rs:246`
- [ ] Test: returns user for valid credentials
- [ ] Test: returns None for invalid password
- [ ] Test: returns None for non-existent user
- [ ] Test: SQL injection resistant (parameterized query)

---

## MODULE 2: API Layer (`src/bridge.rs`)

### 2.1 `set_api_token(token)`
**Source:** `src/bridge.rs:10`
- [ ] Test: stores token globally
- [ ] Test: None disables auth
- [ ] Test: Some("") disables auth (empty string)
- [ ] Test: Some("secret") enables auth

### 2.2 `check_auth(method, raw) → Result<()>`
**Source:** `src/bridge.rs:21`
- [ ] Test: GET requests always pass
- [ ] Test: POST without token → fails (when auth enabled)
- [ ] Test: POST with valid token → passes
- [ ] Test: POST with invalid token → fails
- [ ] Test: PATCH/PATCH/POST all require token
- [ ] Test: OPTIONS always pass

### 2.3 `POST /api/login` → `handle_login(pool, body)`
**Source:** `src/bridge.rs:483`
- [ ] Test: valid username/password → 200 + user JSON
- [ ] Test: invalid password → 401 UNAUTHORIZED
- [ ] Test: missing fields → 400 BAD_REQUEST
- [ ] Test: returns username, role, token in response

### 2.4 `GET /api/dashboard` → `handle_dashboard(pool)`
**Source:** `src/bridge.rs:510`
- [ ] Test: returns status counts (total, draft, documents, customs, checklist, telex)
- [ ] Test: sum of counts equals total
- [ ] Test: empty DB returns all zeros

### 2.5 `GET /api/shipments` → `handle_list_shipments(pool, status, page, pageSize)`
**Source:** `src/bridge.rs:542`
- [ ] Test: returns paginated response with data + pagination object
- [ ] Test: pagination has page, pageSize, totalItems, totalPages
- [ ] Test: default page=1, pageSize=20
- [ ] Test: status filter works
- [ ] Test: headers include auth check

### 2.6 `GET /api/shipments/:id` → `handle_get_shipment(pool, id)`
**Source:** `src/bridge.rs:561`
- [ ] Test: returns full shipment JSON
- [ ] Test: 404 for non-existent id
- [ ] Test: 400 for invalid id

### 2.7 `POST /api/shipments` → `handle_create_shipment(pool, body)`
**Source:** `src/bridge.rs:572`
- [ ] Test: creates shipment with valid input
- [ ] Test: returns 201 with created shipment
- [ ] Test: 400 for invalid JSON
- [ ] Test: 400 for missing required fields
- [ ] Test: 401 if no auth token
- [ ] Test: auto-generates shipment_ref

### 2.8 `PATCH /api/shipments/:id` → `handle_update_shipment(pool, id, body)`
**Source:** `src/bridge.rs:587`
- [ ] Test: updates fields and returns updated shipment
- [ ] Test: 400 for invalid field names
- [ ] Test: 401 if no auth token
- [ ] Test: 404 for non-existent id

### 2.9 `PATCH /api/shipments/:id/checklist` → `handle_toggle_checklist(pool, id, body)`
**Source:** `src/bridge.rs:609`
- [ ] Test: toggles boolean field
- [ ] Test: auto-sets TELEX_RELEASED when telex_released toggled
- [ ] Test: 400 for invalid field name

### 2.10 `DELETE /api/shipments/:id` → `handle_delete_shipment(pool, id)`
**Source:** `src/bridge.rs:634`
- [ ] Test: deletes shipment → 200 {ok:true}
- [ ] Test: non-existent id → 404
- [ ] Test: 401 if no auth token

### 2.11 `PATCH /api/shipments/batch` → `handle_batch_update(pool, body)`
**Source:** `src/bridge.rs:645`
- [ ] Test: updates multiple shipments
- [ ] Test: returns count of updated
- [ ] Test: 400 for invalid status
- [ ] Test: 401 if no auth token

### 2.12 `GET /api/export/all` → `handle_export_all()`
**Source:** `src/bridge.rs:671`
- [ ] Test: returns JSON with base64-encoded XLSX data
- [ ] Test: base64 decodes to valid ZIP/XLSX
- [ ] Test: contains filename and contentType
- [ ] Test: works with empty DB

### 2.13 `regenerate_appdata()`
**Source:** `src/bridge.rs:45`
- [ ] Test: generates AppData.qml with shipment JSON
- [ ] Test: called after mutations

### 2.14 `start_command_server()`
**Source:** `src/bridge.rs:304`
- [ ] Test: starts HTTP server on configured port
- [ ] Test: accepts connections
- [ ] Test: returns CORS headers
- [ ] Test: handles multiple concurrent requests

---

## MODULE 3: Frontend Stores (`web/src/stores/`)

### 3.1 Auth Store — `useAuthStore`
**Source:** `stores/auth.ts`

| Function | Test |
|----------|------|
| `loadFromStorage()` | Loads persisted user from localStorage; handles missing/corrupt data gracefully |
| `login(user, pass)` | Calls POST /api/login; on success stores user + token; on failure returns error message |
| `logout()` | Clears user state + localStorage |
| `isLoggedIn` computed | Returns true when user is set |
| `role` computed | Returns user role string |

- [ ] Test: successful login sets isLoggedIn
- [ ] Test: failed login returns error message
- [ ] Test: logout clears state
- [ ] Test: loadFromStorage restores session on page refresh

### 3.2 Dashboard Store — `useDashboardStore`
**Source:** `stores/dashboard.ts`

| Function | Test |
|----------|------|
| `load()` | Fetches /api/dashboard; sets stats |
| `startPolling(ms)` | Auto-refreshes every N ms |
| `stopPolling()` | Stops auto-refresh |
| `secondsAgo()` | Returns seconds since last update |

- [ ] Test: load sets stats object
- [ ] Test: polling fires repeatedly
- [ ] Test: secondsAgo increases over time
- [ ] Test: stopPolling stops timer

### 3.3 Shipments Store — `useShipmentsStore`
**Source:** `stores/shipments.ts`

| Function | Test |
|----------|------|
| `loadAll(status?, page?)` | Fetches paginated shipments; sets shipments + pagination state |
| `goToPage(n)` | Changes page and reloads |
| `select(id)` | Sets selected shipment by id |
| `create(data)` | Creates new shipment via API |
| `updateCurrent(fields)` | Updates selected shipment via API |
| `toggleChecklistField(field, value)` | Toggles checklist bool + reloads |
| `remove(id)` | Deletes shipment + reloads |
| `batchAdvance(ids, status)` | Batch advances status + reloads |
| `clearToast()` | Clears last toast message |

- [ ] Test: loadAll populates shipments array
- [ ] Test: loadAll sets pagination state correctly
- [ ] Test: selected computed returns correct shipment
- [ ] Test: updateCurrent updates array element in-place
- [ ] Test: remove removes from array
- [ ] Test: error paths set error state + toast

---

## MODULE 4: Frontend API Client (`web/src/api/client.ts`)

| Function | Test |
|----------|------|
| `fetchDashboard()` | GET /api/dashboard → DashboardStats |
| `fetchShipments(opts)` | GET /api/shipments → PaginatedShipments |
| `fetchShipment(id)` | GET /api/shipments/:id → Shipment |
| `createShipment(data)` | POST /api/shipments → Shipment |
| `updateShipment(id, fields)` | PATCH /api/shipments/:id → Shipment |
| `toggleChecklist(id, field, value)` | PATCH /api/shipments/:id/checklist → Shipment |
| `deleteShipment(id)` | DELETE /api/shipments/:id → void |
| `batchAdvanceStatus(ids, status)` | PATCH /api/shipments/batch → number |
| `downloadExcel()` | GET /api/export/all → triggers Blob download |

- [ ] Test: auth header sent for write methods
- [ ] Test: auth header NOT sent for GET
- [ ] Test: error responses throw ApiClientError
- [ ] Test: base64 decode produces valid Blob

---

## MODULE 5: Frontend Views

### 5.1 LoginView
- [ ] Renders username + password fields
- [ ] Submit calls auth.login()
- [ ] Shows error message on failure
- [ ] Redirects to / on success
- [ ] Loading state disables button
- [ ] No credentials hint displayed

### 5.2 DashboardView
- [ ] Shows 6 stat cards (Total, Draft, Documents, Customs, Checklist, Telex)
- [ ] Auto-refreshes every 30s
- [ ] Shows "Updated X ago" indicator
- [ ] Loading state while fetching
- [ ] Recent shipments table (top 10)

### 5.3 WorkflowView
- [ ] Shows WorkflowProgress steps
- [ ] Shows correct Step component based on status
- [ ] Step1 for DRAFT
- [ ] Step2 for DOCUMENTS_READY
- [ ] Step3 for CUSTOMS_CLEARED
- [ ] Step4 for CHECKLIST_IN_PROGRESS / TELEX_RELEASED
- [ ] Selected shipment from sidebar loads into form

### 5.4 ShipmentDetailView
- [ ] Shows all 37 fields read-only
- [ ] Formats dates as dd/MM/yyyy
- [ ] Formats booleans as ✓/✗
- [ ] Back button returns to workflow

### 5.5 ExportView (if relevant)
- [ ] Export button triggers download

---

## MODULE 6: Frontend Components

### 6.1 Step1Create
- [ ] Renders 9 fields
- [ ] Validates 5 required fields
- [ ] Save & Continue calls API + advances to DOCUMENTS_READY
- [ ] Read-only mode when telex_released

### 6.2 Step2Draft
- [ ] Renders 8 fields
- [ ] Validates 5 required fields
- [ ] total_value_usd accepts number input, saves correctly
- [ ] Date fields save correctly

### 6.3 Step3Customs
- [ ] Renders 3 fields
- [ ] customs_status is optional dropdown
- [ ] Validates customs_date + customs_number

### 6.4 Step4Checklist
- [ ] Toggles all boolean fields
- [ ] Payment section: saves prepayment_date, prepayment_amt, remaining_amt
- [ ] Originals section: saves originals_status, originals_sent, originals_description
- [ ] Payment received checkbox works
- [ ] Telex release modal with confirmation

### 6.5 AppSidebar
- [ ] Lists shipments with status badge
- [ ] Search filters by text
- [ ] Status filter dropdown
- [ ] Pagination controls
- [ ] Batch select + advance
- [ ] New shipment button

### 6.6 Toast
- [ ] Shows success/error messages
- [ ] Auto-dismisses after 3.5s
- [ ] Animated slide-in

### 6.7 AppLayout / AppHeader
- [ ] Responsive layout
- [ ] Mobile sidebar toggle
- [ ] Export button in header

---

## MODULE 7: Configuration

### 7.1 Config::load()
- [ ] Loads from config.toml
- [ ] Fallback to ~/.config/logistics-workflow/config.toml
- [ ] Error if no config found
- [ ] api_token defaults to None if not set

### 7.2 Config::db_url()
- [ ] Generates correct PostgreSQL connection string

---

## MODULE 8: Cross-Cutting Concerns

### 8.1 Auth Flow (End-to-End)
- [ ] Unauthenticated user → redirected to /login
- [ ] Login with valid credentials → redirected to dashboard
- [ ] Dashboard loads data after login
- [ ] Write operations require valid LW_API_TOKEN
- [ ] Logout clears state and redirects to login

### 8.2 CORS
- [ ] All origins allowed (`*`)
- [ ] Authorization header allowed
- [ ] All methods allowed

### 8.3 Error Handling
- [ ] Invalid JSON → 400 with error message
- [ ] DB errors → 500 with error message
- [ ] Network errors → toast notification
- [ ] Auth errors → 401 with clear message

### 8.4 Export
- [ ] Generates valid XLSX with all 37 columns
- [ ] Contains Summary sheet with status counts
- [ ] Download via Blob in browser

---

## Test Execution Matrix

| Layer | Framework | Command | Current | Target |
|-------|-----------|---------|---------|--------|
| Rust Unit | cargo test | `cargo test` | 7 tests | 25+ tests |
| Rust Integration | cargo test | `cargo test --test '*'` | 0 tests | 15+ tests |
| Frontend Unit | Vitest | `npm run test:unit` | 0 tests | 20+ tests |
| Frontend Component | Vitest + VTU | `npm run test:unit` | 0 tests | 12+ tests |
| E2E (optional) | Playwright | `npx playwright test` | 0 tests | 5+ tests |

**Total target: 77+ tests across all layers**
