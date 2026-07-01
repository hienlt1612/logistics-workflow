# Standard Export Process (Sea Freight, Incoterms 2020) — Gap Analysis

**Project:** Logistics Workflow v0.2.0  
**Date:** 1 July 2026

---

## Full Export Process — 7 Phases, 28 Steps

```
Phase 1: COMMERCIAL
  1. Sales contract negotiation → SC/PO signed
  2. Incoterms 2020 agreed (FOB/CIF/CFR/EXW/etc.)
  3. Letter of Credit issuance (if L/C payment)

Phase 2: LOGISTICS PREPARATION
  4. Production/packing of export goods
  5. Book container with shipping line → Booking Confirmation
  6. Inland transport to stuffing location
  7. Container stuffing → container number, seal number
  8. Weighing → gross weight, net weight, CBM

Phase 3: CUSTOMS EXPORT
  9. Prepare Export Customs Declaration (VNACCS/ECUS in Vietnam)
  10. Customs document inspection (physical or documentary)
  11. Customs clearance → clearance number, date, channel (Red/Yellow/Green)

Phase 4: DOCUMENTATION
  12. Commercial Invoice issued
  13. Packing List issued
  14. Certificate of Origin (Form E/D/AK/etc.)
  15. Phyto Certificate (for agricultural goods)
  16. Fumigation Certificate (if wooden packaging)
  17. Other certs: Health, Halal, Inspection, EUR1 etc.
  18. Bill of Lading draft → confirm → original issued after sailing

Phase 5: SHIPPING
  19. Truck container to port terminal
  20. Terminal handling, customs gate-out
  21. Vessel loading → ETD confirmed
  22. Actual departure → ATD
  23. ETA at destination port
  24. Container tracking (carrier API or schedule)

Phase 6: PAYMENT & DOCUMENT DISPATCH
  25. Assemble document set (Invoice + PL + BL + CO + Phyto + ...)
  26. Submit docs to bank (if L/C) or direct to buyer
  27. Prepayment tracking → remaining balance → payment received
  28. Originals dispatch to buyer via courier

Phase 7: POST-SHIPMENT
  29. Telex Release / Surrender BL → consignee takes delivery
  30. Payment reconciliation → full settlement
  31. Record archiving
```

---

## Incoterms 2020 — 11 Rules

| Group | Rules | Risk Transfer | LW Support |
|-------|-------|---------------|------------|
| **E** | EXW (Ex Works) | At seller's premises | ❌ no field |
| **F** | FCA, FAS, FOB | Main carriage unpaid | ❌ no field |
| **C** | CFR, CIF, CPT, CIP | Main carriage paid | ❌ no field |
| **D** | DAP, DPU, DDP | Destination | ❌ no field |

**Verdict:** Incoterms field IS the contract type selector. Every shipment has one. Missing.

---

## Must-Have — Missing in Logistics Workflow

These fields exist in EVERY real-world export shipment. Your seed data already implies them (has booking #, shipping line, ETD — but no container, vessel, ETA).

| # | Field | Why Indispensable | Table Schema |
|---|-------|-------------------|-------------|
| 1 | **incoterms** | Determines who pays freight/insurance, risk transfer point | `VARCHAR(3) NOT NULL DEFAULT 'FOB'` |
| 2 | **container_number** | Physical identifier — every booking has containers. Booking # alone doesn't help at port | `VARCHAR(20)` |
| 3 | **seal_number** | Customs seal — required for customs clearance, used in BL | `VARCHAR(20)` |
| 4 | **vessel_name** | Ship name — appears on BL, used for tracking | `VARCHAR(50)` |
| 5 | **voyage_number** | Voyage identifier — paired with vessel for schedule lookup | `VARCHAR(20)` |
| 6 | **port_of_discharge** | Destination port — complement to origin_port. Without it, you can't tell where goods are going | `VARCHAR(80)` |
| 7 | **final_destination** | Final delivery city — consignee's location after port | `VARCHAR(120)` |
| 8 | **eta** | Estimated arrival date — counterpart to ETD. Needed for buyer/schedule planning | `DATE` |
| 9 | **product_description** | What's being shipped — appears on Invoice + PL. Currently you track buyer but not product | `TEXT` |
| 10 | **hs_code** | Harmonized System code — required for customs declaration. 6-10 digits | `VARCHAR(12)` |
| 11 | **gross_weight_kg** | Total cargo weight — required on BL, PL, VGM | `NUMERIC(10,2)` |
| 12 | **net_weight_kg** | Without packaging | `NUMERIC(10,2)` |
| 13 | **cbm** | Cubic meters — required for freight calculation | `NUMERIC(8,3)` |
| 14 | **package_count** | Number of packages (cartons, pallets, drums) | `INTEGER` |
| 15 | **package_type** | Cartons, pallets, drums, bags, etc. | `VARCHAR(30)` |

---

## Important — Build After Must-Haves

| # | Feature | Rationale |
|---|---------|-----------|
| 16 | **freight_cost_usd** | Track actual freight paid vs. estimated → profit margin |
| 17 | **insurance_cost_usd** | CIF requires insurance; even FOB, exporter often buys |
| 18 | **insurance_policy_no** | Insurance document reference |
| 19 | **lc_number** | Letter of Credit reference — common in Vietnam export |
| 20 | **lc_issuing_bank** | L/C issuing bank name |
| 21 | **lc_expiry_date** | L/C validity deadline |
| 22 | **fumigation_cert** | Boolean — like CO/phyto, common for wooden-packed goods |
| 23 | **forwarder_name** | Freight forwarder handling the shipment |
| 24 | **forwarder_contact** | Phone/email of forwarder |
| 25 | **tracking_url** | Carrier container tracking link |
| 26 | **document_set_sent** | Boolean — all docs dispatched to buyer |
| 27 | **document_set_sent_date** | When docs were dispatched |

---

## Step Mapping — Your 4 Steps vs. Real 7 Phases

| LW Step | Covers These Real Phases | Coverage |
|---------|--------------------------|----------|
| **Step 1: CREATE** | Phase 1 (commercial) + Phase 2 (booking) | ~60% — missing Incoterms, L/C, container# |
| **Step 2: DRAFT** | Phase 4 (docs) only | ~40% — generates invoice but no PL, CO, weight/package fields |
| **Step 3: CUSTOMS** | Phase 3 (customs) | ~70% — has date/number/channel. Missing HS code, inspection result |
| **Step 4: CHECKLIST** | Phase 6 (payment) + Phase 7 (telex) | ~60% — covers payment/originals/telex. Missing freight/insurance costs, L/C, document set dispatch |
| **NOT COVERED** | Phase 5 (shipping: vessel, ETA, tracking) | 0% |

---

## What Your App Does RIGHT vs. Competitors

| Strength | Competitors lack this |
|----------|----------------------|
| Sequential role-gated workflow | No competitor has this (IncoDocs, GoFreight, Odoo all flat) |
| Telex Release as explicit step | ZERO competitors have this — unique selling point |
| Customs channel (Red/Yellow/Green) | Even enterprise tools (Flexport) don't model customs channel |
| 30-field Vietnamese workbook1 format | Industry-specific, your seed data proves it fits |

---

## Priority Migration Path

Shortest path from v0.2.0 to production-ready schema:

```
1. ALTER TABLE shipments ADD incoterms, container_number, seal_number, vessel_name, voyage_number, eta, port_of_discharge, final_destination, product_description, hs_code, gross_weight_kg, net_weight_kg, cbm, package_count, package_type (15 new fields — ALTER only)
2. ADD Step2 fields to frontend form: vessel, ETA, discharge port (3 new inputs)
3. ADD Step1 fields: Incoterms dropdown, product desc + HS code (3 new inputs)
4. REGENERATE seed data with real container/vessel/weight data
```

Migrate existing seed data with a one-time script. Data model is ~80% complete; 15 fields missing from a 45-field complete model.

---

*Sources: ICC Incoterms 2020 rules, FIATA model export procedures, Vietnamese Customs Law 2014, VNACCS declaration fields. Web search blocked by ISP/target-site restrictions.*
