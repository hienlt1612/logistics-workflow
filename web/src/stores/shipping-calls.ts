import { defineStore } from 'pinia';
import { ref } from 'vue';
import * as api from '@/api/client';

// ponytail: reuse existing store pattern from shipments.ts.
// Skipped: pagination, polling, toast — add when call list grows.

export const useShippingCallsStore = defineStore('shippingCalls', () => {
  const calls = ref<api.ShippingCall[]>([]);
  const selected = ref<api.ShippingCall | null>(null);
  const warehouses = ref<api.CallWarehouse[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function loadAll() {
    loading.value = true;
    error.value = null;
    try {
      calls.value = await api.fetchShippingCalls();
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load calls';
    } finally {
      loading.value = false;
    }
  }

  async function loadOne(id: number) {
    loading.value = true;
    error.value = null;
    try {
      selected.value = await api.fetchShippingCall(id);
      warehouses.value = await api.fetchCallWarehouses(id);
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load call';
    } finally {
      loading.value = false;
    }
  }

  async function create(data: api.CreateShippingCallInput): Promise<api.ShippingCall | null> {
    loading.value = true;
    error.value = null;
    try {
      const c = await api.createShippingCall(data);
      await loadAll();
      return c;
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to create call';
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function update(id: number, data: Partial<api.CreateShippingCallInput>): Promise<api.ShippingCall | null> {
    loading.value = true;
    error.value = null;
    try {
      const c = await api.updateShippingCall(id, data);
      selected.value = c;
      return c;
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to update call';
      return null;
    } finally {
      loading.value = false;
    }
  }

  return { calls, selected, warehouses, loading, error, loadAll, loadOne, create, update };
});
