# Spec: Logistics Workflow Web UI (Vue.js)

## Objective

Replace the Qt6/QML desktop frontend with a modern Vue.js single-page web application. The Rust backend stays as the API server, enhanced with proper REST endpoints, CORS, and structured JSON responses. Users access the logistics workflow via any modern browser instead of a desktop binary.

### User Stories

- Admin opens browser → sees dashboard with shipment stats
- Admin creates new shipment via web form (Step 1: Create)
- Manager fills draft documentation (Step 2: Draft)
- Accounting records customs clearance (Step 3: Customs)
- Team tracks document checklist (Step 4: Checklist → Telex Release)
- Export completed shipments to Excel workbook1 format
- Responsive: works on desktop, tablet, and phone

### Success Criteria

- [ ] Full 4-step workflow functional in browser (Create → Draft → Customs → Checklist)
- [ ] All 30 workbook1 fields creatable/editable via web forms
- [ ] Dashboard with live shipment stats
- [ ] Shipment list with search/filter
- [ ] Excel export downloads to browser
- [ ] Responsive layout (sidebar collapses on mobile)
- [ ] Zero build warnings, clean lint
- [ ] Rust backend: proper REST API, CORS, JSON error format

## Tech Stack

| Component | Technology | Version |
|---|---|---|
| Frontend | Vue.js 3 + Vite + TypeScript | latest |
| UI Components | Custom (no heavy lib — keep it light) | — |
| Styling | CSS with design tokens (CSS custom properties) | — |
| State | Pinia (Vuex successor) | latest |
| HTTP | fetch API (native) | — |
| Router | Vue Router 4 | latest |
| Backend | Rust (existing) | — |
| API | Enhanced HTTP bridge → proper REST | — |
| DB | PostgreSQL 17 (unchanged) | — |

## Architecture

```
Browser (Vue.js SPA)
    │
    │  HTTP REST API (JSON)
    ▼
Rust Backend (localhost:19876)
    │
    │  sqlx
    ▼
PostgreSQL (logistics_workflow)
```

### API Design

The current raw TCP server on :19876 becomes a proper REST API:

```
GET    /api/shipments              → List all shipments (with ?status= filter)
GET    /api/shipments/:id          → Get single shipment
POST   /api/shipments              → Create new shipment
PATCH  /api/shipments/:id          → Update shipment fields
PATCH  /api/shipments/:id/checklist → Toggle checklist boolean
GET    /api/shipments/:id/export   → Export single shipment to xlsx
GET    /api/export/all             → Export all shipments (workbook1 format)
GET    /api/dashboard              → Dashboard summary stats
```

### CORS

All responses include:
```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, PATCH, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

### Error Format (consistent)

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Booking number is required"
  }
}
```

## Project Structure

```
~/logistics-workflow/
├── web/                          ← NEW: Vue.js frontend
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── index.html
│   ├── public/
│   └── src/
│       ├── main.ts               ← App entry
│       ├── App.vue               ← Root component
│       ├── router/
│       │   └── index.ts          ← Routes
│       ├── stores/
│       │   ├── shipments.ts      ← Shipment data store (Pinia)
│       │   └── dashboard.ts      ← Dashboard stats store
│       ├── api/
│       │   └── client.ts         ← HTTP client for Rust backend
│       ├── components/
│       │   ├── layout/
│       │   │   ├── AppHeader.vue
│       │   │   ├── AppSidebar.vue
│       │   │   └── AppLayout.vue
│       │   ├── workflow/
│       │   │   ├── WorkflowProgress.vue
│       │   │   ├── Step1Create.vue
│       │   │   ├── Step2Draft.vue
│       │   │   ├── Step3Customs.vue
│       │   │   └── Step4Checklist.vue
│       │   ├── dashboard/
│       │   │   ├── StatCard.vue
│       │   │   └── ShipmentTable.vue
│       │   └── shared/
│       │       ├── DatePicker.vue
│       │       ├── StatusBadge.vue
│       │       └── Toast.vue
│       ├── views/
│       │   ├── DashboardView.vue
│       │   ├── WorkflowView.vue
│       │   └── ExportView.vue
│       └── styles/
│           ├── tokens.css        ← Design tokens (colors, spacing, fonts)
│           └── global.css        ← Reset + base styles
├── src/                          ← Rust backend (existing)
│   ├── main.rs                   ← MODIFY: add proper HTTP server
│   ├── bridge.rs                 ← MODIFY: enhance to REST API
│   └── ...
└── ...
```

## Design System

### Role Colors (from existing Theme.qml)

| Role | Color | Hex |
|---|---|---|
| ADMIN | Red | #E74C3C |
| MANAGER | Blue | #3498DB |
| ACCOUNTING | Orange | #F39C12 |
| LOGISTICS | Purple | #9B59B6 |

### Status Colors

| Status | Color | Hex |
|---|---|---|
| DRAFT | Gray | #95A5A6 |
| DOCUMENTS_READY | Blue | #3498DB |
| CUSTOMS_CLEARED | Orange | #F39C12 |
| CHECKLIST_IN_PROGRESS | Green | #27AE60 |
| TELEX_RELEASED | Purple | #9B59B6 |
| COMPLETE | Teal | #1ABC9C |

### Spacing Scale

```
--space-xs: 4px
--space-sm: 8px
--space-md: 16px
--space-lg: 24px
--space-xl: 32px
--space-2xl: 48px
```

### Typography

```
--font-sans: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif
--font-mono: 'JetBrains Mono', monospace
--text-xs: 0.75rem
--text-sm: 0.875rem
--text-base: 1rem
--text-lg: 1.125rem
--text-xl: 1.25rem
--text-2xl: 1.5rem
```

## UI Layout (Desktop)

```
┌──────────────────────────────────────────────────────────┐
│ AppHeader (logo, export btn, settings)                    │
├──────────────┬───────────────────────────────────────────┤
│ AppSidebar   │ Router View                               │
│              │                                           │
│ [+ New]      │ ┌─── WorkflowProgress ────────────────┐   │
│ [Search..]   │ │ CREATE → DRAFT → CUSTOMS → CHECKLIST│   │
│              │ └──────────────────────────────────────┘   │
│ SHP-001 ✓    │                                           │
│ SHP-002 📄   │ ┌─── Step Form ──────────────────────┐   │
│ SHP-003 🛃   │ │  [9-11 fields for current step]     │   │
│ SHP-004 📋   │ │                                      │   │
│ SHP-005 ✓    │ │  [Save] [Save & Continue →]         │   │
│              │ └──────────────────────────────────────┘   │
└──────────────┴───────────────────────────────────────────┘
```

### Mobile Layout

Sidebar collapses to hamburger menu. Workflow steps stack vertically. Full-width forms.

## Component Inventory

### Views (3)
1. **DashboardView** — stat cards + recent shipments table
2. **WorkflowView** — sidebar + 4-step workflow (main view)
3. **ExportView** — export preview + download button

### Layout (3)
4. **AppLayout** — header + sidebar + router-view shell
5. **AppHeader** — logo, nav links, export button
6. **AppSidebar** — shipment list, search, new button

### Workflow (5)
7. **WorkflowProgress** — 4-step indicator
8. **Step1Create** — 9-field ADMIN form (red)
9. **Step2Draft** — 8-field MANAGER form (blue)
10. **Step3Customs** — 3-field ACCOUNTING form (orange)
11. **Step4Checklist** — 11-field multi-section checklist

### Dashboard (2)
12. **StatCard** — single stat (total, draft, customs, telex)
13. **ShipmentTable** — recent shipments table

### Shared (3)
14. **DatePicker** — native date input styled
15. **StatusBadge** — colored status pill
16. **Toast** — notification banner

## Data Flow

```
Vue Component
    │  @submit
    ▼
Pinia Store (action)
    │  fetch()
    ▼
API Client (api/client.ts)
    │  POST /api/shipments
    ▼
Rust Backend (:19876)
    │  sqlx query
    ▼
PostgreSQL
    │
    ▼
Rust → JSON response
    │
    ▼
Pinia Store (state updated)
    │  reactivity
    ▼
Vue Component (re-renders)
```

## Code Style

```vue
<!-- Step1Create.vue -->
<script setup lang="ts">
import { ref } from 'vue';
import { useShipmentsStore } from '@/stores/shipments';

const store = useShipmentsStore();
const form = ref({
  sc_po_id: '',
  buyer_name: '',
  booking_number: '',
  shipping_line: '',
  origin_port: '',
  // ...
});

async function handleSubmit() {
  await store.createShipment(form.value);
}
</script>

<template>
  <form @submit.prevent="handleSubmit" class="step-form step-1">
    <div class="step-header" style="--accent: var(--color-admin)">
      <h2>STEP 1: CREATE SHIPMENT</h2>
      <StatusBadge label="ADMIN" color="var(--color-admin)" />
    </div>
    <div class="form-grid">
      <label>
        SC/PO ID <span class="required">*</span>
        <input v-model="form.sc_po_id" required />
      </label>
      <!-- ... 8 more fields -->
    </div>
    <div class="form-actions">
      <button type="submit" class="btn-primary">Save & Continue</button>
    </div>
  </form>
</template>

<style scoped>
.step-header {
  border-left: 4px solid var(--accent);
  padding: var(--space-md);
  background: var(--surface);
}
/* ... */
</style>
```

```typescript
// api/client.ts
const BASE = 'http://127.0.0.1:19876';

export interface Shipment { /* 30 fields */ }

export async function listShipments(status?: string): Promise<Shipment[]> {
  const params = status ? `?status=${status}` : '';
  const res = await fetch(`${BASE}/api/shipments${params}`);
  if (!res.ok) throw new ApiError(await res.json());
  return res.json();
}

export async function createShipment(data: CreateInput): Promise<Shipment> {
  const res = await fetch(`${BASE}/api/shipments`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new ApiError(await res.json());
  return res.json();
}
```

## Routes

```typescript
const routes = [
  { path: '/',             component: DashboardView },
  { path: '/workflow',     component: WorkflowView },
  { path: '/workflow/:id', component: WorkflowView },  // with selected shipment
  { path: '/export',       component: ExportView },
];
```

## Commands

```
# Frontend
cd web && npm run dev        # Vite dev server (hot reload)
cd web && npm run build      # Production build → web/dist/
cd web && npm run lint       # ESLint + Prettier
cd web && npm run typecheck  # TypeScript check

# Backend (unchanged)
cargo build
cargo test
cargo clippy -- -D warnings
```

## Testing Strategy

- **Type safety:** TypeScript strict mode — catches field name typos at compile time
- **API types:** Shipment interface shared between frontend and backend (JSON contract)
- **Manual verification:** Each step form tested in browser against real DB
- **No unit tests for Vue:** TypeScript + strict types + compile-time checks catch the bugs. Visual verification catches layout issues.

## Boundaries

### Always Do
- Use Pinia stores for all data fetching (never fetch in components)
- Follow the spacing/color design tokens (no raw px/hex outside tokens.css)
- Mobile-first responsive: test at 320px, 768px, 1024px, 1440px
- Loading, error, and empty states for every data-driven component
- CORS headers on all API responses
- JSON error format consistent across all endpoints

### Ask First
- Adding npm dependencies beyond Vue/Router/Pinia/Vite core
- Changing database schema
- Modifying the 4-step workflow order or count
- Adding authentication/user management

### Never Do
- Hardcode API URLs (use config or env var)
- Inline styles (use scoped CSS or tokens)
- Skip loading/error/empty states
- Use `any` type in TypeScript (use proper interfaces)
- Expose internal DB error details in API responses

## Open Questions

None — all resolved. The Rust backend already has all DB operations, just needs the HTTP layer enhanced. Vue.js frontend mirrors the existing QML structure.

## Success Criteria (Actionable)

- [ ] `cd web && npm run dev` starts Vite dev server
- [ ] Browser at localhost:5173 shows dashboard with stats from real DB
- [ ] Workflow page: sidebar shows shipment list, clicking loads 4-step form
- [ ] Create shipment form: all 9 fields, "Save" creates in DB
- [ ] Draft form: all 8 fields, "Save" updates in DB
- [ ] Customs form: 3 fields with dropdown, saves correctly
- [ ] Checklist: 11 booleans/fields toggle and persist
- [ ] Export button downloads .xlsx file
- [ ] Mobile: sidebar collapses, forms stack vertically
- [ ] `npm run build` produces optimized dist/
- [ ] Zero TypeScript errors, zero ESLint warnings
- [ ] Rust backend: `cargo build` and `cargo clippy` clean
