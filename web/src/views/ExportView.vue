<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useShipmentsStore } from '@/stores/shipments';
import { downloadExcel } from '@/api/client';

const store = useShipmentsStore();
const exporting = ref(false);

onMounted(() => store.loadAll());

async function handleExport() {
  exporting.value = true;
  try {
    await downloadExcel();
    store.lastToast = { text: 'Export downloaded!', type: 'success' };
  } catch {
    store.lastToast = { text: 'Export failed', type: 'error' };
  } finally {
    exporting.value = false;
  }
}
</script>

<template>
  <div class="export-view">
    <h1 class="page-title">Export</h1>

    <div class="export-card">
      <p class="export-info">
        {{ store.shipments.length }} shipments ready for export
      </p>
      <button class="export-btn" :disabled="exporting" @click="handleExport">
        {{ exporting ? 'Generating...' : '📥 Download Business Process Report' }}
      </button>
      <p class="export-hint">
        Full business-process report — Shipping Calls with their Shipments in a
        Pivot View, a flat Shipments detail sheet, and a Summary sheet.
      </p>
    </div>
  </div>
</template>

<style scoped>
.export-view {
  padding: var(--space-xl);
  max-width: 600px;
}

.page-title {
  font-size: var(--text-2xl);
  font-weight: 700;
  margin-bottom: var(--space-lg);
}

.export-card {
  background: var(--bg-card);
  border-radius: var(--radius-md);
  padding: var(--space-xl);
  box-shadow: var(--shadow-sm);
  text-align: center;
}

.export-info {
  font-size: var(--text-lg);
  color: var(--text-secondary);
  margin-bottom: var(--space-lg);
}

.export-btn {
  background: var(--color-checklist);
  color: var(--text-inverse);
  border: none;
  padding: var(--space-md) var(--space-xl);
  border-radius: var(--radius-md);
  font-size: var(--text-base);
  font-weight: 600;
  cursor: pointer;
}
.export-btn:hover { opacity: 0.9; }
.export-btn:disabled { opacity: 0.5; }

.export-hint {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin-top: var(--space-md);
}
</style>
