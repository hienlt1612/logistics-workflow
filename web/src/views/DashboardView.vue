<script setup lang="ts">
import { onMounted, onUnmounted, computed } from 'vue';
import { useDashboardStore } from '@/stores/dashboard';
import { useShipmentsStore } from '@/stores/shipments';
import StatCard from '@/components/dashboard/StatCard.vue';
import ShipmentTable from '@/components/dashboard/ShipmentTable.vue';

const dash = useDashboardStore();
const ship = useShipmentsStore();

const cards = [
  { label: 'Total', key: 'total' as const, color: 'var(--color-manager)' },
  { label: 'Draft', key: 'draft' as const, color: 'var(--color-draft)' },
  { label: 'Documents', key: 'documents' as const, color: 'var(--color-documents)' },
  { label: 'Customs', key: 'customs' as const, color: 'var(--color-customs)' },
  { label: 'Checklist', key: 'checklist' as const, color: 'var(--color-checklist)' },
  { label: 'Telex', key: 'telex' as const, color: 'var(--color-telex)' },
];

const ageText = computed(() => {
  const s = dash.secondsAgo();
  if (s < 5) return 'just now';
  if (s < 60) return `${s}s ago`;
  return `${Math.floor(s / 60)}m ago`;
});

onMounted(() => {
  dash.startPolling(30000);
  ship.loadAll();
});

onUnmounted(() => {
  dash.stopPolling();
});
</script>

<template>
  <div class="dashboard">
    <div class="page-header">
      <h1 class="page-title">Dashboard</h1>
      <span v-if="dash.lastUpdated" class="last-updated" title="Auto-refreshes every 30s">
        🔄 Updated {{ ageText }}
      </span>
    </div>

    <div v-if="dash.loading && !dash.lastUpdated" class="loading">Loading stats...</div>

    <div v-else class="stats-grid">
      <StatCard
        v-for="card in cards"
        :key="card.key"
        :label="card.label"
        :value="dash.stats[card.key]"
        :color="card.color"
      />
    </div>

    <section class="section">
      <h2>Recent Shipments</h2>
      <div v-if="ship.loading" class="loading">Loading shipments...</div>
      <ShipmentTable v-else :shipments="ship.shipments.slice(0, 10)" />
    </section>
  </div>
</template>

<style scoped>
.dashboard {
  padding: var(--space-xl);
  max-width: 1200px;
}

.page-header {
  display: flex;
  align-items: baseline;
  gap: var(--space-md);
  margin-bottom: var(--space-lg);
}

.page-title {
  font-size: var(--text-2xl);
  font-weight: 700;
}

.last-updated {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  font-weight: 500;
  white-space: nowrap;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: var(--space-md);
  margin-bottom: var(--space-2xl);
}

.section h2 {
  font-size: var(--text-lg);
  font-weight: 600;
  margin-bottom: var(--space-md);
}

.loading {
  text-align: center;
  padding: var(--space-2xl);
  color: var(--text-secondary);
}
</style>
