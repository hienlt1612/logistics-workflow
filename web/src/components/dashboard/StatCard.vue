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
  padding: var(--space-lg);
  border-top: 3px solid;
  box-shadow: var(--shadow-sm);
  text-align: center;
}

.stat-value {
  font-size: var(--text-3xl);
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
}

.stat-label {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin-top: var(--space-xs);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
</style>
