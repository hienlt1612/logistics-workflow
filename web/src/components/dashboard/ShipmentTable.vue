<script setup lang="ts">
import type { Shipment } from '@/api/client';
import StatusBadge from '@/components/shared/StatusBadge.vue';

defineProps<{
  shipments: Shipment[];
}>();

function fmtDate(iso: string): string {
  return new Date(iso).toLocaleDateString();
}

function fmtVal(v: string | null): string {
  if (!v) return '—';
  const n = parseFloat(v);
  return isNaN(n) ? v : `$${n.toLocaleString()}`;
}
</script>

<template>
  <div class="table-wrap">
    <table class="shipment-table">
      <thead>
        <tr>
          <th>Ref</th>
          <th>Buyer</th>
          <th>Status</th>
          <th>Shipping Line</th>
          <th>Value</th>
          <th>Updated</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="s in shipments" :key="s.id">
          <td class="mono">{{ s.shipment_ref }}</td>
          <td>{{ s.buyer_name ?? '—' }}</td>
          <td><StatusBadge :label="s.status" size="sm" /></td>
          <td>{{ s.shipping_line ?? '—' }}</td>
          <td class="num">{{ fmtVal(s.total_value_usd) }}</td>
          <td class="date">{{ fmtDate(s.updated_at) }}</td>
        </tr>
        <tr v-if="!shipments.length">
          <td colspan="6" class="empty">No shipments yet</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.table-wrap {
  overflow-x: auto;
}

.shipment-table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--text-sm);
}

thead th {
  background: var(--bg-surface);
  padding: var(--space-sm) var(--space-md);
  text-align: left;
  font-weight: 600;
  color: var(--text-secondary);
  border-bottom: 2px solid var(--border-color);
  white-space: nowrap;
}

tbody td {
  padding: var(--space-sm) var(--space-md);
  border-bottom: 1px solid var(--border-light);
}

tbody tr:hover {
  background: rgba(52, 152, 219, 0.04);
}

.mono {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
}

.num {
  text-align: right;
  font-family: var(--font-mono);
}

.date {
  color: var(--text-secondary);
  white-space: nowrap;
}

.empty {
  text-align: center;
  padding: var(--space-xl);
  color: var(--text-secondary);
}
</style>
