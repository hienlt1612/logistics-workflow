<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { Shipment } from '@/api/client';
import { useShipmentsStore } from '@/stores/shipments';

const store = useShipmentsStore();
const emit = defineEmits<{ saved: [] }>();

const form = ref({
  customs_date: '',
  customs_number: '',
  customs_status: '',
});

const saving = ref(false);
const isReadOnly = computed(() => store.selected?.telex_released ?? false);

watch(
  () => store.selected,
  (s: Shipment | null) => {
    if (s) {
      form.value = {
        customs_date: s.customs_date ?? '',
        customs_number: s.customs_number ?? '',
        customs_status: s.customs_status ?? '',
      };
    }
  },
  { immediate: true }
);

function validate(): boolean {
  if (!form.value.customs_date) { store.lastToast = { text: 'Customs Date is required', type: 'error' }; return false; }
  if (!form.value.customs_number.trim()) { store.lastToast = { text: 'Customs Number is required', type: 'error' }; return false; }
  return true;
}

async function handleSave() {
  if (!store.selectedId) return;
  if (!validate()) return;
  saving.value = true;
  const result = await store.updateCurrent({ ...form.value, status: 'CHECKLIST_IN_PROGRESS' });
  saving.value = false;
  if (result) emit('saved');
}
</script>

<template>
  <form v-if="store.selected" class="step-form" @submit.prevent="handleSave">
    <div class="step-header acct">
      <h2>STEP 3: CUSTOMS CLEARANCE</h2>
      <span class="role-badge acct-bg">ACCOUNTING</span>
      <span v-if="isReadOnly" class="locked-badge">🔒 READ-ONLY</span>
    </div>

    <div class="form-grid">
      <label class="field">
        Customs Date <span class="required">*</span>
        <input v-model="form.customs_date" type="date" required :disabled="isReadOnly" />
      </label>
      <label class="field">
        Customs Number <span class="required">*</span>
        <input v-model="form.customs_number" required placeholder="e.g. CUS2026001" :disabled="isReadOnly" />
      </label>
      <label class="field full-width">
        Customs Status
        <select v-model="form.customs_status" :disabled="isReadOnly">
          <option value="">— Select —</option>
          <option value="red">Red Channel</option>
          <option value="yellow">Yellow Channel</option>
          <option value="green">Green Channel</option>
        </select>
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
  border-left: 4px solid var(--color-accounting); background: var(--bg-surface);
}
.step-header h2 { flex: 1; font-size: var(--text-lg); font-weight: 700; }
.role-badge {
  padding: 2px 12px; border-radius: var(--radius-sm);
  font-size: var(--text-xs); font-weight: 700; color: var(--text-inverse); text-transform: uppercase;
}
.acct-bg { background: var(--color-accounting); }
.locked-badge { font-size: var(--text-xs); font-weight: 600; color: var(--text-secondary); }
.form-grid {
  display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-md); padding: var(--space-lg);
}
.field {
  display: flex; flex-direction: column; gap: var(--space-xs);
  font-size: var(--text-sm); font-weight: 500; color: var(--text-secondary);
}
.field input, .field select {
  padding: var(--space-sm); border: 1px solid var(--border-color);
  border-radius: var(--radius-sm); font-size: var(--text-base); color: var(--text-primary);
}
.field input:focus, .field select:focus { outline: none; border-color: var(--color-accounting); }
.field input:disabled, .field select:disabled { opacity: 0.6; background: var(--border-light); cursor: not-allowed; }
.required { color: var(--color-admin); }
.full-width { grid-column: 1 / -1; }
.form-actions {
  padding: var(--space-md) var(--space-lg); border-top: 1px solid var(--border-light);
  display: flex; justify-content: flex-end;
}
.btn-primary {
  background: var(--color-accounting); color: var(--text-inverse); border: none;
  padding: var(--space-sm) var(--space-xl); border-radius: var(--radius-sm);
  font-size: var(--text-base); font-weight: 600; cursor: pointer;
}
.btn-primary:hover { opacity: 0.9; }
.btn-primary:disabled { opacity: 0.5; }
</style>
