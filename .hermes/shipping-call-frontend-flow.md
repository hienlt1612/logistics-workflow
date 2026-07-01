# New Shipping Call — Frontend Flow

## Screen 1: CREATE SHIPPING CALL

```
POST /api/shipping-calls
────────────────────────────────────────────────────────────────

┌─ CALL HEADER ──────────────────────────────────────────────┐
│ Buyer Name *          [________________]                    │
│ Incoterms *           [FOB ▼]                              │
│ Product Description   [________________]                    │
│ SC/PO ID              [________________]                    │
│ SC/PO Date            [____-__-__]                          │
│ SC/PO Made By         [________________]                    │
│                                                             │
│ Total Containers       [__]  (auto: SUM of warehouse plan)  │
└─────────────────────────────────────────────────────────────┘

┌─ WAREHOUSE PLAN ───────────────────────────────────────────┐
│ Warehouse           Containers                              │
│ [Nam Dinh       ▼]  [____]  [✕]                            │  ← dropdown: pre-populated
│ [Quang Chau     ▼]  [____]  [✕]                            │      from existing warehouses
│                                                             │
│ [+ Add Warehouse]                                           │
│                                                             │
│       Planned: 8 containers  |  0 loaded                    │
└─────────────────────────────────────────────────────────────┘

┌─ FIRST BOOKING (optional) ─────────────────────────────────┐
│ Booking Number *      [________________]                    │
│ Shipping Line *       [________________]                    │
│ Origin Port *         [________________]                    │
│ Loading Plan          [________________]                    │
│ ETD                   [____-__-__]                          │
│                                                             │
│ ┌─ Containers ──────────────────────────────────────────┐  │
│ │ Container#    Seal#      Warehouse       Weight   CBM │  │
│ │ [_________]  [_______]  [Nam Dinh ▼]   [_____] [____]│  │
│ │ [_________]  [_______]  [Quang Chau ▼]  [_____] [____]│  │
│ │ [+ Add Container]                                     │  │
│ └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘

    [Cancel]                        [Create Shipping Call]

    → Creates CALL-2026-001 + warehouses + shipment + containers
    → Redirect to detail
```

## Screen 2: SHIPPING CALL DETAIL

```
GET /api/shipping-calls/:id
────────────────────────────────────────────────────────────────

┌─ CALL HEADER ───────────────────────────────────────────────┐
│ CALL-2026-001  │  Buyer: Kotor  │  FOB  │  8 containers     │
│ Cassava Starch │  SC/PO: #42    │  Status: PARTIAL          │
│                                        [Edit] [Delete]       │
└──────────────────────────────────────────────────────────────┘

┌─ WAREHOUSE PROGRESS ────────────────────────────────────────┐
│ Nam Dinh     ████████░░░░  5/5 loaded  ✓ done               │
│ Quang Chau   ██░░░░░░░░░░  1/3 loaded                       │
│                                                              │
│       Total: 6 / 8 containers loaded                        │
└──────────────────────────────────────────────────────────────┘

┌─ BOOKINGS ──────────────────────────────────────────────────┐
│ ┌──────────────────────────────────────────────────────────┐│
│ │ YMLAN6546547123  Yang Ming  Haiphong → Kotor  ETD 28/06 ││
│ │ 5 containers: TCLU1234, TCLU5678, ...                   ││
│ │ Status: DOCUMENTS_READY          [View] [Add Container]  ││
│ ├──────────────────────────────────────────────────────────┤│
│ │ YMLAN6546547159  Yang Ming  Haiphong → Kotor  ETD 30/06 ││
│ │ 1 container: TCLU9012 (Quang Chau)                      ││
│ │ Status: DRAFT                    [View] [Add Container]  ││
│ └──────────────────────────────────────────────────────────┘│
│                                          [+ New Booking]     │
└──────────────────────────────────────────────────────────────┘
```

## States

| State | Trigger |
|-------|---------|
| **OPEN** | Created, 0 containers loaded |
| **PARTIAL** | Some containers loaded, < planned |
| **COMPLETE** | loaded_containers = planned_containers per warehouse |
| **SHIPPED** | All bookings have ETD passed |

## Computations (no stored field)

```
call.total_containers    = SUM(warehouses.planned_containers)  -- shown as auto
call.loaded_containers   = SUM(warehouses.loaded_containers)   -- backend computed
warehouse.loaded_count   = COUNT(containers WHERE warehouse_name = this warehouse AND status = 'LOADED')
warehouse.progress %     = loaded / planned × 100
call.progress %          = total_loaded / total_planned × 100
```

## Container Lifecycle

```
[+ Add Container] → row inserted, status = PENDING
  → user fills container#, seal#, warehouse (from dropdown), weight, CBM
  → POST /api/containers

[Mark Loaded]     → status = LOADED
  → warehouse.loaded_containers +1

[Mark Shipped]    → status = IN_TRANSIT
  → after ETD passed
```

## Skipped

- `quantity_unit` field removed — one unit = one container. YAGNI 20ft/40ft distinction until user asks.
- Call-level auto status computed from warehouse aggregate, not stored. Query-level, no state sync.
- Warehouse dropdown on container form sourced from `call_warehouses`, not a master warehouse list.
- "Split booking" — user adds second booking manually. No auto-split wizard.
