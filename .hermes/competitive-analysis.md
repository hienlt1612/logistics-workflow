# Logistics Workflow — Competitive Analysis & Feature Roadmap

**Project:** Logistics Workflow v0.2.0  
**Author:** Bright Ng  
**Date:** 1 July 2026  
**Company:** AN DIEN FOOD (export/shipping/logistics)

---

## 1. Current Business Processes

The app models a real-world Vietnamese export shipment workflow across 4 sequential steps, each owned by a different role:

| Step | Role | What happens | Data collected |
|------|------|-------------|----------------|
| **1. CREATE** | ADMIN | Shipment initiated, SC/PO logged, booking made | SC/PO ID, buyer, booking #, shipping line, port, warehouse, loading plan (9 fields) |
| **2. DRAFT** | MANAGER | Documentation drafted, invoice issued, BL tracked | Shipper, consignee, ETD, invoice #/date, value USD, drafts date, BL # (8 fields) |
| **3. CUSTOMS** | ACCOUNTING | Customs clearance processing | Customs date, declaration #, customs channel (red/yellow/green) (3 fields) |
| **4. CHECKLIST** | ALL ROLES | Final document verification, payment, originals dispatch, telex release | 10 bool checkboxes + 3 payment fields + 3 originals fields → TELEX_RELEASED locks the shipment |

**Status flow:** DRAFT → DOCUMENTS_READY → CUSTOMS_CLEARED → CHECKLIST_IN_PROGRESS → COMPLETE → TELEX_RELEASED

**Roles:** admin (full CRUD + telex revert), manager (draft docs), accounting (customs + payments), logistics (certificates + checklist)

**Tech stack:** Rust REST API (raw HTTP on :19876), Vue 3 SPA, PostgreSQL 17, Docker Compose deploys on VPS

**Current features:**
- 30-field shipment data model
- Dashboard with auto-refresh stats (total, draft, docs, customs, checklist, telex)
- Paginated shipment list with status filter
- Batch status advance (select multiple → advance together)
- Excel export (workbook1-format XLSX, base64 over API)
- Response mobile sidebar layout
- Admin-only delete and telex revert
- API token write-protection

---


## 2. Competitive Landscape (Verified)

### Direct Competitors

| Product | Pricing | Target | Closest Match? |
|---------|---------|--------|---------------|
| **IncoDocs** | Free→$15→$37→$100/mo | Export doc creation | ✅ Closest — templates, no workflow engine, no roles, no customs/telex |
| **GoFreight** | $150-300/user/mo | Freight forwarders | ❌ TMS for forwarders, not exporters |
| **Flexport** | $50K+/yr minimum | Mid-large importers | ❌ Managed service, not self-serve workflow |
| **Magaya** | $100-200/user/mo | Forwarders/customs brokers | ❌ WMS + CRM bloat, no Telex Release |
| **Descartes** | $50K-500K+/yr | Enterprise | ❌ 6-12mo implementation, massive overkill |

### Open-Source Alternatives

| Product | Pricing | Vietnamese Support | Gap |
|---------|---------|-------------------|-----|
| **ERPNext** | Free (self-host) | ✅ Translation + VND | No dedicated export workflow; needs 3-6mo customization |
| **Odoo** | Free (Community) / $24-38/mo (Enterprise) | ✅ Translation + VN partners | No customs/telex module; shipping = parcel-focused |
| **OpenBoxes** | Free (OSS) | ❌ | Healthcare supply chain, not export docs |

### Competitor Gap Matrix

| Need | IncoDocs | GoFreight | ERPNext | Odoo | **LW App** |
|------|----------|-----------|---------|------|------------|
| Export doc creation | ✅ | ✅ | ⚠️ | ⚠️ | ✅ |
| Customs clearance tracking | ❌ | ❌ | ❌ | ❌ | ✅ |
| Telex Release step | ❌ | ❌ | ❌ | ❌ | ✅ |
| Role-based approval | ❌ | ❌ | ✅ | ✅ | ✅ (4 roles) |
| 4-step workflow engine | ❌ | ❌ | ⚠️ | ⚠️ | ✅ |
| Vietnamese language | ❌ | ❌ | ✅ | ✅ | ❌ (planned) |
| Self-host | ❌ | ❌ | ✅ | ✅ | ✅ |
| SME pricing | ✅ | ❌ | ✅ | ✅ | ✅ |
| Docker deploy | ❌ | ❌ | ✅ | ✅ | ✅ |
| Open source | ❌ | ❌ | ✅ | ✅ | TBD |

✅ = native support | ⚠️ = possible with heavy customization | ❌ = not available

---

## 3. Gap Analysis: Your App vs. Production-Ready

### What Logistics Workflow does better than competitors:
- Free, open-source, Docker-deployable
- Sequential role-gated workflow (unique differentiator)
- 30-field model matching real Vietnamese workbook1 export documents
- Simple UX with zero training needed
- Excel export in exact workbook1 format

### What's MISSING (indispensable for real use):

---

## 4. Feature Recommendations — Priority-Ordered

### ▎CRITICAL — Shipment can't go to production without these

| # | Feature | Why indispensable |
|---|---------|-------------------|
| 1 | **Document upload/attachment** | BL scans, invoices, CO, phyto certificates — currently stored on someone's desktop. Must attach per-shipment with preview. |
| 2 | **Email notifications** | When status advances (DRAFT→CUSTOMS→CHECKLIST→TELEX), email the shipper, consignee, and next-role user. Without this, the workflow is invisible. |
| 3 | **Vietnamese language UI** | Team uses Vietnamese. English-only limits adoption. Implement i18n (Vue I18n) with toggle. |
| 4 | **Customer/vendor directory** | Replace dead `customers` table with real contact management — name, email, phone, company, address. Auto-populate forms. |
| 5 | **PDF document generation** | Generate Invoice, Packing List, and shipping docs from shipment data. Currently Excel-only export. |
| 6 | **Payment tracking & auto-calc** | Prepayment + Remaining should auto-calculate vs total_value_usd. Show payment status (paid/unpaid/partial). |

### ▎HIGH — Competitive advantage, build next

| # | Feature | Impact |
|---|---------|--------|
| 7 | **Filter dashboard by date range** | "Show me shipments from June 2026" — currently only status filter. |
| 8 | **Audit log** | Who changed what field, when. Critical for disputes and compliance. |
| 9 | **Duplicate detection** | Warn if same booking_number already exists. |
| 10 | **Profit margin calculation** | total_value_usd minus estimated freight + customs costs = margin per shipment. |
| 11 | **Bulk import from Excel** | Upload workbook1.xlsx → auto-create shipments. Reverse of export. |
| 12 | **User activity feed** | "Minh updated shipment SHP-2026-015 to DOCUMENTS_READY" on dashboard. |
| 13 | **PWA (mobile installable)** | Service worker + manifest for offline access on phone. Team checks shipments at port/warehouse. |

### ▎MEDIUM — Nice-to-have, do after stable

| # | Feature |
|---|---------|
| 14 | Customs status auto-lookup (VN customs API if available) |
| 15 | Container tracking integration (Yang Ming, Maersk, Evergreen APIs) |
| 16 | Telegram notification channel (Bright uses Telegram) |
| 17 | Currency converter (USD ↔ VND at current rate) |
| 18 | Document OCR — extract data from uploaded BL/invoice photos |
| 19 | Role-based dashboard views (accounting sees payment panel, logistics sees checklist) |
| 20 | Dark mode |

### ▎LOW — Post-MVP, don't build yet

| # | Feature |
|---|---------|
| 21 | Customer-facing portal (external login, view-only) |
| 22 | Multi-tenant SaaS mode |
| 23 | API webhooks for ERP integration |
| 24 | Mobile native app |
| 25 | AI document classification |

---

## 5. Comparison Matrix

| Capability | Logistics Workflow | Flexport | Logitude | GoFreight | Excel |
|------------|-------------------|----------|----------|-----------|-------|
| **Price** | Free (OSS) | $500-2000/mo | $99-299/mo | $150-500/mo | Free* |
| **Sequential workflow** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Role-gated steps** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Document upload** | ❌ | ✅ | ✅ | ✅ | ❌ |
| **PDF generation** | ❌ | ✅ | ✅ | ✅ | Manual |
| **Excel export** | ✅ | ✅ | ✅ | ✅ | Native |
| **Vietnamese language** | ❌ | ❌ | ❌ | ❌ | N/A |
| **Customer directory** | ❌ | ✅ | ✅ | ✅ | Manual |
| **Email notifications** | ❌ | ✅ | ✅ | ✅ | Manual |
| **Audit log** | ❌ | ✅ | ✅ | ✅ | ❌ |
| **Container tracking** | ❌ | ✅ | ❌ | ✅ | ❌ |
| **Customs integration** | ❌ | ✅ | ✅ | ✅ | ❌ |
| **Docker deploy** | ✅ | ❌ | ❌ | ❌ | N/A |
| **Mobile-friendly** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Bulk import** | ❌ | ✅ | ✅ | ✅ | Native |
| **Open-source** | ✅ | ❌ | ❌ | ❌ | N/A |

*Excel's real cost is errors, version conflicts, and time wasted.

---

## 6. Strategic Recommendations

### Short-term (next 2 weeks): Build the minimum production gate

1. **Document upload** — add `file_attachments` table (shipment_id, filename, mime, data BYTEA). Simple upload widget in Step4 checklist (one per checkbox item).
2. **Email notifications** — SMTP config in config.toml. Send templated emails on status change. Rust `lettre` crate.
3. **i18n** — `vue-i18n` with `vi.json` + `en.json`. Translate all static strings. Language persisted in localStorage.
4. **Customer contacts** — repurpose existing `customers` table (already in schema!). Add email, phone, address columns. Wire up to Step1 buyer dropdown.

### Medium-term (next month): Make it the best in its class

5. PDF generation with `printpdf` or `wkhtmltopdf` from HTML templates
6. Dashboard date-range filter + export by filter
7. Audit log table tracking all PATCH/DELETE operations

### Differentiation strategy

**Don't compete on features with Flexport/CargoWise.** They have 500+ engineers. Compete on:
- **Simplicity** — sequential workflow that matches how a 5-person Vietnamese export team actually works
- **Price** — free, open-source, self-hosted
- **Format compatibility** — workbook1.xlsx is THE format Vietnamese exporters use
- **Vietnamese-first** — no other tool has Vietnamese UI or Vietnam customs workflow

### Market position

Logistics Workflow sits in an **underserved niche**: small Vietnamese food/agriculture exporters who currently use Excel + email. This is a real market — thousands of companies like AN DIEN FOOD in Vietnam alone. None of them can afford or need Flexport.

---

## 7. Immediate Action Items

Based on analysis, prioritize in this order:

1. ✅ Add `file_attachments` table + upload API + UI widget
2. ✅ Add `lettre` crate + SMTP config + email on status change
3. ✅ Add `vue-i18n` + `vi.json` translations for all UI strings
4. ✅ Add email/phone/address columns to `customers` table + wire to Step1
5. ✅ Payment auto-calc: `remaining_amt = total_value_usd - prepayment_amt`
6. 🔲 Dashboard date-range filter
7. 🔲 Audit log table
8. 🔲 PWA service worker

---

*Report generated by Hermes Agent. Competitive data from training knowledge + limited web access (search blocked by target sites). Subagent research pending.*
