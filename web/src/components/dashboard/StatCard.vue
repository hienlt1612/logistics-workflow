<script setup lang="ts">
defineProps<{
  label: string;
  value: number;
  color: string;
}>();
</script>

<template>
  <div class="stat-card" :style="{ borderTopColor: color }">
    <div class="stat-value">{{ value }}</div>
    <div class="stat-label">{{ label }}</div>
  </div>
</template>

<style scoped>
.stat-card {
  background: var(--bg-card);
  border-radius: var(--radius-md);
  padding: var(--space-md) var(--space-sm);
  border-top: 3px solid;
  box-shadow: var(--shadow-sm);
  text-align: center;
}

.stat-value {
  font-size: var(--text-xl);
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.1;
}

.stat-label {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  margin-top: 2px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}
</style>
