<script setup lang="ts">
defineProps<{
  label: string;
  color?: string;
  size?: 'sm' | 'md';
}>();

const statusMap: Record<string, string> = {
  DRAFT: 'var(--color-draft)',
  DOCUMENTS_READY: 'var(--color-documents)',
  CUSTOMS_CLEARED: 'var(--color-customs)',
  CHECKLIST_IN_PROGRESS: 'var(--color-checklist)',
  COMPLETE: 'var(--color-complete)',
  TELEX_RELEASED: 'var(--color-telex)',
  // ponytail: call status colors
  OPEN: 'var(--color-manager)',
  ON_LOADING: 'var(--color-checklist)',
  CLOSED: '#7f8c8d',
};
</script>

<template>
  <span
    class="badge"
    :class="size"
    :style="{ background: color ?? statusMap[label] ?? 'var(--text-secondary)' }"
  >
    {{ label.replace(/_/g, ' ') }}
  </span>
</template>

<style scoped>
.badge {
  display: inline-block;
  padding: 2px 10px;
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--text-inverse);
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.badge.sm {
  padding: 1px 8px;
  font-size: 0.65rem;
}
.badge.md {
  padding: 3px 12px;
  font-size: var(--text-sm);
}
</style>
