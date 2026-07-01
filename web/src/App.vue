<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import { RouterView, useRouter } from 'vue-router';
import AppLayout from '@/components/layout/AppLayout.vue';
import Toast from '@/components/shared/Toast.vue';
import { useShipmentsStore } from '@/stores/shipments';
import { useAuthStore } from '@/stores/auth';

const router = useRouter();
const ship = useShipmentsStore();
const auth = useAuthStore();
const toastRef = ref<InstanceType<typeof Toast> | null>(null);

// ponytail: 59-min idle session timeout — any action resets
const IDLE_MINUTES = 59;
let idleTimer: ReturnType<typeof setTimeout> | null = null;

function resetIdleTimer() {
  if (idleTimer) clearTimeout(idleTimer);
  if (!auth.isLoggedIn || router.currentRoute.value.path === '/login') return;
  idleTimer = setTimeout(() => {
    auth.logout();
    router.push('/login');
  }, IDLE_MINUTES * 60 * 1000);
}

function onUserActivity() { resetIdleTimer(); }

onMounted(() => {
  resetIdleTimer();
  document.addEventListener('click', onUserActivity, { passive: true });
  document.addEventListener('keydown', onUserActivity, { passive: true });
  document.addEventListener('scroll', onUserActivity, { passive: true });
  router.afterEach(() => resetIdleTimer());
});

onUnmounted(() => {
  if (idleTimer) clearTimeout(idleTimer);
  document.removeEventListener('click', onUserActivity);
  document.removeEventListener('keydown', onUserActivity);
  document.removeEventListener('scroll', onUserActivity);
});

watch(
  () => ship.lastToast,
  (t) => {
    if (t && toastRef.value) {
      toastRef.value.show(t.text, t.type);
      ship.clearToast();
    }
  }
);
</script>

<template>
  <AppLayout v-if="auth.isLoggedIn">
    <RouterView />
  </AppLayout>
  <RouterView v-else />
  <Toast ref="toastRef" />
</template>
