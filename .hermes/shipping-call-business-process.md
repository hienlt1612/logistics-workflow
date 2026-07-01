# New Shipping Call — Frontend Business Process

## Full Flow (6 steps, 2 screens)

```
Screen 1: CREATE SHIPPING CALL
═══════════════════════════════════════════════════════════════

┌─ SECTION A: Call Header ───────────────────────────────────┐
│ Buyer Name *          [________________]                    │
│ Incoterms *           [FOB ▼]  (EXW/FCA/FAS/FOB/CFR/CIF...)│
│ Product Description   [________________]                    │
│ Total Quantity        [______]  [tons ▼]                    │
│ SC/PO ID              [________________]                    │
│ SC/PO Date            [____-__-__]  (date picker)           │
│ SC/PO Made By         [________________]                    │
└─────────────────────────────────────────────────────────────┘

┌─ SECTION B: Warehouse Loading Plan ────────────────────────┐
│ ┌──────────┬──────────────┬──────┐                         │
│ │ Warehouse│ Planned Qty  │      │                         │
│ ├──────────┼──────────────┼──────┤                         │
│ │ Nam Dinh │ 120.00  tons │  [✕] │                         │
│ │ Quang Ch │ 80.00   tons │  [✕] │                         │
│ └──────────┴──────────────┴──────┘                         │
│ [+ Add Warehouse]                                          │
│                                                            │
│ Progress bar: 0 / 200 tons loaded                          │
└─────────────────────────────────────────────────────────────┘

┌─ SECTION C: First Booking (optional — add now or later) ───┐
│ Booking Number *      [________________]                    │
│ Shipping Line *       [________________]                    │
│ Origin Port *         [________________]                    │
│ Loading Plan          [________________]                    │
│ ETD                   [____-__-__]                          │
│ Shipper/Exporter      [________________]                    │
│ Consignee             [________________]                    │
│                                                             │
│ ┌─ Containers (this booking) ────────────────────────────┐ │
│ │ ┌──────────┬──────────┬───────────┬────────┬─────────┐ │ │
│ │ │ Container│ Seal #   │ Warehouse │ Weight │ CBM     │ │ │
│ │ ├──────────┼──────────┼───────────┼────────┼─────────┤ │ │
│ │ │ TCLU1234 │ SEAL001  │ Nam Dinh  │ 25000  │ 33.2    │ │ │
│ │ │ TCLU5678 │ SEAL002  │ Quang Chau│ 25000  │ 33.2    │ │ │
│ │ └──────────┴──────────┴───────────┴────────┴─────────┘ │ │
│ │ [+ Add Container]                                       │ │
│ └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘

    [Cancel]                    [Create Shipping Call]

    POST /api/shipping-calls
    Body: { call_header, warehouses[], shipment, containers[] }
    → Creates CALL-2026-001 + warehouse plan + first booking + containers
    → Redirect to call detail


Screen 2: SHIPPING CALL DETAIL
═══════════════════════════════════════════════════════════════

┌─ Call Header ───────────────────────────────────────────────┐
│ CALL-2026-001  |  Buyer: Kotor  |  FOB  |  Total: 200 tons │
│ Cassava Starch  |  SC/PO: #42  |  Status: OPEN              │
└─────────────────────────────────────────────────────────────┘

┌─ Warehouse Progress ────────────────────────────────────────┐
│ Nam Dinh    ████████░░░░░░░░  120/200 tons (60%)           │
│ Quang Chau  ████████░░░░░░░░   80/200 tons (40%)           │
└─────────────────────────────────────────────────────────────┘

┌─ Bookings (2 of 3 loaded) ──────────────────────────────────┐
│ ┌──────────────────────────────────────────────────────────┐│
│ │ YMLAN6546547123  Yang Ming   Haiphong → Kotor  ETD 28/06││
│ │ 2 containers loaded: TCLU1234 (Nam Dinh), TCLU5678 (QC) ││
│ │ Status: DOCUMENTS_READY            [View] [Add Container]││
│ ├──────────────────────────────────────────────────────────┤│
│ │ YMLAN6546547159  Yang Ming   Haiphong → Kotor  ETD 30/06││
│ │ 0 containers (planned)                                   ││
│ │ Status: DRAFT                       [View] [Add Container]││
│ └──────────────────────────────────────────────────────────┘│
│ [+ New Booking]                                             │
└──────────────────────────────────────────────────────────────┘

    [← Back to Calls]                              [Edit Call]
```

## Key Behaviors

| Action | What Happens |
|--------|-------------|
| Create call without booking | Call status = OPEN. Bookings added later. |
| Create call with booking | Single POST. Call + first shipment + containers in one transaction. |
| Add booking to existing call | "New Booking" button → modal/slide-over → fill booking fields → POST. |
| Add container to booking | "Add Container" per booking → fill container#/seal/warehouse/weight → POST. |
| Warehouse progress bar | Aggregates loaded_quantity / planned_quantity across all call_warehouses. |

## Ponytail Simplifications

- **Skipped:** Inline-editable warehouse list on detail screen. Build when user asks to edit warehouse plan after creation.
- **Skipped:** Drag-and-drop container assignment to bookings. Select warehouse from dropdown.
- **Skipped:** Partial load tracking per container. Start with PENDING/LOADED boolean. Add quantity tracking when a container loads half from warehouse A and half from warehouse B.
- **Skipped:** Call-level status automation (OPEN→PARTIAL→COMPLETE). Computed from bookings status. Add when status gating logic is needed.
- **Skipped:** "Split booking" feature (when loading can't finish → split to another booking). User creates a second booking manually. Add auto-split when user measures the pain.

## Data Flow

```
ShippingCall
  ├── call_warehouses[]     (planned per warehouse)
  ├── shipments[]           (bookings — each is a Shipment with FK to call)
  │     └── containers[]    (per booking, per warehouse)
  └── computed:
        total_loaded = SUM(containers.loaded_quantity)
        warehouse_progress = loaded_quantity / planned_quantity per call_warehouse
```
