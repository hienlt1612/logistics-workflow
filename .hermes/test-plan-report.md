# Testing Plan & Coverage Report — Logistics Workflow v0.2.0

Date: 1 July 2026 | Commit: 5b8b096

---

## 1. Existing Test Coverage (81 tests)

### Backend — Rust (24 tests + 1 ignored, 6 files)

| File | Tests | What's Covered |
|------|-------|---------------|
| `tests/db_tests.rs` | 6 | list_shipments, get_shipment, create_shipment, set_checklist_bool, invalid_field, advance_status |
| `tests/auth_tests.rs` | 3 | authenticate_user (valid, invalid, nonexistent) |
| `tests/update_tests.rs` | 5 | update_shipment_fields, invalid_field, BigDecimal, dates, status |
| `tests/pagination_tests.rs` | 3 | list_shipments_paginated (page1, page2, out-of-range) |
| `tests/batch_tests.rs` | 2 | batch_advance_status, invalid status |
| `tests/delete_tests.rs` | 2 | delete existing, nonexistent |
| `src/config.rs` | 3 | `test_db_url_format`, `test_env_var_overrides`, `test_api_token_from_env` (unit) |
| `src/db/schema.rs` | 1 | `test_schema_creation` (schema check) |

### Frontend — Vue.js (56 tests, 8 files)

| File | What's Covered |
|------|---------------|
| `smoke.test.ts` | App mounts without error |
| `api-client.test.ts` | authHeader, getRole, getToken, X-User-Role |
| `auth-store.test.ts` | Login state, localStorage persistence |
| `dashboard-store.test.ts` | Stats loading, polling start/stop, secondsAgo |
| `format.test.ts` | fmtDateDisplay, fmtDateISO, fmtCurrency |
| `header.test.ts` | Nav links, logout, export button states |
| `sidebar.test.ts` | Render, search filter, admin delete, pagination |
| `step-forms.test.ts` | Step1/2/3 validation, number .trim() safety |

---

## 2. Function Coverage Map

### Backend — Every `pub fn` / `pub async fn`

| Function | Location | Existing Test | Status |
|----------|----------|--------------|--------|
| `init_pool` | db/mod.rs:11 | indirect (all tests use it) | ✅ |
| `pool` | db/mod.rs:21 | indirect | ✅ |
| `ensure_schema` | db/schema.rs:3 | `test_schema_creation` | ✅ |
| `seed_if_empty` | db/seed.rs:4 | none | ⚠️ |
| `list_shipments` | queries.rs:112 | `test_list_shipments_empty` | ✅ |
| `list_shipments_paginated` | queries.rs:117 | 3 tests (page1/2/out) | ✅ |
| `get_shipment` | queries.rs:149 | `test_get_shipment` | ✅ |
| `next_shipment_ref` | queries.rs:154 | `test_create_and_verify` | ✅ |
| `create_shipment` | queries.rs:164 | `test_create_and_verify` | ✅ |
| `update_shipment_fields` | queries.rs:189 | 5 tests (fields/BigDecimal/dates/status) | ✅ |
| `advance_status` | queries.rs:247 | `test_advance_status` | ✅ |
| `batch_advance_status` | queries.rs:257 | 2 tests | ✅ |
| `delete_shipment` | queries.rs:279 | 2 tests | ✅ |
| `authenticate_user` | queries.rs:288 | 3 tests | ✅ |
| `set_checklist_bool` | queries.rs:296 | `test_set_checklist_bool` | ✅ |
| **`list_shipping_calls`** | **queries.rs:310** | **none** | **❌ NEW** |
| **`get_shipping_call`** | **queries.rs:315** | **none** | **❌ NEW** |
| **`next_call_ref`** | **queries.rs:320** | **none** | **❌ NEW** |
| **`create_shipping_call`** | **queries.rs:328** | **none** | **❌ NEW** |
| **`list_call_warehouses`** | **queries.rs:342** | **none** | **❌ NEW** |
| **`create_call_warehouse`** | **queries.rs:347** | **none** | **❌ NEW** |
| **`list_containers`** | **queries.rs:356** | **none** | **❌ NEW** |
| **`create_container`** | **queries.rs:361** | **none** | **❌ NEW** |
| `set_api_token` | bridge.rs:10 | none | ⚠️ |
| `regenerate_appdata` | bridge.rs:58 | none | ⚠️ QML-gated |
| `create_shipment` (bridge) | bridge.rs:130 | none | ⚠️ QML-gated |
| `update_shipment` (bridge) | bridge.rs:147 | none | ⚠️ QML-gated |
| `toggle_checklist` (bridge) | bridge.rs:164 | none | ⚠️ QML-gated |
| `export_workbook1_async` | bridge.rs:178 | none | ⚠️ |
| `start_command_server` | bridge.rs:317 | none (manual) | ⚠️ |
| `Config::load` | config.rs:19 | 2 unit tests | ✅ |
| `Config::db_url` | config.rs:52 | 1 unit test | ✅ |

### Frontend — Stores & API

| Module | Existing Test | Status |
|--------|-------------|--------|
| `api/client.ts` (shipments) | `api-client.test.ts` | ✅ |
| `api/client.ts` (shipping calls) | **none** | **❌ NEW** |
| `stores/auth.ts` | `auth-store.test.ts` | ✅ |
| `stores/dashboard.ts` | `dashboard-store.test.ts` | ✅ |
| `stores/shipments.ts` | `sidebar.test.ts`, `step-forms.test.ts` | ✅ |
| `stores/shipping-calls.ts` | **none** | **❌ NEW** |
| `utils/format.ts` | `format.test.ts` | ✅ |

### Frontend — Views

| View | Existing Test | Status |
|------|-------------|--------|
| `DashboardView.vue` | via sidebar/dashboard stores | ✅ |
| `WorkflowView.vue` | `step-forms.test.ts` | ✅ |
| `ShipmentDetailView.vue` | none | ⚠️ |
| `LoginView.vue` | none | ⚠️ |
| `ExportView.vue` | none | ⚠️ |
| **`ShippingCallList.vue`** | **none** | **❌ NEW** |
| **`ShippingCallDetail.vue`** | **none** | **❌ NEW** |
| **`ShippingCallCreate.vue`** | **none** | **❌ NEW** |

---

## 3. Test Plan — What to Build

### Phase 1: Backend (New Functions) — Priority HIGH

**File: `tests/call_tests.rs`**

| Test Name | Function Tested | Acceptance Criteria |
|-----------|----------------|---------------------|
| `test_create_shipping_call` | `create_shipping_call` | Create call with buyer/incoterms → returns ShippingCall with call_ref |
| `test_get_shipping_call` | `get_shipping_call` | Get by id → returns Some. Get nonexistent → None |
| `test_list_shipping_calls` | `list_shipping_calls` | List all calls → returns Vec (may be empty) |
| `test_next_call_ref_format` | `next_call_ref` | Format is CALL-YYYY-NNN |
| `test_create_call_warehouse` | `create_call_warehouse` | Create warehouse for call → returns CallWarehouse with planned_containers |
| `test_list_call_warehouses` | `list_call_warehouses` | List warehouses by call_id → returns Vec |

**File: `tests/container_tests.rs`**

| Test Name | Function Tested | Acceptance Criteria |
|-----------|----------------|---------------------|
| `test_create_container` | `create_container` | Create container for shipment → returns Container with container_number |
| `test_list_containers` | `list_containers` | List containers by shipment_id → returns Vec |
| `test_list_containers_empty` | `list_containers` | Empty list for shipment with no containers |

**File: `tests/bridge_tests.rs`** (NEW — HTTP-level)

| Test Name | Endpoint | Acceptance Criteria |
|-----------|----------|---------------------|
| `test_get_shipping_calls_empty` | GET /api/shipping-calls | Returns 200, body is [] |
| `test_create_shipping_call_http` | POST /api/shipping-calls | Returns 201, call_ref present |
| `test_create_call_with_warehouses` | POST /api/shipping-calls | Call created + warehouses in one request |
| `test_get_call_by_id` | GET /api/shipping-calls/:id | Returns 200 with call data |
| `test_export_all` | GET /api/export/all | Returns 200 with base64 content |

### Phase 2: Frontend (New Views/Stores) — Priority MEDIUM

**File: `web/src/__tests__/shipping-calls.test.ts`**

| Test Name | What's Tested | Acceptance Criteria |
|-----------|--------------|---------------------|
| `ships call list renders` | ShippingCallList.vue | Renders card grid, shows "+ New Shipping Call" button |
| `ships call create form validates` | ShippingCallCreate.vue | Empty buyer → shows validation error |
| `ships call detail shows warehouses` | ShippingCallDetail.vue | Renders call header, warehouse progress bars |
| `ships call store loads calls` | stores/shipping-calls.ts | `loadAll()` fetches from API, populates `calls` ref |

---

## 4. Priority Matrix

| Priority | Module | Tests Needed | Rationale |
|----------|--------|-------------|-----------|
| **P0** | `queries.rs` (shipping calls) | 6 tests | Core data layer — everything depends on this |
| **P0** | `queries.rs` (containers) | 3 tests | New entity, no existing coverage |
| **P1** | `bridge.rs` (call endpoints) | 5 tests | HTTP layer — API contract must not break |
| **P1** | `stores/shipping-calls.ts` | 1 test | Store is the frontend data backbone |
| **P2** | `ShippingCallList.vue` | 1 test | Smoke test — renders without crash |
| **P2** | `ShippingCallCreate.vue` | 1 test | Form validation |
| **P2** | `ShippingCallDetail.vue` | 1 test | Renders with mock data |
| **P3** | `bridge.rs` (export) | 1 test | Existing un-tested handler |
| **P3** | `db/seed.rs` | 1 test | Seed data integrity |

---

## 5. What NOT to Test (ponytail)

| Skip | Reason |
|------|--------|
| QML bridge functions (`create_shipment`, `regenerate_appdata`, etc.) | QML-gated behind `gui` feature. Web-only default. Delete these first (ponytail audit already flagged). |
| `start_command_server` unit test | Requires server lifecycle. Integration tests cover endpoints. |
| `set_api_token` / `check_auth` unit | Covered by auth_tests integration. |
| `handle_login` / `handle_dashboard` etc. | Covered indirectly by bridge integration tests. |
| Multi-container shipment test | YAGNI — one container per shipment is current schema. |

---

## 6. Verification Commands

```bash
# Run all existing + new tests
cargo test                          # 24 → ~39 (after new tests)

# Run only shipping call tests
cargo test call                     # new tests only
cargo test container                # new tests only

# Frontend
cd web && npm run test:unit         # 56 → ~60 (after new tests)

# Full verification
cargo test && cd web && npm run test:unit
```

---

## 7. Summary

| Layer | Existing | New Needed | After Plan |
|-------|----------|------------|------------|
| Rust tests | 24 + 1 ignored | +9 (6 calls + 3 containers) + 5 (bridge) | 39 |
| Frontend tests | 56 | +4 (1 store + 3 views) | 60 |
| **Total** | **81** | **+18** | **99** |

**Untested new functions:** 8 query functions + 6 HTTP handlers + 1 store + 3 views = 18 gap items.  
**Pre-existing gaps:** seed, export, QML bridge (QML is dead code).  
**Skipped:** QML bridge tests (delete the code instead), server lifecycle unit tests (integration covers), multi-container logic (YAGNI).
