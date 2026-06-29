<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  currentStatus: string;
}>();

interface Step {
  key: string;
  label: string;
  color: string;
}

const steps: Step[] = [
  { key: 'DRAFT', label: 'CREATE', color: 'var(--color-admin)' },
  { key: 'DOCUMENTS_READY', label: 'DRAFT', color: 'var(--color-manager)' },
  { key: 'CUSTOMS_CLEARED', label: 'CUSTOMS', color: 'var(--color-accounting)' },
  { key: 'CHECKLIST_IN_PROGRESS', label: 'CHECKLIST', color: 'var(--color-logistics)' },
  { key: 'COMPLETE', label: 'CHECKLIST', color: 'var(--color-logistics)' },
  { key: 'TELEX_RELEASED', label: 'TELEX', color: 'var(--color-telex)' },
];

const statusOrder = ['DRAFT', 'DOCUMENTS_READY', 'CUSTOMS_CLEARED', 'CHECKLIST_IN_PROGRESS', 'COMPLETE', 'TELEX_RELEASED'];

const currentIdx = computed(() => statusOrder.indexOf(props.currentStatus));

function stepState(idx: number): 'done' | 'current' | 'future' {
  if (idx < currentIdx.value) return 'done';
  if (idx === currentIdx.value) return 'current';
  return 'future';
}
</script>

<template>
  <div class="progress-bar">
    <template v-for="(step, idx) in steps.slice(0, 4)" :key="step.key">
      <div
        class="step"
        :class="stepState(idx)"
        :style="{ '--step-color': step.color }"
      >
        <div class="circle">
          <span v-if="stepState(idx) === 'done'">✓</span>
          <span v-else>{{ idx + 1 }}</span>
        </div>
        <span class="label">{{ step.label }}</span>
      </div>
      <div
        v-if="idx < 3"
        class="connector"
        :class="{ done: stepState(idx) === 'done' }"
      />
    </template>
  </div>
</template>

<style scoped>
.progress-bar {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-lg) var(--space-xl);
  gap: 0;
}

.step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-xs);
}

.circle {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: var(--text-sm);
  font-weight: 700;
  border: 2px solid var(--border-color);
  color: var(--text-secondary);
  background: var(--bg-surface);
  transition: all 0.3s;
}

.step.current .circle {
  border-color: var(--step-color);
  background: var(--step-color);
  color: var(--text-inverse);
}

.step.done .circle {
  border-color: var(--color-checklist);
  background: var(--color-checklist);
  color: var(--text-inverse);
}

.label {
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
}
.step.current .label { color: var(--step-color); }
.step.done .label { color: var(--color-checklist); }

.connector {
  width: 60px;
  height: 2px;
  background: var(--border-color);
  margin: 0 4px;
  margin-bottom: 18px;
}
.connector.done {
  background: var(--color-checklist);
}
</style>
