<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useShipmentsStore } from '@/stores/shipments';
import type { Shipment } from '@/api/client';
import { fmtDateDisplay, fmtCurrency } from '@/utils/format';
import StatusBadge from '@/components/shared/StatusBadge.vue';

const route = useRoute();
const router = useRouter();
const store = useShipmentsStore();

const id = computed(() => Number(route.params.id));

onMounted(async () => {
  await store.loadAll();
  store.select(id.value);
});

function fmtDate(v: string | null): string {
  return fmtDateDisplay(v);
}

function fmtBool(v: boolean): string {
  return v ? '✓ Yes' : '✗ No';
}

function fmtVal(v: string | null): string {
  return fmtCurrency(v);
}

function back() {
  router.push('/workflow');
}
</script>

<template>
  <div class="detail-view">
    <button class="back-btn" @click="back">← Back to Workflow</button>

    <div v-if="store.loading" class="loading">Loading...</div>

    <template v-else-if="store.selected">
      <div class="detail-card">
        <!-- Header -->
        <div class="detail-header">
          <div class="header-left">
            <h1>{{ store.selected.shipment_ref }}</h1>
            <StatusBadge :label="store.selected.status" size="md" />
          </div>
          <div class="header-right">
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

.back-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-xs);
  background: none;
  border: none;
  color: var(--color-manager);
  font-size: var(--text-sm);
  font-weight: 600;
  cursor: pointer;
  padding: var(--space-xs) 0;
  margin-bottom: var(--space-lg);
}
.back-btn:hover { text-decoration: underline; }

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
