<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import { useShipmentsStore } from '@/stores/shipments';
import { useAuthStore } from '@/stores/auth';
import * as api from '@/api/client';
import type { Shipment } from '@/api/client';
import StatCard from '@/components/dashboard/StatCard.vue';
import StatusBadge from '@/components/shared/StatusBadge.vue';
import { fmtDateDisplay } from '@/utils/format';

const store = useShipmentsStore();
const auth = useAuthStore();
const router = useRouter();

const statusFilter = ref('NO_TELEX'); // ponytail: default view = exclude Telex Released
// ponytail: full list drives stats + client-side filter. Server-side status
// refetch corrupted counts (it replaced the list). pageSize 1000; paginate if
// shipments ever exceed that.
const allShips = ref<Shipment[]>([]);

async function loadShips() {
  const r = await api.fetchShipments({ pageSize: 1000 });
  allShips.value = r.data;
}
onMounted(loadShips);

const statusStats = computed(() => {
  const m: Record<string, number> = {};
  for (const s of allShips.value) {
    m[s.status] = (m[s.status] || 0) + 1;
  }
  return m;
});

const statCards = [
  { label: 'Total', key: 'total', color: 'var(--color-manager)' },
  { label: 'Draft', key: 'DRAFT', color: 'var(--color-draft)' },
  { label: 'Documents', key: 'DOCUMENTS_READY', color: 'var(--color-documents)' },
  { label: 'Customs', key: 'CUSTOMS_CLEARED', color: 'var(--color-customs)' },
  { label: 'Checklist', key: 'CHECKLIST_IN_PROGRESS', color: 'var(--color-checklist)' },
  { label: 'Telex', key: 'TELEX_RELEASED', color: 'var(--color-telex)' },
];

const filtered = computed(() => {
  if (!statusFilter.value) return allShips.value;
  if (statusFilter.value === 'NO_TELEX')
    return allShips.value.filter(s => s.status !== 'TELEX_RELEASED');
  return allShips.value.filter(s => s.status === statusFilter.value);
});

function handleFilterChange(status: string) {
  statusFilter.value = status;
}

async function handleNew() {
  const s = await store.create({});
  await loadShips();
  if (s) router.push(`/workflow/${s.id}`);
}

async function handleDelete(id: number, ref: string) {
  if (!confirm(`Delete ${ref}? This cannot be undone.`)) return;
  await store.remove(id);
  await loadShips();
}
</script>

<template>
  <div class="shipments-page">
    <div class="page-header">
      <h1>Shipments</h1>
      <button class="btn-new" @click="handleNew" :disabled="store.loading">+ New Shipment</button>
    </div>

    <!-- Stats row -->
    <div class="stats-grid">
      <StatCard
        :label="'Total'"
        :value="allShips.length"
        :color="'var(--color-manager)'"
        :active="statusFilter === ''"
        @click="handleFilterChange('')"
      />
      <StatCard
        v-for="card in statCards.slice(1)"
        :key="card.key"
        :label="card.label"
        :value="statusStats[card.key] || 0"
        :color="card.color"
        :active="statusFilter === card.key"
        @click="handleFilterChange(card.key)"
      />
    </div>

    <!-- Filter + table -->
    <div class="table-header">
      <select
        class="status-filter"
        :value="statusFilter"
        @change="handleFilterChange(($event.target as HTMLSelectElement).value)"
      >
        <option value="">All Statuses</option>
        <option value="NO_TELEX">No Telex Released</option>
        <option value="DRAFT">Draft</option>
        <option value="DOCUMENTS_READY">Documents Ready</option>
        <option value="CUSTOMS_CLEARED">Customs Cleared</option>
        <option value="CHECKLIST_IN_PROGRESS">Checklist</option>
        <option value="TELEX_RELEASED">Telex Released</option>
      </select>
      <span class="table-count">{{ filtered.length }} shipments</span>
    </div>

    <div v-if="store.loading && !allShips.length" class="loading">Loading...</div>

    <table v-else-if="filtered.length" class="shipment-table">
      <thead>
        <tr>
          <th>Booking #</th>
          <th>ETD</th>
          <th>Buyer</th>
          <th>BL #</th>
          <th>All LOADED</th>
          <th>Status</th>
          <th>Ref</th>
          <th>Call</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="s in filtered" :key="s.id" @click="router.push(`/shipment/${s.id}`)" class="table-row">
          <td class="mono">{{ s.booking_number ?? '—' }}</td>
          <td class="date">{{ s.etd ? fmtDateDisplay(s.etd) : '—' }}</td>
          <td>{{ s.buyer_name ?? '—' }}</td>
          <td class="mono">{{ s.bill_of_lading ?? '—' }}</td>
          <td class="loaded">{{ s.containers_loaded ? '✓' : '—' }}</td>
          <td><StatusBadge :label="s.status" size="sm" /></td>
          <td class="mono">{{ s.shipment_ref }}</td>
          <td class="mono muted">{{ s.shipping_call_id ? `CALL-${s.shipping_call_id}` : '—' }}</td>
          <td>
            <button v-if="auth.isAdmin" class="btn-del" @click.stop="handleDelete(s.id, s.shipment_ref)">×</button>
          </td>
        </tr>
      </tbody>
    </table>

    <div v-else class="empty">No shipments match the filter.</div>
  </div>
</template>

<style scoped>
.shipments-page {
  max-width: 1200px;
  margin: 0 auto;
  padding: var(--space-lg);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-xl);
}
.page-header h1 { font-size: var(--text-2xl); font-weight: 700; }

.btn-new {
  background: var(--color-manager);
  color: var(--text-inverse);
  border: none;
  padding: var(--space-sm) var(--space-lg);
  border-radius: var(--radius-sm);
  font-weight: 600;
  cursor: pointer;
}
.btn-new:hover { opacity: 0.9; }

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: var(--space-md);
  margin-bottom: var(--space-xl);
}

.table-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  margin-bottom: var(--space-md);
}
.status-filter {
  padding: var(--space-xs) var(--space-sm);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-sm);
  background: var(--bg-card);
  color: var(--text-primary);
  font-size: var(--text-sm);
}
.table-count { font-size: var(--text-xs); color: var(--text-secondary); }

.shipment-table {
  width: 100%;
  border-collapse: collapse;
  background: var(--bg-card);
  border-radius: var(--radius-md);
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}
.shipment-table th {
  text-align: left;
  padding: var(--space-sm) var(--space-md);
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  border-bottom: 2px solid var(--border-light);
}
.table-row {
  cursor: pointer;
  transition: background 0.1s;
}
.table-row:hover { background: var(--bg-sidebar-hover); }
.table-row td {
  padding: var(--space-sm) var(--space-md);
  font-size: var(--text-sm);
  border-bottom: 1px solid var(--border-light);
}
.mono { font-family: var(--font-mono); font-size: var(--text-xs); }
.date { white-space: nowrap; color: var(--text-secondary); }
.muted { color: var(--text-secondary); }

.btn-del {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 1.2rem;
  cursor: pointer;
  padding: 0 4px;
}
.btn-del:hover { color: var(--color-admin); }

.loading, .empty {
  text-align: center;
  padding: var(--space-2xl);
  color: var(--text-secondary);
}
</style>
