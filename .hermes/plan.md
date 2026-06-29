# Implementation Plan: Logistics Workflow Desktop

## Overview

Rebuild logistics desktop app from scratch following workbook1.xlsx 30-field business process. Single-window QML UI with 4-step sequential workflow (Create → Draft → Customs → Checklist → Telex Release). Rust backend with sqlx + PostgreSQL.

## Architecture Decision: File-Based Data Bridge

cxx-qt 0.6 lacks `rootContext()` for Rust→QML data binding. Use proven pattern:
1. Rust queries DB → serializes to JSON
2. Writes `qml/AppData.qml` singleton with `property string allShipmentsJson`
3. Writes `qml/qmldir` for module registration
4. QML imports local dir, parses with `JSON.parse()`
5. After mutations, regenerate AppData.qml and reload

## Phase 1: Foundation

### Task 1.1: Project scaffold

**Description:** Create Cargo.toml, build.rs, qml.qrc, directory structure, copy workbook1.xlsx.

**Acceptance criteria:**
- [ ] Cargo.toml with cxx-qt 0.6, cxx-qt-lib 0.6, sqlx 0.8, tokio, rust_xlsxwriter 0.80, serde, serde_json, chrono, log, env_logger, anyhow
- [ ] build.rs with CxxQtBuilder + QmlModule
- [ ] resources/qml.qrc referencing all QML files
- [ ] Directory structure created (src/, src/db/, qml/, tests/)
- [ ] workbook1.xlsx symlinked/copied

**Verification:** `cargo build` succeeds (empty main.rs, no actual code yet)

**Files:** Cargo.toml, build.rs, resources/qml.qrc

**Estimate:** S (1-2 files + dirs)

### Task 1.2: Database schema

**Description:** Create shipments table with all 30 workbook1 fields plus metadata. Supporting tables for customers and carriers.

**Acceptance criteria:**
- [ ] `shipments` table with 30 business columns + id, shipment_ref, status, created_at, updated_at
- [ ] `customers` table (id, name)
- [ ] `carriers` table (id, name)
- [ ] All columns match workbook1 field names exactly
- [ ] Booleans have DEFAULT false
- [ ] shipment_ref has UNIQUE constraint

**Verification:** `SELECT column_name, data_type FROM information_schema.columns WHERE table_name='shipments'` returns 34 columns

**Files:** src/db/schema.rs

**Estimate:** S (1 file)

### Task 1.3: Config loader

**Description:** TOML config file reader for database connection parameters. Template with example values.

**Acceptance criteria:**
- [ ] config.toml loaded from current directory
- [ ] Fallback to ~/.config/logistics-workflow/config.toml
- [ ] Fields: host, port, db_name, user, password
- [ ] db_url() method returns connection string
- [ ] config.toml.example with placeholder values
- [ ] config.toml in .gitignore

**Verification:** Unit test: load example config, verify db_url() format

**Files:** src/config.rs, config.toml.example

**Estimate:** S (1 file)

### Task 1.4: Database pool init + connection

**Description:** Initialize sqlx PgPool, store in global OnceLock, verify connectivity.

**Acceptance criteria:**
- [ ] `db::init_pool(url) -> Result<()>` creates pool, stores in OnceLock
- [ ] `db::pool() -> &PgPool` returns reference
- [ ] Pool validates on init (sqlx::pool::PoolOptions with test_before_acquire)
- [ ] Connection test succeeds against local PostgreSQL

**Verification:** Unit test: init test pool, run `SELECT 1`, verify result

**Files:** src/db/mod.rs

**Estimate:** S (1 file)

### Checkpoint 1: Foundation

- [ ] `cargo build` compiles all 4 files
- [ ] Database schema exists in logistics_workflow
- [ ] Config loads and connects to DB
- [ ] `cargo test` passes connection test

---

## Phase 2: Core UI

### Task 2.1: MainWindow.qml — Layout

**Description:** Root QML window with horizontal split: sidebar (left, 250px) + workflow canvas (right, fills rest).

**Acceptance criteria:**
- [ ] Window 1280x800, title "Logistics Workflow"
- [ ] Left panel: 250px wide, dark background (#1A1A2E)
- [ ] Right panel: fills remaining, light background (#F5F5F5)
- [ ] No actual content — just placeholder Rectangles
- [ ] AppData singleton loads without errors

**Verification:** Offscreen: `QT_QPA_PLATFORM=offscreen cargo run` — QML loads, no errors

**Files:** qml/MainWindow.qml, src/main.rs (minimal QGuiApplication + QQmlApplicationEngine)

**Estimate:** M (main.rs + MainWindow.qml + AppData generation)

### Task 2.2: ShipmentSidebar.qml

**Description:** Left panel showing shipment list with search bar, new shipment button.

**Acceptance criteria:**
- [ ] Scrollable ListView of shipments (shows shipment_ref + status + buyer_name)
- [ ] Search/filter TextField at top
- [ ] "New Shipment" Button at top
- [ ] Clicking shipment highlights it and emits selected signal
- [ ] Shows empty state message when no shipments: "No shipments yet"
- [ ] Color-coded status badges (DRAFT=gray, DOCUMENTS=blue, CUSTOMS=orange, CHECKLIST=green, TELEX=purple)

**Verification:** Offscreen: QML loads, list renders (empty or with seed data)

**Files:** qml/ShipmentSidebar.qml

**Estimate:** M (1 QML file + integration with MainWindow)

### Task 2.3: WorkflowProgress.qml

**Description:** 4-step horizontal progress indicator showing current phase.

**Acceptance criteria:**
- [ ] 4 circles connected by lines: CREATE → DRAFT → CUSTOMS → CHECKLIST (→ TELEX)
- [ ] Completed steps: filled green circle with checkmark
- [ ] Current step: filled colored circle (role color)
- [ ] Future steps: hollow gray circle
- [ ] Step labels underneath each circle
- [ ] Responsive to selected shipment's status

**Verification:** Visual: renders in offscreen test, no QML errors

**Files:** qml/WorkflowProgress.qml

**Estimate:** S (1 QML file)

### Task 2.4: Theme.qml — Design System

**Description:** Centralized colors, fonts, spacing constants.

**Acceptance criteria:**
- [ ] Role colors: ADMIN=#E74C3C (red), MANAGER=#3498DB (blue), ACCT=#F39C12 (orange), LOGISTICS=#9B59B6 (purple)
- [ ] Status colors: DRAFT=#95A5A6, DOCUMENTS=#3498DB, CUSTOMS=#F39C12, CHECKLIST=#27AE60, TELEX=#9B59B6
- [ ] Font sizes: h1=24, h2=18, body=14, small=12
- [ ] Spacing: xs=4, sm=8, md=16, lg=24, xl=32
- [ ] Border radius: sm=4, md=8

**Verification:** No QML errors, importable by all other QML files

**Files:** qml/Theme.qml

**Estimate:** S (1 file)

### Checkpoint 2: Core UI

- [ ] `cargo run` shows window with sidebar + workflow canvas
- [ ] Sidebar shows empty state or seed shipments
- [ ] Progress bar renders all 4 steps
- [ ] Offscreen: all QML files load without errors

---

## Phase 3: Step 1 — CREATE (ADMIN)

### Task 3.1: Step1Create.qml — ADMIN Form

**Description:** 9-field form for Phase 1 shipment creation. Red header with "CREATE SHIPMENT" title. Fields match workbook1 columns 0-8.

**Acceptance criteria:**
- [ ] Header: red accent bar + "STEP 1: CREATE SHIPMENT" + "ADMIN" badge
- [ ] Fields: SC/PO ID (text), SC/PO Date (date picker), Made By (text), Buyer (text), Booking# (text), Shipping Line (text), Port of Loading (text), Warehouse (text), Loading Plan (textarea)
- [ ] Required fields marked with asterisk: SC/PO ID, Buyer, Booking#, Shipping Line, Port of Loading
- [ ] Inline validation (red border on empty required fields)
- [ ] "Save & Continue" button → saves shipment, advances to DRAFT status
- [ ] Shows existing data when editing an already-created shipment

**Verification:** Form renders in offscreen, no QML errors. Fields accept input.

**Files:** qml/Step1Create.qml

**Estimate:** L (complex form with validation, bindings)

### Task 3.2: Rust: create_shipment() + list_shipments()

**Description:** Database operations for shipment CRUD — Phase 1 fields.

**Acceptance criteria:**
- [ ] `create_shipment(form) -> Result<Shipment>` — inserts new row, auto-generates shipment_ref (SHP-YYYY-NNN)
- [ ] `get_shipment(id) -> Result<Shipment>` — single shipment by id
- [ ] `list_shipments() -> Result<Vec<Shipment>>` — all shipments, newest first
- [ ] `update_shipment_phase1(id, form) -> Result<()>` — updates Phase 1 fields
- [ ] Shipment struct with all 30 fields + metadata, derives Serialize + FromRow

**Verification:** Unit tests: create → get → verify fields, list returns created shipment

**Files:** src/db/queries.rs, src/db/seed.rs

**Estimate:** M (queries + struct + seed data)

### Task 3.3: Bridge: AppData generation + Step 1 wiring

**Description:** Rust bridge that generates AppData.qml with all shipment JSON. Wire Step 1 create flow end-to-end.

**Acceptance criteria:**
- [ ] `regenerate_appdata()` — queries all shipments, writes AppData.qml with escape_json
- [ ] `create_shipment_json(json_str) -> Result<String>` — parses JSON, calls create_shipment, regenerates AppData, returns new shipment_ref
- [ ] QML calls bridge via timer/signal pattern (since no direct Rust→QML calls in cxx-qt 0.6)
- [ ] After create, sidebar refreshes with new shipment

**Verification:** Create shipment via Rust test → verify AppData.qml updated → QML shows new shipment

**Files:** src/bridge.rs

**Estimate:** M (bridge + escape_json + wiring)

### Checkpoint 3: Step 1 Complete

- [ ] Can create new shipment from UI
- [ ] Shipment appears in sidebar with DRAFT status
- [ ] Clicking shipment loads Step 1 form with existing data
- [ ] Progress bar shows Step 1 as current

---

## Phase 4: Steps 2-3 — DRAFT + CUSTOMS

### Task 4.1: Step2Draft.qml — MANAGER Form

**Description:** 8-field form for Phase 2: draft documentation. Blue header.

**Acceptance criteria:**
- [ ] Header: blue accent + "STEP 2: DRAFT DOCUMENTATION" + "MANAGER" badge
- [ ] Fields: Shipper (text), Consignee (text), ETD (date), Invoice# (text), Invoice Date (date), Total Value (number), Drafts Date (date), Bill of Lading# (text)
- [ ] Required: Shipper, Consignee, ETD, Invoice#, Total Value
- [ ] "Save & Continue" → updates shipment, advances to CUSTOMS_CLEARED status
- [ ] Only visible when status >= DOCUMENTS_READY

**Verification:** QML loads, fields render, validation works

**Files:** qml/Step2Draft.qml

**Estimate:** M (1 QML file)

### Task 4.2: Step3Customs.qml — ACCOUNTING Form

**Description:** 3-field form for Phase 3: customs clearance. Orange header.

**Acceptance criteria:**
- [ ] Header: orange accent + "STEP 3: CUSTOMS CLEARANCE" + "ACCOUNTING" badge
- [ ] Fields: Customs Date (date), Customs Number (text), Customs Status (dropdown: red/yellow/green)
- [ ] Required: Customs Date, Customs Number, Customs Status
- [ ] "Save & Continue" → updates shipment, advances to CHECKLIST_IN_PROGRESS
- [ ] Only visible when status >= CUSTOMS_CLEARED

**Verification:** QML loads, dropdown works, status progression correct

**Files:** qml/Step3Customs.qml

**Estimate:** S (1 QML file)

### Task 4.3: Rust: batch_update_shipment()

**Description:** Update multiple Phase 2 or Phase 3 fields in one query. Use whitelist column validation.

**Acceptance criteria:**
- [ ] `update_shipment_fields(id, fields: HashMap<String, Value>)` — dynamic column updates with whitelist
- [ ] `advance_status(id, new_status)` — validates transition is forward-only
- [ ] Whitelist: all 30 column names validated before SQL interpolation
- [ ] Timestamps auto-updated on any mutation

**Verification:** Unit test: update 3 fields → get shipment → verify only those fields changed

**Files:** src/db/queries.rs

**Estimate:** S (1 function + tests)

### Task 4.4: Status progression + step visibility

**Description:** Wire the state machine so each step only appears when its prerequisite status is met.

**Acceptance criteria:**
- [ ] Step 1 (Create): always visible when shipment selected
- [ ] Step 2 (Draft): visible when status >= DOCUMENTS_READY
- [ ] Step 3 (Customs): visible when status >= CUSTOMS_CLEARED
- [ ] Step 4 (Checklist): visible when status >= CHECKLIST_IN_PROGRESS
- [ ] Steps auto-expand/collapse based on status
- [ ] Completed steps show read-only summary

**Verification:** Create shipment → verify only Step 1 visible. Fill Step 1 → Step 2 appears. Fill Step 2 → Step 3 appears.

**Files:** qml/MainWindow.qml (step visibility logic)

**Estimate:** M (QML logic + status wiring)

### Checkpoint 4: Steps 2-3 Complete

- [ ] Full 3-step form flow works end-to-end
- [ ] Status advances: DRAFT → DOCUMENTS → CUSTOMS
- [ ] Progress bar updates correctly
- [ ] Cannot skip steps

---

## Phase 5: Step 4 — CHECKLIST + TELEX

### Task 5.1: Step4Checklist.qml — Document Checklist

**Description:** Multi-section checklist with boolean toggles, payment tracking, originals management, and telex release button. Multi-color header.

**Acceptance criteria:**
- [ ] Header: "STEP 4: DOCUMENT CHECKLIST" with role color badges
- [ ] Section 1 - DOCUMENTS (blue): BL Received ✓, Docs Confirmed ✓
- [ ] Section 2 - CHARGES (orange): Charges/THC Paid ✓
- [ ] Section 3 - CERTIFICATES (purple): CO Received ✓, Phyto Received ✓
- [ ] Section 4 - PAYMENT (orange): Prepayment Date, Prepayment Amount, Remaining Amount
- [ ] Section 5 - ORIGINALS (blue): Originals Status (text), Date Sent (date)
- [ ] Section 6 - TELEX RELEASE (blue): Telex Released ✓ (final action)
- [ ] Each boolean renders as styled checkbox with label
- [ ] "Save Changes" button for payment/originals sections
- [ ] Telex release has confirmation dialog: "Release telex? This is final."
- [ ] Only visible when status >= CHECKLIST_IN_PROGRESS

**Verification:** QML renders all 6 sections. Checkboxes toggle. Telex confirm dialog works.

**Files:** qml/Step4Checklist.qml

**Estimate:** L (complex multi-section form, 11 fields)

### Task 5.2: Rust: set_checklist_bool() with whitelist

**Description:** Toggle boolean checklist fields with SQL injection-safe whitelist.

**Acceptance criteria:**
- [ ] `set_checklist_bool(id, field_name, value)` — whitelist validates field_name
- [ ] Whitelist: bl_received, charges_paid, co_received, phyto_received, docs_confirmed, telex_released
- [ ] Returns error for invalid field_name
- [ ] Setting telex_released=true auto-advances status to TELEX_RELEASED

**Verification:** Unit test: toggle bl_received true → get shipment → verify true. Pass invalid field → verify error.

**Files:** src/db/queries.rs

**Estimate:** S (1 function)

### Task 5.3: Rust: update_checklist_fields()

**Description:** Update payment and originals fields (date/number/text) with whitelist.

**Acceptance criteria:**
- [ ] `update_checklist_fields(id, fields)` — updates prepayment_date, prepayment_amt, remaining_amt, originals_status, originals_sent
- [ ] Whitelist validates all field names
- [ ] Numeric validation for amount fields

**Verification:** Unit test: update payment → verify. Pass invalid field → verify error.

**Files:** src/db/queries.rs

**Estimate:** S (1 function)

### Checkpoint 5: Full Workflow Complete

- [ ] All 4 steps work end-to-end
- [ ] All 30 fields can be created and updated
- [ ] Telex release completes shipment (terminal state)
- [ ] Status progression: DRAFT → DOCUMENTS → CUSTOMS → CHECKLIST → TELEX_RELEASED
- [ ] All checklist booleans toggle and persist

---

## Phase 6: Export + Polish

### Task 6.1: Excel Export (workbook1 format)

**Description:** Export all shipments to 30-column Excel matching workbook1 layout. Role-grouped color-coded headers.

**Acceptance criteria:**
- [ ] Sheet name: "Workbook1 Export"
- [ ] Merged header rows showing role groups (ADMIN | MANAGER | ACCOUNTING | LOGISTICS)
- [ ] Second header row: individual field names
- [ ] Data rows: one per shipment
- [ ] Boolean fields shown as "✓" or ""
- [ ] Color-coded column groups matching role colors
- [ ] Auto-column-width
- [ ] Export button in toolbar
- [ ] Saves to ~/Documents/ with timestamp filename

**Verification:** Export 5 seed shipments → open in LibreOffice → verify layout matches workbook1

**Files:** src/bridge.rs (export function)

**Estimate:** M (reuse pattern from logistics-desk, adapt to 30 columns)

### Task 6.2: Search/filter in sidebar

**Description:** Filter shipments by status, search by shipment_ref or buyer_name.

**Acceptance criteria:**
- [ ] Text search: filters by shipment_ref or buyer_name (case-insensitive)
- [ ] Status filter: dropdown with All/DRAFT/DOCUMENTS/CUSTOMS/CHECKLIST/TELEX
- [ ] Filters combine (AND logic)
- [ ] Empty state: "No shipments match filters"
- [ ] Filter bar above list

**Verification:** Create 3 shipments with different statuses → filter by status → verify only matching shown

**Files:** qml/ShipmentSidebar.qml

**Estimate:** S (filter logic in existing QML)

### Task 6.3: Error handling + logging

**Description:** Structured logging for all operations. User-facing error messages for common failures.

**Acceptance criteria:**
- [ ] log::info! for all DB operations (create, update, export)
- [ ] log::error! for all failures with context
- [ ] QML toast/notification for errors (bottom-right banner)
- [ ] DB connection errors show meaningful message
- [ ] Form validation errors shown inline

**Verification:** Simulate DB disconnect → verify error toast appears

**Files:** src/main.rs (logging init), qml/MainWindow.qml (toast component)

**Estimate:** S (logging + simple toast)

### Task 6.4: Desktop launcher

**Description:** run.sh script + .desktop file for application menu.

**Acceptance criteria:**
- [ ] run.sh: cd to project dir, set RUST_LOG=info, exec cargo run
- [ ] logistics-workflow.desktop: Name, Exec, Path, Icon, Categories=Office
- [ ] Placed at ~/.local/share/applications/
- [ ] Works from application menu

**Verification:** Click .desktop entry → app launches

**Files:** run.sh, logistics-workflow.desktop

**Estimate:** XS (2 small files)

### Checkpoint 6: Polish Complete

- [ ] Export produces correct workbook1-format Excel
- [ ] Search/filter works in sidebar
- [ ] Errors are logged and displayed
- [ ] Desktop entry launches app

---

## Phase 7: Testing + Debug

### Task 7.1: DB query unit tests

**Description:** Test all query functions in isolation.

**Acceptance criteria:**
- [ ] test_create_shipment: creates, retrieves, verifies all fields
- [ ] test_list_shipments: seeds 3, lists 3
- [ ] test_update_shipment: updates fields, verifies
- [ ] test_set_checklist_bool: toggles, verifies
- [ ] test_invalid_field_rejected: returns error for bad field name
- [ ] test_advance_status: valid and invalid transitions
- [ ] test_seed_data: 5 workbook1 shipments seeded correctly

**Verification:** `cargo test db_tests` — all pass

**Files:** tests/db_tests.rs

**Estimate:** M (7+ tests)

### Task 7.2: Integration tests

**Description:** Full CRUD cycle + status progression + export.

**Acceptance criteria:**
- [ ] test_full_workflow: create → draft → customs → checklist → telex → verify status
- [ ] test_appdata_regeneration: create shipment → regenerate → verify JSON contains it
- [ ] test_excel_export: export → verify file exists, has correct sheet name
- [ ] test_config_loader: load example config → verify values

**Verification:** `cargo test integration` — all pass

**Files:** tests/integration_tests.rs

**Estimate:** M (4+ tests)

### Task 7.3: QML offscreen tests

**Description:** Verify all QML files load without errors in offscreen mode.

**Acceptance criteria:**
- [ ] `QT_QPA_PLATFORM=offscreen cargo run` exits clean (no QML errors)
- [ ] All module imports resolve
- [ ] No "Property value set multiple times" errors
- [ ] No undefined property warnings

**Verification:** Scripted offscreen run with QT_LOGGING_RULES, grep for errors/warnings

**Estimate:** S (verification script)

### Task 7.4: Edge cases + robustness

**Description:** Test boundary conditions and error recovery.

**Acceptance criteria:**
- [ ] Empty database: app shows empty state, no crash
- [ ] Invalid config.toml: meaningful error message, exits clean
- [ ] DB connection refused: retry logic or clear error
- [ ] Missing required fields: validation prevents save
- [ ] Duplicate shipment_ref: handled gracefully (auto-increment prevents)
- [ ] Very long text in loading_plan: stored correctly (TEXT type handles)
- [ ] Special characters in text fields: escaped correctly in JSON

**Verification:** Manual testing of each edge case

**Estimate:** S (edge case tests)

### Task 7.5: `cargo clippy` pass

**Description:** Zero clippy warnings on release build.

**Acceptance criteria:**
- [ ] `cargo clippy -- -D warnings` exits 0
- [ ] All code follows Rust idioms

**Verification:** Run clippy, fix any warnings

**Estimate:** S (lint fix)

### Checkpoint 7: All Tests Green

- [ ] `cargo test` — all tests pass
- [ ] `cargo clippy -- -D warnings` — zero warnings
- [ ] `cargo build --release` — builds successfully
- [ ] Offscreen QML loads without errors
- [ ] Full end-to-end workflow verified manually

---

## Risk Matrix

| Risk | Impact | Mitigation |
|---|---|---|
| cxx-qt 0.6 API breakage | High | Use file-based AppData.qml (no QObject bridge). Proven pattern from logistics-desk. |
| QML form complexity | Medium | Build one step at a time, verify each independently before wiring together |
| DB connection issues | Low | Config validation at startup, clear error messages, test with local PG |
| Excel export formatting | Medium | Reuse proven rust_xlsxwriter pattern, verify against workbook1 reference |
| Status state machine bugs | Medium | Unit test every transition, prevent backward moves |
| Inline linter false positives | Low | Ignore Rust 2015 edition warnings, verify with cargo build instead |

## Total Estimates

- **XS tasks:** 1 (launcher)
- **S tasks:** 10 (schema, config, pool, progress, theme, customs form, batch update, bool toggle, checklist fields, search, error handling, QML tests, edge cases, clippy)
- **M tasks:** 8 (main window, sidebar, step2 form, step1 wiring, status progression, excel export, db tests, integration tests)
- **L tasks:** 2 (step1 form, step4 checklist)

**Total:** 21 tasks across 7 phases
