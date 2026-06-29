<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { Shipment } from '@/api/client';
import { useShipmentsStore } from '@/stores/shipments';

const store = useShipmentsStore();
const emit = defineEmits<{ saved: [] }>();

const form = ref({
  shipper_name: '',
  consignee_name: '',
  etd: '',
  invoice_number: '',
  invoice_date: '',
  total_value_usd: '',
  drafts_date: '',
  bill_of_lading: '',
});

const saving = ref(false);
const isReadOnly = computed(() => store.selected?.telex_released ?? false);

watch(
  () => store.selected,
  (s: Shipment | null) => {
    if (s) {
      form.value = {
        shipper_name: s.shipper_name ?? '',
        consignee_name: s.consignee_name ?? '',
        etd: s.etd ?? '',
        invoice_number: s.invoice_number ?? '',
        invoice_date: s.invoice_date ?? '',
        total_value_usd: s.total_value_usd ?? '',
        drafts_date: s.drafts_date ?? '',
        bill_of_lading: s.bill_of_lading ?? '',
      };
    }
  },
  { immediate: true }
);

function validate(): boolean {
  if (!form.value.shipper_name.trim()) { store.lastToast = { text: 'Shipper is required', type: 'error' }; return false; }
  if (!form.value.consignee_name.trim()) { store.lastToast = { text: 'Consignee is required', type: 'error' }; return false; }
  if (!form.value.etd) { store.lastToast = { text: 'ETD is required', type: 'error' }; return false; }
  if (!form.value.invoice_number.trim()) { store.lastToast = { text: 'Invoice Number is required', type: 'error' }; return false; }
  if (!form.value.total_value_usd.trim()) { store.lastToast = { text: 'Total Value is required', type: 'error' }; return false; }
  return true;
}

async function handleSave() {
  if (!store.selectedId) return;
  if (!validate()) return;
  saving.value = true;
  const result = await store.updateCurrent({ ...form.value, status: 'CUSTOMS_CLEARED' });
  saving.value = false;
  if (result) emit('saved');
}
</script>

<template>
  <form v-if="store.selected" class="step-form" @submit.prevent="handleSave">
    <div class="step-header manager">
      <h2>STEP 2: DRAFT DOCUMENTATION</h2>
      <span class="role-badge mgr-bg">MANAGER</span>
      <span v-if="isReadOnly" class="locked-badge">🔒 READ-ONLY</span>
    </div>

    <div class="form-grid">
      <label class="field">
        Shipper / Exporter <span class="required">*</span>
        <input v-model="form.shipper_name" required placeholder="e.g. An Dien Food" :disabled="isReadOnly" />
      </label>
      <label class="field">
        Consignee <span class="required">*</span>
        <input v-model="form.consignee_name" required placeholder="e.g. Global Trade Ltd." :disabled="isReadOnly" />
      </label>
      <label class="field">
        ETD <span class="required">*</span>
        <input v-model="form.etd" type="date" required :disabled="isReadOnly" />
      </label>
      <label class="field">
        Invoice Number <span class="required">*</span>
        <input v-model="form.invoice_number" required placeholder="e.g. INV-2026-001" :disabled="isReadOnly" />
      </label>
      <label class="field">
        Invoice Date
        <input v-model="form.invoice_date" type="date" :disabled="isReadOnly" />
      </label>
      <label class="field">
        Total Value USD <span class="required">*</span>
        <input v-model="form.total_value_usd" type="number" step="0.01" required placeholder="0.00" :disabled="isReadOnly" />
      </label>
      <label class="field">
        Drafts Date
        <input v-model="form.drafts_date" type="date" :disabled="isReadOnly" />
      </label>
      <label class="field">
        Bill of Lading #
        <input v-model="form.bill_of_lading" placeholder="e.g. BL2026001" :disabled="isReadOnly" />
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
  border-left: 4px solid var(--color-manager); background: var(--bg-surface);
}
.step-header h2 { flex: 1; font-size: var(--text-lg); font-weight: 700; }
.role-badge {
  padding: 2px 12px; border-radius: var(--radius-sm);
  font-size: var(--text-xs); font-weight: 700; color: var(--text-inverse); text-transform: uppercase;
}
.mgr-bg { background: var(--color-manager); }
.locked-badge { font-size: var(--text-xs); font-weight: 600; color: var(--text-secondary); }
.form-grid {
  display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-md); padding: var(--space-lg);
}
.field {
  display: flex; flex-direction: column; gap: var(--space-xs);
  font-size: var(--text-sm); font-weight: 500; color: var(--text-secondary);
}
.field input {
  padding: var(--space-sm); border: 1px solid var(--border-color);
  border-radius: var(--radius-sm); font-size: var(--text-base); color: var(--text-primary);
}
.field input:focus { outline: none; border-color: var(--color-manager); }
.field input:disabled { opacity: 0.6; background: var(--border-light); cursor: not-allowed; }
.required { color: var(--color-admin); }
.form-actions {
  padding: var(--space-md) var(--space-lg); border-top: 1px solid var(--border-light);
  display: flex; justify-content: flex-end;
}
.btn-primary {
  background: var(--color-manager); color: var(--text-inverse); border: none;
  padding: var(--space-sm) var(--space-xl); border-radius: var(--radius-sm);
  font-size: var(--text-base); font-weight: 600; cursor: pointer;
}
.btn-primary:hover { opacity: 0.9; }
.btn-primary:disabled { opacity: 0.5; }
</style>
