# Test Report — Logistics Workflow v0.2.0

Generated: 2026-06-30

## Results Summary

| Layer | Files | Tests | Passed | Failed | Ignored |
|-------|-------|-------|--------|--------|---------|
| Rust Unit | 2 | 3 | 3 | 0 | 0 |
| Rust Integration | 7 | 22 | 21 | 0 | 1 |
| Frontend | 8 | 56 | 56 | 0 | 0 |
| **TOTAL** | **17** | **81** | **80** | **0** | **1** |

## Rust Tests (24 passed, 1 ignored)

### Unit Tests (src/)
| File | Tests | Status |
|------|-------|--------|
| src/config.rs | 3 | ✓ 3 passed |
| src/db/schema.rs | 1 | ✓ 1 passed |

- test_db_url_format
- test_env_var_overrides
- test_api_token_from_env
- test_schema_creation

### Integration Tests (tests/)
| File | Tests | Status |
|------|-------|--------|
| tests/db_tests.rs | 6 | ✓ 5 passed, 1 ignored |
| tests/auth_tests.rs | 3 | ✓ 3 passed |
| tests/update_tests.rs | 5 | ✓ 5 passed |
| tests/pagination_tests.rs | 3 | ✓ 3 passed |
| tests/batch_tests.rs | 2 | ✓ 2 passed |
| tests/delete_tests.rs | 2 | ✓ 2 passed |

**Auth (3):** valid login, wrong password, nonexistent user
**Update (5):** field update, invalid field rejection, BigDecimal, dates, status
**Pagination (3):** page 1, page 2, out-of-range
**Batch (2):** advance 2 shipments, invalid status rejection
**Delete (2):** existing shipment, nonexistent
**DB core (6):** list, get, create [ignored], checklist toggle, invalid field, advance status

## Frontend Tests (56 passed)

| File | Tests | Status |
|------|-------|--------|
| format.test.ts | 13 | ✓ 13 passed |
| auth-store.test.ts | 3 | ✓ 3 passed |
| smoke.test.ts | 2 | ✓ 2 passed |
| api-client.test.ts | 6 | ✓ 6 passed |
| dashboard-store.test.ts | 6 | ✓ 6 passed |
| step-forms.test.ts | 10 | ✓ 10 passed |
| sidebar.test.ts | 10 | ✓ 10 passed |
| header.test.ts | 6 | ✓ 6 passed |

**Format (13):** fmtDateDisplay, fmtDateISO, fmtCurrency
**Auth Store (3):** starts empty, loads from storage, logout clears
**Smoke (2):** component render, Pinia store
**API Client (6):** authHeader, getRole, getToken, X-User-Role
**Dashboard Store (6):** polling start/stop, secondsAgo
**Step Forms (10):** Step1/2/3 validation, number .trim() safety
**Sidebar (10):** render, search, filter, admin delete, pagination
**Header (6):** nav links, logout, export states

## Commands

```bash
# Backend
cd ~/logistics-workflow && cargo test

# Frontend
cd ~/logistics-workflow/web && npm run test:unit

# Watch mode
cd ~/logistics-workflow/web && npm run test:watch
```

## Coverage Gap Analysis

| Area | Status |
|------|--------|
| DB queries (CRUD) | ✓ Covered |
| Auth / login | ✓ Covered |
| Update fields | ✓ Covered |
| Pagination | ✓ Covered |
| Batch operations | ✓ Covered |
| Delete | ✓ Covered |
| Config + env vars | ✓ Covered |
| Step form validation | ✓ Covered |
| Sidebar UI | ✓ Covered |
| Header UI | ✓ Covered |
| API client | ✓ Covered |
| Dashboard polling | ✓ Covered |
| E2E (Playwright) | ✗ Not started |
| Rust handler E2E | ✗ Not started |
