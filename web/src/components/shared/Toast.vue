<script setup lang="ts">
import { watch, ref } from 'vue';

export interface ToastMessage {
  id: number;
  text: string;
  type: 'success' | 'error';
}

const toasts = ref<ToastMessage[]>([]);
let nextId = 0;

function show(text: string, type: 'success' | 'error' = 'success', duration = 3500) {
  const id = nextId++;
  toasts.value.push({ id, text, type });
  setTimeout(() => {
    toasts.value = toasts.value.filter((t) => t.id !== id);
  }, duration);
}

defineExpose({ show });
</script>

<template>
  <Teleport to="body">
    <div class="toast-container" aria-live="polite">
      <div
        v-for="t in toasts"
        :key="t.id"
        class="toast"
        :class="t.type"
      >
        <span class="toast-icon">{{ t.type === 'success' ? '✓' : '✕' }}</span>
        <span class="toast-text">{{ t.text }}</span>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.toast-container {
  position: fixed;
  bottom: var(--space-lg);
  right: var(--space-lg);
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
  max-width: 360px;
}

.toast {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-md) var(--space-lg);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  color: var(--text-inverse);
  font-size: var(--text-sm);
  font-weight: 500;
  animation: slide-in 0.25s ease-out;
}

.toast.success {
  background: var(--color-checklist);
}

.toast.error {
  background: var(--color-admin);
}

.toast-icon {
  font-size: var(--text-lg);
  font-weight: 700;
}

.toast-text {
  flex: 1;
}

@keyframes slide-in {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}
</style>
