<script setup lang="ts">
import { ref, watch, computed, onMounted } from 'vue';
import type { Shipment, ShippingCall } from '@/api/client';
import { useShipmentsStore } from '@/stores/shipments';
import * as api from '@/api/client';

const store = useShipmentsStore();
const emit = defineEmits<{ saved: [] }>();

const openCalls = ref<ShippingCall[]>([]);
const selectedCallId = ref<number | null>(null);

const form = ref({
  buyer_name: '',
  booking_number: '',
  shipping_line: '',
  origin_port: '',
  warehouse_loc: '',
  loading_plan: '',
});

const saving = ref(false);
const isReadOnly = computed(() => store.selected?.telex_released ?? false);

onMounted(async () => {
  try {
    const calls = (await api.fetchShippingCalls()).filter(c => c.status === 'OPEN');
    // ponytail: include the selected shipment's call (may be ON_LOADING, not in
    // OPEN list) here so this assignment can't clobber the watch's added call.
    const cid = store.selected?.shipping_call_id;
    if (cid && !calls.find(c => c.id === cid)) {
      try { calls.push(await api.fetchShippingCall(cid)); } catch { /* silent */ }
    }
    openCalls.value = calls;
  } catch { /* silent */ }
});

watch(
  () => store.selected,
  (s: Shipment | null) => {
    if (s) {
      form.value = {
        buyer_name: s.buyer_name ?? '',
        booking_number: s.booking_number ?? '',
        shipping_line: s.shipping_line ?? '',
        origin_port: s.origin_port ?? '',
        warehouse_loc: s.warehouse_loc ?? '',
        loading_plan: s.loading_plan ?? '',
      };
      selectedCallId.value = s.shipping_call_id ?? null;
      // ponytail: selected call may be ON_LOADING (not in OPEN-only list), add it
      if (s.shipping_call_id && !openCalls.value.find(c => c.id === s.shipping_call_id)) {
        void (async () => {
          try { const c = await api.fetchShippingCall(s.shipping_call_id!); openCalls.value.unshift(c); }
          catch { /* silent */ }
        })();
      }
    }
  },
  { immediate: true }
);

// ponytail: auto-fill buyer from selected shipping call
watch(selectedCallId, (id) => {
  const call = openCalls.value.find(c => c.id === id);
  if (call) form.value.buyer_name = call.buyer_name;
});

function validate(): boolean {
  const t = (v: unknown) => String(v ?? '').trim();
  const req = ['buyer_name', 'booking_number', 'shipping_line', 'origin_port'];
  for (const key of req) {
    const val = (form.value as unknown as Record<string, unknown>)[key];
    if (!t(val)) {
      store.lastToast = { text: `${key.replace(/_/g, ' ').toUpperCase()} is required`, type: 'error' };
      return false;
    }
  }
  if (!selectedCallId.value) {
    store.lastToast = { text: 'Please select a Shipping Call', type: 'error' };
    return false;
  }
  return true;
}

async function handleSave() {
  if (!store.selectedId) return;
  if (!validate()) return;
  saving.value = true;
  const payload: any = { ...form.value, shipping_call_id: selectedCallId.value, status: 'DOCUMENTS_READY' };
  const result = await store.updateCurrent(payload);
  saving.value = false;
  if (result) emit('saved');
}
</script>

<template>
  <form v-if="store.selected" class="step-form" @submit.prevent="handleSave">
    <div class="step-header admin">
      <h2>STEP 1: CREATE SHIPMENT</h2>
      <span class="role-badge admin-bg">ADMIN</span>
      <span v-if="isReadOnly" class="locked-badge">🔒 READ-ONLY</span>
    </div>

    <div class="form-grid">
      <label class="field full-width">
        Shipping Call <span class="required">*</span>
        <select v-model.number="selectedCallId" :disabled="isReadOnly">
          <option :value="null">— Select Shipping Call —</option>
          <option v-for="c in openCalls" :key="c.id" :value="c.id">
            {{ c.call_ref }} — {{ c.buyer_name }} ({{ c.total_containers }} containers)
          </option>
        </select>
      </label>

      <label class="field">
        Buyer <span class="required">*</span>
        <input v-model="form.buyer_name" required placeholder="e.g. Global Trade Ltd." :disabled="isReadOnly" />
      </label>

      <label class="field">
        Booking Number <span class="required">*</span>
        <input v-model="form.booking_number" required placeholder="e.g. BK2026001" :disabled="isReadOnly" />
      </label>

      <label class="field">
        Shipping Line <span class="required">*</span>
        <input v-model="form.shipping_line" required placeholder="e.g. Maersk" :disabled="isReadOnly" />
      </label>

      <label class="field">
        Port of Loading <span class="required">*</span>
        <input v-model="form.origin_port" required placeholder="e.g. Haiphong" :disabled="isReadOnly" />
      </label>

      <label class="field">
        Warehouse
        <input v-model="form.warehouse_loc" placeholder="e.g. Nam Dinh" :disabled="isReadOnly" />
      </label>

      <label class="field full-width">
        Loading Plan
        <textarea v-model="form.loading_plan" rows="2" placeholder="e.g. 3K A10" :disabled="isReadOnly" />
      </label>
    </div>

    <div class="form-actions">
      <button type="submit" class="btn-primary" :disabled="saving || isReadOnly || store.loading">
        {{ saving || store.loading ? 'Saving...' : 'Save & Continue →' }}
      </button>
    </div>
  </form>
</template>

<style scoped>
.step-form {
  background: var(--bg-card); border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm); overflow: hidden;
}
.step-header {
  display: flex; align-items: center; gap: var(--space-md);
  padding: var(--space-md) var(--space-lg);
  border-left: 4px solid var(--color-admin); background: var(--bg-surface);
}
.step-header h2 { flex: 1; font-size: var(--text-lg); font-weight: 700; }
.role-badge {
  padding: 2px 12px; border-radius: var(--radius-sm);
  font-size: var(--text-xs); font-weight: 700; color: var(--text-inverse); text-transform: uppercase;
}
.admin-bg { background: var(--color-admin); }
.locked-badge { font-size: var(--text-xs); font-weight: 600; color: var(--text-secondary); }
.form-grid {
  display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-md); padding: var(--space-lg);
}
.field {
  display: flex; flex-direction: column; gap: var(--space-xs);
  font-size: var(--text-sm); font-weight: 500; color: var(--text-secondary);
}
.field input, .field textarea, .field select {
  padding: var(--space-sm); border: 1px solid var(--border-color);
  border-radius: var(--radius-sm); font-size: var(--text-base); color: var(--text-primary);
  background: var(--bg-card);
}
.field input:focus, .field textarea:focus, .field select:focus {
  outline: none; border-color: var(--color-admin);
}
.field input:disabled, .field textarea:disabled, .field select:disabled { opacity: 0.6; background: var(--border-light); cursor: not-allowed; }
.required { color: var(--color-admin); }
.full-width { grid-column: 1 / -1; }
.form-actions {
  padding: var(--space-md) var(--space-lg); border-top: 1px solid var(--border-light);
  display: flex; justify-content: flex-end;
}
.btn-primary {
  background: var(--color-admin); color: var(--text-inverse); border: none;
  padding: var(--space-sm) var(--space-xl); border-radius: var(--radius-sm);
  font-size: var(--text-base); font-weight: 600; cursor: pointer;
}
.btn-primary:hover { opacity: 0.9; }
.btn-primary:disabled { opacity: 0.5; }
</style>
