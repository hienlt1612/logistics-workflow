<script setup lang="ts">
defineProps<{
  label: string;
  value: number;
  color: string;
  active?: boolean;
}>();

defineEmits<{ click: [] }>();
</script>

<template>
  <div class="stat-card" :class="{ active }" :style="{ borderTopColor: color }" @click="$emit('click')">
    <div class="stat-value">{{ value }}</div>
    <div class="stat-label">{{ label }}</div>
  </div>
</template>

<style scoped>
.stat-card {
  background: var(--bg-card);
  border-radius: var(--radius-md);
  padding: var(--space-sm);
  border-top: 3px solid;
  box-shadow: var(--shadow-sm);
  text-align: center;
  cursor: pointer;
  transition: transform 0.1s, box-shadow 0.1s;
  aspect-ratio: 1;
  width: 55px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.stat-card:hover { transform: translateY(-1px); box-shadow: var(--shadow-md); }
.stat-card.active { box-shadow: 0 0 0 2px var(--color-manager); }

.stat-value {
  font-size: var(--text-md);
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.1;
}

.stat-label {
  font-size: 8px;
  color: var(--text-secondary);
  margin-top: 2px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  white-space: nowrap;
}
</style>
