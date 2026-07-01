<script setup lang="ts">
import { onMounted, computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useShippingCallsStore } from '@/stores/shipping-calls';
import { useShipmentsStore } from '@/stores/shipments';

const router = useRouter();
const store = useShippingCallsStore();
const shipStore = useShipmentsStore();

onMounted(() => { store.loadAll(); shipStore.loadAll(); });

import { fmtDateDisplay } from '@/utils/format';
function fmtDate(iso: string): string {
  return fmtDateDisplay(iso);
}

const incotermsMap: Record<string, string> = {
  FOB: 'FOB', CIF: 'CIF', CFR: 'CFR', EXW: 'EXW',
  FCA: 'FCA', FAS: 'FAS', CPT: 'CPT', CIP: 'CIP',
  DAP: 'DAP', DPU: 'DPU', DDP: 'DDP',
};

// ponytail: group calls by status for operator-friendly ordering
const groups = computed(() => [
  { status: 'OPEN', label: 'Open — Needs Bookings', color: 'var(--color-manager)', calls: store.calls.filter(c => c.status === 'OPEN') },
  { status: 'ON_LOADING', label: 'On Loading — In Progress', color: 'var(--color-checklist)', calls: store.calls.filter(c => c.status === 'ON_LOADING') },
  { status: 'CLOSED', label: 'Closed', color: '#7f8c8d', calls: store.calls.filter(c => c.status === 'CLOSED') },
].filter(g => g.calls.length > 0));

// ponytail: map call_id → linked shipment refs
const shipmentsByCall = computed(() => {
  const m: Record<number, typeof shipStore.shipments> = {};
  for (const s of shipStore.shipments) {
    if (s.shipping_call_id) (m[s.shipping_call_id] ??= []).push(s);
  }
  return m;
});

// ponytail: collapsible status groups — click header to toggle; Closed starts collapsed
const collapsed = ref<Record<string, boolean>>({ CLOSED: true });
function toggleGroup(status: string) {
  collapsed.value[status] = !collapsed.value[status];
}
</script>

<template>
  <div class="calls-page">
    <div class="page-header">
      <h1>Shipping Calls</h1>
      <button class="btn-new" @click="router.push('/calls/new')">+ New Call</button>
    </div>

    <div v-if="store.loading" class="loading">Loading...</div>

    <div v-else-if="!store.calls.length" class="empty">
      <p>No shipping calls yet.</p>
      <button class="btn-new" @click="router.push('/calls/new')">Create your first shipping call</button>
    </div>

    <div v-else>
      <div v-for="g in groups" :key="g.status" class="status-group">
        <div class="group-header" :style="{ borderLeftColor: g.color }"
          @click="toggleGroup(g.status)" role="button" tabindex="0"
          @keydown.enter="toggleGroup(g.status)">
          <span class="group-chevron" :class="{ collapsed: collapsed[g.status] }">▾</span>
          <span class="group-dot" :style="{ background: g.color }"></span>
          <span class="group-label">{{ g.label }}</span>
          <span class="group-count">{{ g.calls.length }}</span>
        </div>
        <div class="card-grid" v-show="!collapsed[g.status]">
          <div v-for="c in g.calls" :key="c.id" class="call-card">
            <div class="card-header" @click="router.push(`/calls/${c.id}`)">
              <span class="call-ref">{{ c.call_ref }}</span>
              <span class="status-badge" :class="c.status.toLowerCase()">{{ c.status }}</span>
            </div>
            <div class="card-body" @click="router.push(`/calls/${c.id}`)">
              <div class="buyer-row">
                <strong>{{ c.buyer_name }}</strong>
                <!-- ponytail: linked bookings beside buyer -->
                <span v-if="shipmentsByCall[c.id]?.length" class="bookings-inline">
                  <span v-for="s in shipmentsByCall[c.id]" :key="s.id" class="booking-ref"
                    @click.stop="router.push(`/shipment/${s.id}`)">{{ s.booking_number ?? s.shipment_ref }}</span>
                </span>
              </div>
              <div class="meta">{{ incotermsMap[c.incoterms] ?? c.incoterms }} · {{ c.total_containers }} containers</div>
              <div class="meta" v-if="c.product_description">{{ c.product_description }}</div>
            </div>
            <div class="card-footer">
              <span class="date">{{ fmtDate(c.created_at) }}</span>
              <button class="btn-open" @click="router.push(`/calls/${c.id}`)">Edit S/C</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.calls-page { padding: var(--space-xl); max-width: 1100px; }
.page-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-xl); }
.page-header h1 { font-size: var(--text-2xl); font-weight: 700; }
.btn-new { background: var(--color-manager); color: var(--text-inverse); border: none; padding: var(--space-sm) var(--space-lg); border-radius: var(--radius-sm); font-weight: 600; cursor: pointer; }
.btn-new:hover { opacity: 0.9; }
.loading, .empty { text-align: center; padding: var(--space-2xl); color: var(--text-secondary); }
.empty p { margin-bottom: var(--space-md); }

/* Status grouping */
.status-group { margin-bottom: var(--space-xl); }
.group-header {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md);
  border-left: 3px solid;
  margin-bottom: var(--space-md);
  background: var(--bg-card);
  border-radius: var(--radius-sm);
  cursor: pointer;
  user-select: none;
}
.group-header:hover { background: var(--bg-sidebar-hover); }
.group-chevron { font-size: var(--text-sm); transition: transform 0.15s; flex-shrink: 0; }
.group-chevron.collapsed { transform: rotate(-90deg); }
.group-dot {
  width: 10px; height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}
.group-label {
  font-size: var(--text-sm);
  font-weight: 700;
  flex: 1;
}
.group-count {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  font-weight: 600;
}

.card-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: var(--space-md); }
.call-card { background: var(--bg-card); border-radius: var(--radius-md); box-shadow: var(--shadow-sm); padding: var(--space-md); display: flex; flex-direction: column; }
.card-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--space-sm); cursor: pointer; }
.call-ref { font-family: var(--font-mono); font-size: var(--text-base); font-weight: 700; }
.status-badge { padding: 2px 10px; border-radius: var(--radius-sm); font-size: var(--text-xs); font-weight: 700; text-transform: uppercase; }
.status-badge.open { background: rgba(52,152,219,0.15); color: var(--color-manager); }
.status-badge.on_loading { background: rgba(39,174,96,0.15); color: var(--color-checklist); }
.status-badge.partial { background: rgba(243,156,18,0.15); color: var(--color-accounting); }
.status-badge.complete { background: rgba(39,174,96,0.15); color: var(--color-checklist); }
.status-badge.closed { background: rgba(149,165,166,0.15); color: #7f8c8d; }
.card-body { margin-bottom: var(--space-sm); font-size: var(--text-sm); color: var(--text-primary); cursor: pointer; flex: 1; }
.meta { color: var(--text-secondary); font-size: var(--text-xs); margin-top: 2px; }
.card-footer { display: flex; justify-content: space-between; align-items: center; border-top: 1px solid var(--border-light); padding-top: var(--space-xs); }
.buyer-row { display: flex; align-items: center; gap: var(--space-sm); }
.bookings-inline { display: flex; gap: 4px; margin-left: auto; }
.booking-ref {
  padding: 1px 6px;
  border-radius: var(--radius-sm);
  font-size: 10px;
  font-family: var(--font-mono);
  color: var(--color-manager);
  cursor: pointer;
}
.booking-ref:hover { background: var(--color-manager); color: var(--text-inverse); }
.date { font-size: var(--text-xs); color: var(--text-secondary); }
.btn-open { background: var(--color-manager); color: var(--text-inverse); border: none; padding: 4px var(--space-md); border-radius: var(--radius-sm); font-size: var(--text-xs); font-weight: 600; cursor: pointer; }
.btn-open:hover { opacity: 0.9; }
</style>
