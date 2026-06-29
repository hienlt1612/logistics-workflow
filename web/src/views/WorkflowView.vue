<script setup lang="ts">
import { onMounted, watch, computed } from 'vue';
import { useRoute } from 'vue-router';
import { useShipmentsStore } from '@/stores/shipments';
import WorkflowProgress from '@/components/workflow/WorkflowProgress.vue';
import Step1Create from '@/components/workflow/Step1Create.vue';
import Step2Draft from '@/components/workflow/Step2Draft.vue';
import Step3Customs from '@/components/workflow/Step3Customs.vue';
import Step4Checklist from '@/components/workflow/Step4Checklist.vue';

const route = useRoute();
const store = useShipmentsStore();

const statusOrder = ['DRAFT', 'DOCUMENTS_READY', 'CUSTOMS_CLEARED', 'CHECKLIST_IN_PROGRESS', 'COMPLETE', 'TELEX_RELEASED'];

const currentStatusIdx = computed(() => {
  if (!store.selected) return -1;
  return statusOrder.indexOf(store.selected.status);
});

onMounted(() => store.loadAll());

watch(
  () => route.params.id,
  (id) => { if (id) store.select(Number(id)); },
  { immediate: true }
);

function onSaved() {
  store.loadAll();
}
</script>

<template>
  <div class="workflow-view">
    <template v-if="store.selected">
      <div class="workflow-header">
        <WorkflowProgress :current-status="store.selected.status" />
        <router-link
          :to="`/shipment/${store.selected.id}`"
          class="detail-link"
        >📋 View Full Detail</router-link>
      </div>

      <div class="steps-stack">
        <Step1Create @saved="onSaved" />
        <Step2Draft v-if="currentStatusIdx >= 1" @saved="onSaved" />
        <Step3Customs v-if="currentStatusIdx >= 2" @saved="onSaved" />
        <Step4Checklist v-if="currentStatusIdx >= 3" @saved="onSaved" />
      </div>
    </template>

    <div v-else class="empty-state">
      <p class="empty-icon">📋</p>
      <h2>Select a shipment</h2>
      <p>Choose a shipment from the sidebar or create a new one to begin.</p>
    </div>
  </div>
</template>

<style scoped>
.workflow-view {
  padding: var(--space-xl);
  max-width: 900px;
}

.steps-stack {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

.workflow-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-md);
}
.detail-link {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-manager);
  white-space: nowrap;
  padding: var(--space-xs) var(--space-sm);
  border-radius: var(--radius-sm);
  transition: background 0.15s;
}
.detail-link:hover {
  background: rgba(52, 152, 219, 0.1);
}

.empty-state {
  text-align: center;
  padding: var(--space-2xl);
  color: var(--text-secondary);
}
.empty-icon { font-size: 3rem; margin-bottom: var(--space-md); }
.empty-state h2 { font-size: var(--text-xl); color: var(--text-primary); margin-bottom: var(--space-sm); }
</style>
