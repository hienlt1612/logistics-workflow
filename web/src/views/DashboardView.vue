<script setup lang="ts">
import { onMounted, onUnmounted, computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useDashboardStore } from '@/stores/dashboard';
import { useShippingCallsStore } from '@/stores/shipping-calls';
import * as api from '@/api/client';
import type { Shipment } from '@/api/client';
import StatCard from '@/components/dashboard/StatCard.vue';
import StatusBadge from '@/components/shared/StatusBadge.vue';
import { fmtDateDisplay } from '@/utils/format';

const dash = useDashboardStore();
const callStore = useShippingCallsStore();
const router = useRouter();
// ponytail: load all shipments (like calls) so the dashboard table + filters
// match the full card counts. pageSize 1000; paginate if it ever exceeds that.
const allShips = ref<Shipment[]>([]);

const shipCards = [
  { label: 'Draft', key: 'draft' as const, color: 'var(--color-draft)' },
  { label: 'Documents', key: 'documents' as const, color: 'var(--color-documents)' },
  { label: 'Customs', key: 'customs' as const, color: 'var(--color-customs)' },
  { label: 'Checklist', key: 'checklist' as const, color: 'var(--color-checklist)' },
  { label: 'Telex', key: 'telex' as const, color: 'var(--color-telex)' },
];

const callCards = [
  { label: 'Open', key: 'calls_open' as const, color: 'var(--color-manager)' },
  { label: 'On Loading', key: 'calls_loading' as const, color: 'var(--color-checklist)' },
  { label: 'Closed', key: 'calls_closed' as const, color: '#7f8c8d' },
];

const ageText = computed(() => {
  const s = dash.secondsAgo();
  if (s < 5) return 'just now';
  if (s < 60) return `${s}s ago`;
  return `${Math.floor(s / 60)}m ago`;
});

// ponytail: map call_id → linked shipment refs for dashboard call list
const bookingsByCall = computed(() => {
  const m: Record<number, Shipment[]> = {};
  for (const s of allShips.value) {
    if (s.shipping_call_id) (m[s.shipping_call_id] ??= []).push(s);
  }
  return m;
});
function callStatusFromKey(key: string): string {
  const raw = key.replace('calls_', '').toUpperCase();
  return raw === 'LOADING' ? 'ON_LOADING' : raw;
}
// ponytail: map shipment card keys to DB status values
function shipStatusFromKey(key: string): string {
  const m: Record<string, string> = {
    draft: 'DRAFT',
    documents: 'DOCUMENTS_READY',
    customs: 'CUSTOMS_CLEARED',
    checklist: 'CHECKLIST_IN_PROGRESS',
    telex: 'TELEX_RELEASED',
  };
  return m[key] ?? key;
}
const activeCallFilter = ref('ON_LOADING'); // ponytail: default view = On Loading
const activeShipFilter = ref('NO_TELEX'); // ponytail: default view = exclude Telex Released

function toggleCallFilter(status: string) {
  activeCallFilter.value = activeCallFilter.value === status ? '' : status;
}
function toggleShipFilter(status: string) {
  activeShipFilter.value = activeShipFilter.value === status ? '' : status;
}

const filteredCalls = computed(() => {
  if (!activeCallFilter.value) return callStore.calls;
  return callStore.calls.filter(c => c.status === activeCallFilter.value);
});
const filteredShipments = computed(() => {
  if (!activeShipFilter.value) return allShips.value;
  if (activeShipFilter.value === 'NO_TELEX')
    return allShips.value.filter(s => s.status !== 'TELEX_RELEASED');
  return allShips.value.filter(s => s.status === activeShipFilter.value);
});

onMounted(async () => {
  dash.startPolling(30000);
  callStore.loadAll();
  const r = await api.fetchShipments({ pageSize: 1000 });
  allShips.value = r.data;
});

onUnmounted(() => {
  dash.stopPolling();
});
</script>

<template>
  <div class="dashboard">
    <div v-if="dash.loading && !dash.lastUpdated" class="loading">Loading stats...</div>

    <div v-else class="dashboard-grid">
      <!-- Left column: Shipping Calls -->
      <div class="dash-left">
        <section class="stat-section">
          <h2 class="section-label">Shipping Calls · {{ dash.stats.calls_total }} total</h2>
          <div class="stats-grid">
            <StatCard v-for="card in callCards" :key="card.key"
              :label="card.label" :value="dash.stats[card.key]" :color="card.color"
              :active="activeCallFilter === callStatusFromKey(card.key)"
              @click="toggleCallFilter(callStatusFromKey(card.key))" />
          </div>
        </section>
        <!-- Active calls list -->
        <section class="stat-section">
          <h2 class="section-label">Active Calls</h2>
          <div v-if="filteredCalls.length" class="call-mini-list">
            <div v-for="c in filteredCalls" :key="c.id" class="call-mini-row"
              @click="router.push(`/calls/${c.id}`)">
              <span class="call-mini-ref">{{ c.call_ref }}</span>
              <span v-if="bookingsByCall[c.id]" class="call-bookings">
                <span v-for="s in bookingsByCall[c.id]" :key="s.id"
                  class="call-booking-pill"
                  :class="c.status.toLowerCase()"
                  @click.stop="router.push(`/shipment/${s.id}`)">{{ s.booking_number ?? s.shipment_ref }}</span>
              </span>
              <StatusBadge :label="c.status" size="sm" />
              <span class="call-mini-buyer">{{ c.buyer_name }}</span>
            </div>
          </div>
          <div v-else class="empty-mini">No calls</div>
        </section>
      </div>

      <!-- Right column: Shipments -->
      <div class="dash-right">
        <section class="stat-section">
          <h2 class="section-label">Shipments · {{ dash.stats.total }} total</h2>
          <div class="stats-grid">
            <StatCard v-for="card in shipCards" :key="card.key"
              :label="card.label" :value="dash.stats[card.key]" :color="card.color"
              :active="activeShipFilter === shipStatusFromKey(card.key)"
              @click="toggleShipFilter(shipStatusFromKey(card.key))" />
          </div>
        </section>
        <!-- Recent shipments — same columns as /shipments -->
        <section class="stat-section">
          <h2 class="section-label">Recent Shipments</h2>
          <table v-if="filteredShipments.length" class="ship-table">
            <thead><tr>
              <th>Booking #</th><th>ETD</th><th>Buyer</th><th>BL #</th><th>Loaded</th><th>Status</th><th>Ref</th><th>Call</th>
            </tr></thead>
            <tbody>
              <tr v-for="s in filteredShipments" :key="s.id" class="ship-row"
                @click="router.push(`/shipment/${s.id}`)">
                <td class="mono">{{ s.booking_number ?? '—' }}</td>
                <td class="date">{{ s.etd ? fmtDateDisplay(s.etd) : '—' }}</td>
                <td>{{ s.buyer_name ?? '—' }}</td>
                <td class="mono">{{ s.bill_of_lading ?? '—' }}</td>
                <td class="loaded">{{ s.containers_loaded ? '✓' : '—' }}</td>
                <td><StatusBadge :label="s.status" size="sm" /></td>
                <td class="mono">{{ s.shipment_ref }}</td>
                <td class="mono muted">{{ s.shipping_call_id ? `CALL-${s.shipping_call_id}` : '—' }}</td>
              </tr>
            </tbody>
          </table>
          <div v-else class="empty-mini">No shipments</div>
        </section>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  max-width: none;
  margin: 0;
  padding: 0 var(--space-lg) var(--space-lg) var(--space-lg);
}
.loading { text-align: center; padding: var(--space-2xl); color: var(--text-secondary); }

/* 2-column grid */
.dashboard-grid {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: var(--space-lg);
}
@media (max-width: 900px) {
  .dashboard-grid { grid-template-columns: 1fr; }
}

.stat-section { margin-bottom: var(--space-sm); }
.section-label {
  font-size: var(--text-sm); font-weight: 600;
  color: var(--text-secondary); text-transform: uppercase;
  letter-spacing: 0.5px; margin-bottom: var(--space-md);
}
.stats-grid {
  display: flex;
  gap: var(--space-sm);
  justify-content: flex-start;
}

/* Call mini list */
.call-mini-list { display: flex; flex-direction: column; gap: 2px; }
.call-mini-row {
  display: flex; align-items: center; gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md); border-radius: var(--radius-sm);
  cursor: pointer; transition: background 0.1s;
}
.call-mini-row:hover { background: var(--bg-sidebar-hover); }
.call-mini-ref { font-family: var(--font-mono); font-size: var(--text-sm); font-weight: 700; flex: 1; }
.call-bookings { display: flex; gap: 3px; margin-left: auto; margin-right: var(--space-xs); }
.call-booking-pill {
  font-size: 11px;
  font-family: var(--font-mono);
  padding: 0px 5px;
  border-radius: var(--radius-sm);
  cursor: pointer;
}
.call-booking-pill:hover { opacity: 0.8; }
.call-booking-pill.open { background: var(--color-manager); color: #fff; }
.call-booking-pill.on_loading { background: var(--color-checklist); color: #fff; }
.call-booking-pill.closed { background: #7f8c8d; color: #fff; }
.call-mini-buyer { font-size: var(--text-sm); color: var(--text-secondary); }

/* Shipment table */
.ship-table { width: 100%; border-collapse: collapse; font-size: var(--text-sm); }
.ship-table th {
  text-align: left; padding: var(--space-xs) var(--space-sm);
  font-weight: 600; color: var(--text-secondary); text-transform: uppercase;
  border-bottom: 2px solid var(--border-light);
}
.ship-row { cursor: pointer; transition: background 0.1s; }
.ship-row:hover { background: var(--bg-sidebar-hover); }
.ship-row td { padding: var(--space-xs) var(--space-sm); border-bottom: 1px solid var(--border-light); }
.mono { font-family: var(--font-mono); }
.muted { color: var(--text-secondary); }
.empty-mini { text-align: center; padding: var(--space-md); color: var(--text-secondary); font-size: var(--text-xs); }
</style>
