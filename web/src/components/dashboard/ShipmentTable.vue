<script setup lang="ts">
import type { Shipment } from '@/api/client';
import StatusBadge from '@/components/shared/StatusBadge.vue';
import { fmtDateDisplay } from '@/utils/format';

defineProps<{
  shipments: Shipment[];
}>();

function fmtDate(iso: string): string {
  return fmtDateDisplay(iso);
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
          <th>Booking #</th>
          <th>ETD</th>
          <th>Invoice #</th>
          <th>Loading Plan</th>
          <th>Updated</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="s in shipments" :key="s.id">
          <td class="mono">{{ s.shipment_ref }}</td>
          <td>{{ s.buyer_name ?? '—' }}</td>
          <td><StatusBadge :label="s.status" size="sm" /></td>
          <td>{{ s.booking_number ?? '—' }}</td>
          <td>{{ s.etd ?? '—' }}</td>
          <td>{{ s.invoice_number ?? '—' }}</td>
          <td>{{ s.loading_plan ?? '—' }}</td>
          <td class="date">{{ fmtDate(s.updated_at) }}</td>
        </tr>
        <tr v-if="!shipments.length">
          <td colspan="8" class="empty">No shipments yet</td>
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
