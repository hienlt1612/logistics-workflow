import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Shipment } from '@/api/client';
import * as api from '@/api/client';

export const useShipmentsStore = defineStore('shipments', () => {
  const shipments = ref<Shipment[]>([]);
  const selectedId = ref<number | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const lastToast = ref<{ text: string; type: 'success' | 'error' } | null>(null);

  // Pagination
  const currentPage = ref(1);
  const pageSize = ref(20);
  const totalItems = ref(0);
  const totalPages = ref(0);
  // ponytail: remember last status filter so a bare loadAll() (e.g. after Save)
  // doesn't clobber the sidebar's active filter into an unfiltered reload.
  const currentStatus = ref<string | undefined>(undefined);

  const selected = computed(() =>
    shipments.value.find((s) => s.id === selectedId.value) ?? null
  );

  async function loadAll(statusFilter?: string, page?: number) {
    if (statusFilter !== undefined) currentStatus.value = statusFilter || undefined;
    loading.value = true;
    error.value = null;
    try {
      const result = await api.fetchShipments({
        status: currentStatus.value,
        page: page ?? currentPage.value,
        pageSize: pageSize.value,
      });
      shipments.value = result.data;
      currentPage.value = result.pagination.page;
      totalItems.value = result.pagination.totalItems;
      totalPages.value = result.pagination.totalPages;
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load shipments';
    } finally {
      loading.value = false;
    }
  }

  function goToPage(page: number) {
    if (page < 1 || page > totalPages.value) return;
    currentPage.value = page;
    loadAll(undefined, page);
  }

  function select(id: number | null) {
    selectedId.value = id;
  }

  async function create(data: api.CreateShipmentInput): Promise<Shipment | null> {
    loading.value = true;
    error.value = null;
    try {
      const s = await api.createShipment(data);
      await loadAll(); // reload to keep pagination correct
      selectedId.value = s.id;
      lastToast.value = { text: `Created ${s.shipment_ref}`, type: 'success' };
      return s;
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Failed to create shipment';
      error.value = msg;
      lastToast.value = { text: msg, type: 'error' };
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function updateCurrent(fields: Record<string, unknown>): Promise<Shipment | null> {
    if (!selectedId.value) return null;
    loading.value = true;
    error.value = null;
    try {
      const s = await api.updateShipment(selectedId.value, fields);
      const idx = shipments.value.findIndex((x) => x.id === s.id);
      if (idx >= 0) shipments.value[idx] = s;
      lastToast.value = { text: 'Saved successfully', type: 'success' };
      return s;
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Failed to update shipment';
      error.value = msg;
      lastToast.value = { text: msg, type: 'error' };
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function toggleChecklistField(field: string, value: boolean): Promise<void> {
    if (!selectedId.value) return;
    try {
      await api.toggleChecklist(selectedId.value, field, value);
      await loadAll();
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Failed to toggle checklist';
      error.value = msg;
      lastToast.value = { text: msg, type: 'error' };
    }
  }

  async function remove(id: number): Promise<boolean> {
    try {
      await api.deleteShipment(id);
      await loadAll();
      if (selectedId.value === id) selectedId.value = null;
      lastToast.value = { text: 'Shipment deleted', type: 'success' };
      return true;
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Failed to delete shipment';
      error.value = msg;
      lastToast.value = { text: msg, type: 'error' };
      return false;
    }
  }

  async function batchAdvance(ids: number[], status: string): Promise<number> {
    try {
      const count = await api.batchAdvanceStatus(ids, status);
      await loadAll();
      lastToast.value = { text: `Updated ${count} shipment(s) to ${status.replace(/_/g, ' ')}`, type: 'success' };
      return count;
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Batch update failed';
      error.value = msg;
      lastToast.value = { text: msg, type: 'error' };
      return 0;
    }
  }

  function clearToast() {
    lastToast.value = null;
  }

  return {
    shipments,
    selectedId,
    selected,
    loading,
    error,
    lastToast,
    currentPage,
    pageSize,
    totalItems,
    totalPages,
    loadAll,
    goToPage,
    select,
    create,
    updateCurrent,
    toggleChecklistField,
    remove,
    batchAdvance,
    clearToast,
  };
});
