<script setup lang="ts">
import { onMounted, ref, computed, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useShipmentsStore } from '@/stores/shipments';
import type { Shipment } from '@/api/client';
import * as api from '@/api/client';
import { fmtDateDisplay, fmtCurrency } from '@/utils/format';
import StatusBadge from '@/components/shared/StatusBadge.vue';

const route = useRoute();
const store = useShipmentsStore();

const id = computed(() => Number(route.params.id));
const containerCount = ref(0);
const callTotal = ref(0);
const callRef = ref('');

onMounted(async () => {
  await store.loadAll();
  await ensureSelected();
  await loadExtras();
});

// ponytail: re-trigger on route change (same component)
watch(id, async () => {
  await ensureSelected();
  await loadExtras();
});

// ponytail: the paginated list may not hold this shipment (older non-telex ones
// fall past page 1). Fetch it directly and add to the store so `selected` resolves.
async function ensureSelected() {
  if (!store.shipments.find((s) => s.id === id.value)) {
    try { store.shipments.push(await api.fetchShipment(id.value)); } catch { /* not found */ }
  }
  store.select(id.value);
}

async function loadExtras() {
  try { containerCount.value = (await api.fetchContainers(id.value)).length; } catch { containerCount.value = 0; }
  const callId = store.selected?.shipping_call_id;
  if (callId) {
    try {
      const c = await api.fetchShippingCall(callId);
      callTotal.value = c.total_containers;
      callRef.value = c.call_ref;
    } catch { callTotal.value = 0; }
  }
}

function fmtDate(v: string | null): string {
  return fmtDateDisplay(v);
}

function fmtBool(v: boolean): string {
  return v ? '✓ Yes' : '✗ No';
}

function fmtVal(v: string | null): string {
  return fmtCurrency(v);
}
</script>

<template>
  <div class="detail-view">
    <div v-if="store.loading" class="loading">Loading...</div>

    <template v-else-if="store.selected">
      <div class="detail-card">
        <!-- Header -->
        <div class="detail-header">
          <div class="header-left">
            <h1>{{ store.selected.shipment_ref }}</h1>
            <StatusBadge :label="store.selected.status" size="md" />
            <router-link :to="`/workflow/${store.selected.id}`" class="edit-btn">✎ Edit</router-link>
          </div>
          <div class="header-right">
            <span v-if="callRef" class="meta">
              🚢
              <router-link :to="`/calls/${store.selected?.shipping_call_id}`" class="call-link">
                {{ callRef }}
              </router-link>
            </span>
            <span class="meta">Created: {{ fmtDate(store.selected.created_at) }}</span>
            <span class="meta">Updated: {{ fmtDate(store.selected.updated_at) }}</span>
          </div>
        </div>

        <!-- Phase 1: CREATE (ADMIN) -->
        <section class="phase" style="border-left-color: var(--color-admin)">
          <h2 class="phase-title" style="color: var(--color-admin)">
            <span class="phase-num">1</span> CREATE SHIPMENT — ADMIN
          </h2>
          <div class="field-grid">
            <div class="field"><label>SC/PO ID</label><span class="val">{{ store.selected.sc_po_id ?? '—' }}</span></div>
            <div class="field"><label>SC/PO Date</label><span class="val">{{ fmtDate(store.selected.sc_po_date) }}</span></div>
            <div class="field"><label>Made By</label><span class="val">{{ store.selected.sc_po_by ?? '—' }}</span></div>
            <div class="field"><label>Buyer</label><span class="val">{{ store.selected.buyer_name ?? '—' }}</span></div>
            <div class="field"><label>Booking #</label><span class="val mono">{{ store.selected.booking_number ?? '—' }}</span></div>
            <div class="field"><label>Shipping Line</label><span class="val">{{ store.selected.shipping_line ?? '—' }}</span></div>
            <div class="field"><label>Port of Loading</label><span class="val">{{ store.selected.origin_port ?? '—' }}</span></div>
            <div class="field"><label>Warehouse</label><span class="val">{{ store.selected.warehouse_loc ?? '—' }}</span></div>
            <div class="field full"><label>Loading Plan</label><span class="val pre">{{ store.selected.loading_plan || '—' }}</span></div>
          </div>
        </section>

        <!-- Phase 2: DRAFT (MANAGER) -->
        <section class="phase" style="border-left-color: var(--color-manager)">
          <h2 class="phase-title" style="color: var(--color-manager)">
            <span class="phase-num">2</span> DRAFT DOCUMENTATION — MANAGER
          </h2>
          <div class="field-grid">
            <div class="field"><label>Shipper</label><span class="val">{{ store.selected.shipper_name ?? '—' }}</span></div>
            <div class="field"><label>Consignee</label><span class="val">{{ store.selected.consignee_name ?? '—' }}</span></div>
            <div class="field"><label>ETD</label><span class="val">{{ fmtDate(store.selected.etd) }}</span></div>
            <div class="field"><label>Invoice #</label><span class="val mono">{{ store.selected.invoice_number ?? '—' }}</span></div>
            <div class="field"><label>Invoice Date</label><span class="val">{{ fmtDate(store.selected.invoice_date) }}</span></div>
            <div class="field"><label>Total Value (USD)</label><span class="val num">{{ fmtVal(store.selected.total_value_usd) }}</span></div>
            <div class="field"><label>Drafts Date</label><span class="val">{{ fmtDate(store.selected.drafts_date) }}</span></div>
            <div class="field"><label>Bill of Lading #</label><span class="val mono">{{ store.selected.bill_of_lading ?? '—' }}</span></div>
          </div>
        </section>

        <!-- Phase 3: CUSTOMS (ACCOUNTING) -->
        <section class="phase" style="border-left-color: var(--color-accounting)">
          <h2 class="phase-title" style="color: var(--color-accounting)">
            <span class="phase-num">3</span> CUSTOMS CLEARANCE — ACCOUNTING
          </h2>
          <div class="field-grid">
            <div class="field"><label>Customs Date</label><span class="val">{{ fmtDate(store.selected.customs_date) }}</span></div>
            <div class="field"><label>Customs #</label><span class="val mono">{{ store.selected.customs_number ?? '—' }}</span></div>
            <div class="field"><label>Customs Status</label>
              <span class="val">
                <span v-if="store.selected.customs_status" class="customs-badge" :class="store.selected.customs_status">
                  {{ store.selected.customs_status === 'red' ? '🔴 Red' : store.selected.customs_status === 'yellow' ? '🟡 Yellow' : '🟢 Green' }}
                </span>
                <span v-else>—</span>
              </span>
            </div>
          </div>
          <!-- Containers -->
          <div class="check-group">
            <h3 style="color: var(--color-manager)">CONTAINERS LOADED</h3>
            <span class="val">{{ containerCount }} / {{ callTotal }} loaded
              <span v-if="store.selected?.containers_loaded" style="color: var(--color-checklist); font-weight: 700;">✓</span>
            </span>
          </div>
        </section>

        <!-- Phase 4: CHECKLIST -->
        <section class="phase" style="border-left-color: var(--color-logistics)">
          <h2 class="phase-title" style="color: var(--color-logistics)">
            <span class="phase-num">4</span> DOCUMENT CHECKLIST — ALL ROLES
          </h2>

          <!-- Documents -->
          <div class="check-group">
            <h3 style="color: var(--color-manager)">DOCUMENTS</h3>
            <div class="check-row">
              <span class="check-item" :class="{ done: store.selected.bl_received }">BL Received: {{ fmtBool(store.selected.bl_received) }}</span>
              <span class="check-item" :class="{ done: store.selected.docs_confirmed }">Docs Confirmed: {{ fmtBool(store.selected.docs_confirmed) }}</span>
            </div>
          </div>

          <!-- Charges -->
          <div class="check-group">
            <h3 style="color: var(--color-accounting)">CHARGES</h3>
            <div class="check-row">
              <span class="check-item" :class="{ done: store.selected.charges_paid }">Charges/THC Paid: {{ fmtBool(store.selected.charges_paid) }}</span>
            </div>
          </div>

          <!-- Certificates -->
          <div class="check-group">
            <h3 style="color: var(--color-logistics)">CERTIFICATES</h3>
            <div class="check-row">
              <span class="check-item" :class="{ done: store.selected.co_received }">CO Received: {{ fmtBool(store.selected.co_received) }}</span>
              <span class="check-item" :class="{ done: store.selected.phyto_received }">Phyto Received: {{ fmtBool(store.selected.phyto_received) }}</span>
            </div>
          </div>

          <!-- Payment -->
          <div class="check-group">
            <h3 style="color: var(--color-accounting)">PAYMENT</h3>
            <div class="check-row">
              <span class="check-item" :class="{ done: store.selected.payment_received }">Payment Received: {{ fmtBool(store.selected.payment_received) }}</span>
            </div>
            <div class="field-grid cols-3" style="margin-top: var(--space-sm)">
              <div class="field"><label>Payment Date</label><span class="val">{{ fmtDate(store.selected.prepayment_date) }}</span></div>
              <div class="field"><label>Prepayment</label><span class="val num">{{ fmtVal(store.selected.prepayment_amt) }}</span></div>
              <div class="field"><label>Remaining</label><span class="val num">{{ fmtVal(store.selected.remaining_amt) }}</span></div>
            </div>
          </div>

          <!-- Originals -->
          <div class="check-group">
            <h3 style="color: var(--color-manager)">ORIGINALS</h3>
            <div class="field-grid">
              <div class="field"><label>Status</label><span class="val">{{ store.selected.originals_status ?? '—' }}</span></div>
              <div class="field"><label>Date Sent</label><span class="val">{{ fmtDate(store.selected.originals_sent) }}</span></div>
            </div>
            <div v-if="store.selected.originals_description" class="field" style="margin-top: var(--space-sm)">
              <label>Contact & Sending</label><span class="val">{{ store.selected.originals_description }}</span>
            </div>
          </div>

          <!-- Telex -->
          <div class="check-group telex-group">
            <h3 style="color: var(--color-telex)">TELEX RELEASE</h3>
            <span class="check-item telex-item" :class="{ done: store.selected.telex_released }">
              {{ store.selected.telex_released ? '✓ TELEX RELEASED — Shipment Complete' : 'Pending' }}
            </span>
          </div>
        </section>
      </div>
    </template>

    <div v-else class="loading">Shipment not found.</div>
  </div>
</template>

<style scoped>
.detail-view {
  padding: var(--space-xl);
  max-width: 900px;
  margin: 0 auto;
}

.edit-btn {
  background: var(--color-manager);
  color: var(--text-inverse);
  text-decoration: none;
  font-size: var(--text-sm);
  font-weight: 600;
  padding: var(--space-xs) var(--space-md);
  border-radius: var(--radius-sm);
}
.edit-btn:hover { opacity: 0.9; }

.loading {
  text-align: center;
  padding: var(--space-2xl);
  color: var(--text-secondary);
}

.detail-card {
  background: var(--bg-card);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  overflow: hidden;
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: var(--space-lg);
  background: var(--bg-surface);
  border-bottom: 2px solid var(--border-color);
}
.header-left {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}
.header-left h1 {
  font-size: var(--text-2xl);
  font-weight: 700;
  font-family: var(--font-mono);
}
.header-right {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
}
.meta {
  font-size: var(--text-xs);
  color: var(--text-secondary);
}
.call-link {
  color: var(--color-manager);
  font-weight: 600;
  text-decoration: none;
}
.call-link:hover { text-decoration: underline; }

/* Phase sections */
.phase {
  padding: var(--space-lg);
  border-left: 4px solid;
  border-bottom: 1px solid var(--border-light);
}
.phase:last-child { border-bottom: none; }

.phase-title {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  font-size: var(--text-base);
  font-weight: 700;
  margin-bottom: var(--space-md);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.phase-num {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: currentColor;
  color: var(--text-inverse);
  font-size: var(--text-xs);
  font-weight: 700;
}

.field-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-md);
}
.field-grid.cols-3 {
  grid-template-columns: 1fr 1fr 1fr;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.field.full { grid-column: 1 / -1; }
.field label {
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.val {
  font-size: var(--text-base);
  color: var(--text-primary);
}
.val.mono {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
}
.val.num {
  font-family: var(--font-mono);
  font-weight: 600;
}
.val.pre {
  white-space: pre-wrap;
  font-size: var(--text-sm);
}

/* Checklist groups */
.check-group {
  margin-bottom: var(--space-md);
  padding: var(--space-sm) 0;
}
.check-group h3 {
  font-size: var(--text-xs);
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: var(--space-xs);
}
.check-row {
  display: flex;
  gap: var(--space-xl);
  flex-wrap: wrap;
}
.check-item {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  font-weight: 500;
}
.check-item.done {
  color: var(--color-checklist);
  font-weight: 600;
}

.customs-badge {
  display: inline-block;
  font-size: var(--text-sm);
  font-weight: 600;
  padding: 1px 8px;
  border-radius: var(--radius-sm);
}
.customs-badge.red { background: rgba(231, 76, 60, 0.1); color: var(--color-admin); }
.customs-badge.yellow { background: rgba(243, 156, 18, 0.1); color: var(--color-accounting); }
.customs-badge.green { background: rgba(39, 174, 96, 0.1); color: var(--color-checklist); }

.telex-group {
  padding-top: var(--space-sm);
  border-top: 1px dashed var(--border-color);
}
.telex-item {
  font-size: var(--text-lg);
  font-weight: 700;
  color: var(--text-secondary);
}
.telex-item.done {
  color: var(--color-telex);
}

@media (max-width: 768px) {
  .detail-view { padding: var(--space-md); }
  .detail-header { flex-direction: column; gap: var(--space-sm); }
  .header-right { align-items: flex-start; }
  .field-grid { grid-template-columns: 1fr; }
  .field-grid.cols-3 { grid-template-columns: 1fr; }
}
</style>
