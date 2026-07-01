#!/usr/bin/env python3
"""E2E test-data generator + business-process driver for logistics-workflow.

Drives the LIVE API (port 19876): creates 10 shipping calls with fake data,
generates shipments per call, fills container loading, advances every shipment
through the full workflow to TELEX_RELEASED, then closes each call.
Records every API call and writes a markdown test report.

ponytail: stdlib only (urllib), no deps. Failures are recorded, not fatal —
the whole run completes so the report captures every defect in one pass.
"""
import json, random, subprocess, datetime as dt, sys, os
# ponytail: shell out to curl — the server's naive HTTP parser drops the body
# when a client (urllib) splits headers/body across TCP segments. curl coalesces.
# Noted as a server-robustness finding in the report.

BASE = "http://127.0.0.1:19876"
USER, PW = "tp_admin", "@tp_admin123"
random.seed(42)  # ponytail: deterministic runs; drop seed for varied data

# ---- fake data pools ----
BUYERS = ["ACME Foods Ltd","Pacific Rim Trading","Golden Harvest Co","EuroFresh Imports",
          "Sunrise Commodities","Nordic Seafood AS","Sahara Grains LLC","Andes Coffee Export",
          "Mekong Agri Group","Baltic Provisions"]
INCOTERMS = ["FOB","CIF","CFR","EXW","DAP"]
PRODUCTS = ["Frozen Pangasius","Roasted Cashew W320","Robusta Green Coffee","Long Grain White Rice",
            "Black Pepper 550G/L","Dried Coconut","Frozen Shrimp HLSO","Instant Coffee",
            "Cassava Starch","Passion Fruit Puree"]
PORTS = ["Cat Lai","Hai Phong","Cai Mep","Da Nang","Qui Nhon"]
LINES = ["Maersk","MSC","CMA CGM","ONE","Hapag-Lloyd","Evergreen","COSCO"]
WHS = ["WH-Binh Duong","WH-Long An","WH-Dong Nai","WH-Cu Chi","WH-Hai Phong","WH-Can Tho"]
LINE_PREFIX = {"Maersk":"MSKU","MSC":"MSCU","CMA CGM":"CMAU","ONE":"ONEU",
               "Hapag-Lloyd":"HLXU","Evergreen":"EGHU","COSCO":"CSNU"}
SHIPPER = "AN DIEN FOOD JSC"

TODAY = dt.date.today()
def d(offset):  # date string offset days from today
    return (TODAY + dt.timedelta(days=offset)).isoformat()

results = []  # (ok, method, path, status, note)
def ok(s): return 200 <= s < 300
def api(method, path, body=None, token=None):
    cmd = ["curl", "-s", "-X", method, BASE + path,
           "-H", "Content-Type: application/json",
           "-w", "\n%{http_code}"]
    if token:
        cmd += ["-H", f"Authorization: Bearer {token}", "-H", "X-User-Role: admin"]
    if body is not None:
        cmd += ["-d", json.dumps(body)]
    try:
        out = subprocess.run(cmd, capture_output=True, text=True, timeout=20).stdout
        nl = out.rfind("\n")
        raw, code = out[:nl], int(out[nl+1:] or 0)
        js = json.loads(raw) if raw.strip() else {}
        ok = 200 <= code < 300
        results.append((ok, method, path, code, "" if ok else raw[:300]))
        return code, (js if isinstance(js, dict) else {"_data": js})
    except Exception as e:
        results.append((False, method, path, 0, str(e)[:300]))
        return 0, {"_error": str(e)[:300]}

def cnum(line, n): return f"{LINE_PREFIX[line]}{random.randint(1000000,9999999)}"

# ---- login ----
st, js = api("POST", "/api/login", {"username": USER, "password": PW})
if not ok(st) or "token" not in js:
    print(f"LOGIN FAILED ({st}): {js}"); sys.exit(1)
TOKEN = js["token"]
print(f"logged in as {js.get('username')} role={js.get('role')}")

call_reports = []

for ci in range(1, 11):
    buyer = random.choice(BUYERS)
    inco = random.choice(INCOTERMS)
    product = random.choice(PRODUCTS)
    total = random.randint(8, 25)
    # split total across 1-3 warehouses, each >=1
    n_wh = random.randint(1, 3)
    n_wh = min(n_wh, total)
    picks = random.sample(WHS, n_wh)
    # partition `total` into n_wh positive parts
    cuts = sorted(random.sample(range(1, total), n_wh-1)) if n_wh > 1 else []
    bounds = [0] + cuts + [total]
    wh_plan = [{"warehouse_name": picks[i], "planned_containers": bounds[i+1]-bounds[i]}
               for i in range(n_wh)]

    line = random.choice(LINES)
    call_body = {
        "sc_po_id": f"PO-{ci:03d}-{random.randint(100,999)}",
        "sc_po_date": d(-random.randint(20,60)),
        "sc_po_by": buyer,
        "buyer_name": buyer,
        "incoterms": inco,
        "product_description": product,
        "total_containers": total,
        "warehouses": wh_plan,
    }
    st, call = api("POST", "/api/shipping-calls", call_body, TOKEN)
    if not ok(st) or "id" not in call:
        call_reports.append({"idx": ci, "status": "CALL_CREATE_FAILED", "http": st,
                             "err": call.get("_error",""), "total": total})
        continue
    cid, cref = call["id"], call.get("call_ref", f"#{call['id']}")

    # shipments (bookings): 1..min(4, n_wh+1)
    n_ship = random.randint(1, min(4, total))
    ship_ids = []
    for si in range(n_ship):
        sbody = {
            "sc_po_id": call_body["sc_po_id"],
            "sc_po_date": call_body["sc_po_date"],
            "sc_po_by": buyer,
            "buyer_name": buyer,
            "booking_number": f"BKG{random.randint(10000000,99999999)}",
            "shipping_line": line,
            "origin_port": random.choice(PORTS),
            "warehouse_loc": picks[0],
            "loading_plan": f"Load {product} @ {picks[0]}",
            "shipping_call_id": cid,
        }
        st, sh = api("POST", "/api/shipments", sbody, TOKEN)
        if ok(st) and "id" in sh:
            ship_ids.append(sh["id"])
    if not ship_ids:
        call_reports.append({"idx": ci, "ref": cref, "status": "NO_SHIPMENTS", "total": total})
        continue

    # fill containers: for each warehouse, create `planned` containers,
    # round-robin assigned to shipments. Respects per-wh capacity (fills to planned).
    made = 0
    si = 0
    for wh in wh_plan:
        for _ in range(wh["planned_containers"]):
            sid = ship_ids[si % len(ship_ids)]; si += 1
            cbody = {
                "shipment_id": sid,
                "container_number": cnum(line, made),
                "seal_number": f"SEAL{random.randint(100000,999999)}",
                "warehouse_name": wh["warehouse_name"],
                "loaded_date": d(random.randint(-5, 3)),
            }
            st, _ = api("POST", "/api/containers", cbody, TOKEN)
            if ok(st): made += 1

    # advance each shipment through full workflow
    telex_ok = 0
    for sid in ship_ids:
        val = random.randint(20000, 200000)
        prepay = round(val * 0.3, 2)
        remain = round(val - prepay, 2)
        # -> DOCUMENTS_READY
        api("PATCH", f"/api/shipments/{sid}", {
            "shipper_name": SHIPPER, "consignee_name": buyer,
            "etd": d(random.randint(3,20)),
            "invoice_number": f"INV-{sid}-{random.randint(100,999)}",
            "invoice_date": d(-random.randint(1,10)),
            "total_value_usd": str(val),
            "drafts_date": d(-random.randint(1,5)),
            "status": "DOCUMENTS_READY",
        }, TOKEN)
        # -> CUSTOMS_CLEARED
        api("PATCH", f"/api/shipments/{sid}", {
            "customs_date": d(-random.randint(0,4)),
            "customs_number": f"CUS-{random.randint(10000,99999)}",
            "customs_status": "CLEARED",
            "bill_of_lading": f"BL-{sid}-{random.randint(1000,9999)}",
            "status": "CUSTOMS_CLEARED",
        }, TOKEN)
        # -> CHECKLIST_IN_PROGRESS (+ payment/originals)
        api("PATCH", f"/api/shipments/{sid}", {
            "bl_received": True, "charges_paid": True, "co_received": True,
            "phyto_received": True, "docs_confirmed": True,
            "prepayment_date": d(-random.randint(2,8)),
            "prepayment_amt": str(prepay), "remaining_amt": str(remain),
            "payment_received": True,
            "originals_status": "SENT",
            "originals_sent": d(-random.randint(0,3)),
            "originals_description": "3/3 original BL + CO + Phyto via DHL",
            "containers_loaded": True,
            "status": "CHECKLIST_IN_PROGRESS",
        }, TOKEN)
        # -> TELEX_RELEASED (bool auto-sets status)
        st, sh = api("PATCH", f"/api/shipments/{sid}", {"telex_released": True}, TOKEN)
        if ok(st) and sh.get("status") == "TELEX_RELEASED":
            telex_ok += 1

    # finish the shipping call -> CLOSED
    st, _ = api("PATCH", f"/api/shipping-calls/{cid}", {
        "buyer_name": buyer, "incoterms": inco, "total_containers": total,
        "status": "CLOSED",
    }, TOKEN)
    closed = ok(st)

    call_reports.append({
        "idx": ci, "ref": cref, "id": cid, "buyer": buyer, "product": product,
        "total": total, "warehouses": wh_plan, "shipments": len(ship_ids),
        "containers_made": made, "telex_ok": telex_ok, "closed": closed,
        "status": "OK",
    })
    print(f"call {ci}/10 {cref}: {len(ship_ids)} ship, {made}/{total} ctn, "
          f"telex {telex_ok}/{len(ship_ids)}, closed={closed}")

# ---- report ----
passed = sum(1 for r in results if r[0])
fail = [r for r in results if not r[0]]
ts = dt.datetime.now().strftime("%Y%m%d-%H%M")
outdir = os.path.join(os.path.dirname(__file__), "..", ".hermes")
os.makedirs(outdir, exist_ok=True)
path = os.path.abspath(os.path.join(outdir, f"e2e-test-report-{ts}.md"))

L = []
L.append(f"# E2E Test Report — {dt.datetime.now():%Y-%m-%d %H:%M}\n")
L.append(f"Target: {BASE}  |  10 shipping calls, full workflow → Telex release → close\n")
L.append("## Summary\n")
L.append(f"- Total API calls: **{len(results)}**")
L.append(f"- Passed: **{passed}**")
L.append(f"- Failed: **{len(fail)}**")
calls_ok = sum(1 for c in call_reports if c.get("status")=="OK")
telex_total = sum(c.get("telex_ok",0) for c in call_reports)
ship_total = sum(c.get("shipments",0) for c in call_reports)
ctn_made = sum(c.get("containers_made",0) for c in call_reports)
ctn_plan = sum(c.get("total",0) for c in call_reports)
closed_ok = sum(1 for c in call_reports if c.get("closed"))
L.append(f"- Calls fully processed: **{calls_ok}/10**")
L.append(f"- Shipments created: **{ship_total}**, reached TELEX_RELEASED: **{telex_total}**")
L.append(f"- Containers loaded: **{ctn_made}/{ctn_plan}**")
L.append(f"- Calls CLOSED: **{closed_ok}/10**\n")

L.append("## Per-Call Breakdown\n")
L.append("| # | Ref | Buyer | Product | Ctn | WH split | Ships | Loaded | Telex | Closed |")
L.append("|---|-----|-------|---------|-----|----------|-------|--------|-------|--------|")
for c in call_reports:
    if c.get("status") != "OK":
        L.append(f"| {c['idx']} | {c.get('ref','-')} | — | — | {c.get('total','?')} "
                 f"| — | — | — | — | **{c['status']}** |")
        continue
    whs = ", ".join(f"{w['warehouse_name'].replace('WH-','')}:{w['planned_containers']}"
                    for w in c["warehouses"])
    L.append(f"| {c['idx']} | {c['ref']} | {c['buyer']} | {c['product']} | {c['total']} "
             f"| {whs} | {c['shipments']} | {c['containers_made']} | "
             f"{c['telex_ok']}/{c['shipments']} | {'✓' if c['closed'] else '✗'} |")

L.append("\n## Failures / Anomalies\n")
if not fail:
    L.append("None — all API calls returned success.\n")
else:
    from collections import Counter
    by_path = Counter((f[1], f[2].split("?")[0], f[3]) for f in fail)
    L.append("### Grouped\n")
    L.append("| Method | Path | HTTP | Count |")
    L.append("|--------|------|------|-------|")
    for (m,p,code),n in by_path.most_common():
        L.append(f"| {m} | {p} | {code} | {n} |")
    L.append("\n### Sample details (first 25)\n")
    for f in fail[:25]:
        L.append(f"- `{f[1]} {f[2]}` → **{f[3]}** — {f[4]}")

L.append("\n## Verdict\n")
if not fail and calls_ok == 10 and telex_total == ship_total and closed_ok == 10:
    L.append("PASS — full business process completed end-to-end for all 10 calls.")
else:
    L.append("ISSUES FOUND — see failures above. Fix items then re-run this script.")

open(path, "w").write("\n".join(L))
print(f"\n{'='*50}")
print(f"API calls: {len(results)} | passed: {passed} | failed: {len(fail)}")
print(f"Calls OK: {calls_ok}/10 | shipments telex: {telex_total}/{ship_total} | closed: {closed_ok}/10")
print(f"Report: {path}")
