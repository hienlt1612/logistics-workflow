import { defineStore } from 'pinia';
import { ref, onUnmounted } from 'vue';
import type { DashboardStats } from '@/api/client';
import * as api from '@/api/client';

export const useDashboardStore = defineStore('dashboard', () => {
  const stats = ref<DashboardStats>({
    total: 0,
    draft: 0,
    documents: 0,
    customs: 0,
    checklist: 0,
    telex: 0,
    calls_total: 0,
    calls_open: 0,
    calls_loading: 0,
    calls_closed: 0,
  });
  const loading = ref(false);
  const error = ref<string | null>(null);
  const lastUpdated = ref<Date | null>(null);
  let timer: ReturnType<typeof setInterval> | null = null;

  async function load() {
    loading.value = true;
    error.value = null;
    try {
      stats.value = await api.fetchDashboard();
      lastUpdated.value = new Date();
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load dashboard';
    } finally {
      loading.value = false;
    }
  }

  function startPolling(intervalMs = 30000) {
    stopPolling();
    load();
    timer = setInterval(load, intervalMs);
  }

  function stopPolling() {
    if (timer) {
      clearInterval(timer);
      timer = null;
    }
  }

  function secondsAgo(): number {
    if (!lastUpdated.value) return 999;
    return Math.floor((Date.now() - lastUpdated.value.getTime()) / 1000);
  }

  onUnmounted(() => stopPolling());

  return { stats, loading, error, lastUpdated, load, startPolling, stopPolling, secondsAgo };
});
