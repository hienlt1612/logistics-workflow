<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { Shipment } from '@/api/client';
import { useShipmentsStore } from '@/stores/shipments';
import { useAuthStore } from '@/stores/auth';
import * as api from '@/api/client';

const store = useShipmentsStore();
const auth = useAuthStore();

interface BoolField {
  key: keyof Shipment;
  label: string;
  color: string;
}

const docFields: BoolField[] = [
  { key: 'bl_received', label: 'BL Received', color: 'var(--color-manager)' },
  { key: 'docs_confirmed', label: 'Docs Confirmed', color: 'var(--color-manager)' },
];

const chargeFields: BoolField[] = [
  { key: 'charges_paid', label: 'Charges / THC Paid', color: 'var(--color-accounting)' },
];

const certFields: BoolField[] = [
  { key: 'co_received', label: 'CO Received', color: 'var(--color-logistics)' },
  { key: 'phyto_received', label: 'Phyto Received', color: 'var(--color-logistics)' },
];

const payForm = ref({
  prepayment_date: '',
  prepayment_amt: '',
  remaining_amt: '',
});

const origForm = ref({
  originals_status: '',
  originals_sent: '',
  originals_description: '',
});

const isReadOnly = computed(() => store.selected?.telex_released ?? false);
const savingPayments = ref(false);
const savingOriginals = ref(false);

watch(
  () => store.selected,
  (s: Shipment | null) => {
    if (s && !savingPayments.value && !savingOriginals.value) {
      payForm.value = {
        prepayment_date: s.prepayment_date ?? '',
        prepayment_amt: s.prepayment_amt ?? '',
        remaining_amt: s.remaining_amt ?? '',
      };
      origForm.value = {
        originals_status: s.originals_status ?? '',
        originals_sent: s.originals_sent ?? '',
        originals_description: s.originals_description ?? '',
      };
    }
  },
  { immediate: true }
);

async function toggle(field: keyof Shipment, current: boolean) {
  if (isReadOnly.value) return;
  await store.toggleChecklistField(field as string, !current);
}

const showTelexConfirm = ref(false);

async function confirmTelex() {
  await store.toggleChecklistField('telex_released', true);
  showTelexConfirm.value = false;
}

async function handleTelexToggle() {
  if (!store.selected) return;
  if (store.selected.telex_released) {
    // Admin can revert telex release
    if (!auth.isAdmin) return;
    if (!confirm('Revert Telex Release? This will allow modifications to this shipment again.')) return;
    await store.toggleChecklistField('telex_released', false);
  } else {
    // Show confirmation before releasing
    showTelexConfirm.value = true;
  }
}

async function savePayments() {
  if (!store.selectedId || isReadOnly.value) return;
  savingPayments.value = true;
  await store.updateCurrent({
    prepayment_date: payForm.value.prepayment_date,
    prepayment_amt: payForm.value.prepayment_amt === '' || payForm.value.prepayment_amt === null ? '' : String(payForm.value.prepayment_amt),
    remaining_amt: payForm.value.remaining_amt === '' || payForm.value.remaining_amt === null ? '' : String(payForm.value.remaining_amt),
  });
  if (store.selectedId) {
    const fresh = await api.fetchShipment(store.selectedId);
    payForm.value.prepayment_date = fresh.prepayment_date ?? '';
    payForm.value.prepayment_amt = fresh.prepayment_amt ?? '';
    payForm.value.remaining_amt = fresh.remaining_amt ?? '';
  }
  savingPayments.value = false;
}

async function saveOriginals() {
  if (!store.selectedId || isReadOnly.value) return;
  savingOriginals.value = true;
  await store.updateCurrent(origForm.value);
  if (store.selectedId) {
    const fresh = await api.fetchShipment(store.selectedId);
    origForm.value.originals_status = fresh.originals_status ?? '';
    origForm.value.originals_sent = fresh.originals_sent ?? '';
    origForm.value.originals_description = fresh.originals_description ?? '';
  }
  savingOriginals.value = false;
}
</script>

<template>
  <div v-if="store.selected" class="checklist">
    <div class="step-header">
      <h2>STEP 4: DOCUMENT CHECKLIST</h2>
      <span v-if="isReadOnly" class="locked-badge">🔒 READ-ONLY</span>
    </div>

    <!-- DOCUMENTS -->
    <section class="check-section" style="border-left-color: var(--color-manager)">
      <h3 class="section-title" style="color: var(--color-manager)">DOCUMENTS</h3>
      <div class="bool-grid">
        <label v-for="f in docFields" :key="f.key" class="bool-item" :style="{ '--accent': f.color }">
          <input type="checkbox" :checked="!!store.selected[f.key]" :disabled="isReadOnly" @change="toggle(f.key, !!store.selected[f.key])" />
          <span>{{ f.label }}</span>
        </label>
      </div>
    </section>

    <!-- CHARGES -->
    <section class="check-section" style="border-left-color: var(--color-accounting)">
      <h3 class="section-title" style="color: var(--color-accounting)">CHARGES</h3>
      <div class="bool-grid">
        <label v-for="f in chargeFields" :key="f.key" class="bool-item" :style="{ '--accent': f.color }">
          <input type="checkbox" :checked="!!store.selected[f.key]" :disabled="isReadOnly" @change="toggle(f.key, !!store.selected[f.key])" />
          <span>{{ f.label }}</span>
        </label>
      </div>
    </section>

    <!-- CERTIFICATES -->
    <section class="check-section" style="border-left-color: var(--color-logistics)">
      <h3 class="section-title" style="color: var(--color-logistics)">CERTIFICATES</h3>
      <div class="bool-grid">
        <label v-for="f in certFields" :key="f.key" class="bool-item" :style="{ '--accent': f.color }">
          <input type="checkbox" :checked="!!store.selected[f.key]" :disabled="isReadOnly" @change="toggle(f.key, !!store.selected[f.key])" />
          <span>{{ f.label }}</span>
        </label>
      </div>
    </section>

    <!-- PAYMENT -->
    <section class="check-section" style="border-left-color: var(--color-accounting)">
      <h3 class="section-title" style="color: var(--color-accounting)">PAYMENT</h3>
      <div class="bool-grid">
        <label class="bool-item" style="--accent: var(--color-accounting)">
          <input type="checkbox" :checked="!!store.selected.payment_received" :disabled="isReadOnly" @change="toggle('payment_received', !!store.selected.payment_received)" />
          <span>Received Payment</span>
        </label>
      </div>
      <div class="form-grid-3">
        <label class="field">Payment Date <input v-model="payForm.prepayment_date" type="date" :disabled="isReadOnly" /></label>
        <label class="field">Prepayment Amount <input v-model="payForm.prepayment_amt" type="number" step="0.01" :disabled="isReadOnly" /></label>
        <label class="field">Remaining <input v-model="payForm.remaining_amt" type="number" step="0.01" :disabled="isReadOnly" /></label>
      </div>
      <div class="section-actions">
        <button class="btn-save" :disabled="savingPayments || isReadOnly" @click="savePayments">{{ savingPayments ? 'Saving...' : 'Save Payment' }}</button>
      </div>
    </section>

    <!-- ORIGINALS -->
    <section class="check-section" style="border-left-color: var(--color-manager)">
      <h3 class="section-title" style="color: var(--color-manager)">ORIGINALS</h3>
      <div class="form-grid-2">
        <label class="field">Status <input v-model="origForm.originals_status" placeholder="e.g. Sent / Pending" :disabled="isReadOnly" /></label>
        <label class="field">Date Sent <input v-model="origForm.originals_sent" type="date" :disabled="isReadOnly" /></label>
      </div>
      <div class="field" style="margin-top: var(--space-sm)">
        <label style="font-size: var(--text-sm); font-weight: 500; color: var(--text-secondary)">Contact and Sending Description</label>
        <input v-model="origForm.originals_description" type="text" placeholder="e.g. Sent via DHL, tracking #..." :disabled="isReadOnly" />
      </div>
      <div class="section-actions">
        <button class="btn-save" :disabled="savingOriginals || isReadOnly" @click="saveOriginals">{{ savingOriginals ? 'Saving...' : 'Save Originals' }}</button>
      </div>
    </section>

    <!-- TELEX -->
    <section class="check-section telex-section">
      <h3 class="section-title" style="color: var(--color-telex)">TELEX RELEASE</h3>
      <label class="bool-item telex-item" style="--accent: var(--color-telex)">
        <input type="checkbox"
          :checked="store.selected.telex_released"
          @change="handleTelexToggle"
          :disabled="store.selected.telex_released && !auth.isAdmin"
        />
        <span :class="{ done: store.selected.telex_released }">{{ store.selected.telex_released ? '✓ Telex Released' : 'Release Telex' }}</span>
      </label>
      <p v-if="store.selected.telex_released" class="telex-done">This shipment has been telex released. No further changes.</p>
    </section>

    <!-- Modal -->
    <div v-if="showTelexConfirm" class="modal-overlay" @click.self="showTelexConfirm = false">
      <div class="modal-box">
        <h3>⚠ Release Telex?</h3>
        <p>This action is final. The shipment will be marked as TELEX RELEASED and cannot be modified further.</p>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showTelexConfirm = false">Cancel</button>
          <button class="btn-danger" @click="confirmTelex">Release Telex</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.checklist { display: flex; flex-direction: column; gap: var(--space-md); }
.step-header { display: flex; align-items: center; gap: var(--space-md); padding: var(--space-md) var(--space-lg); background: var(--bg-surface); border-radius: var(--radius-md); }
.step-header h2 { flex: 1; font-size: var(--text-lg); font-weight: 700; }
.locked-badge { font-size: var(--text-xs); font-weight: 600; color: var(--text-secondary); }
.check-section { background: var(--bg-card); border-radius: var(--radius-md); padding: var(--space-md) var(--space-lg); border-left: 4px solid var(--border-color); box-shadow: var(--shadow-sm); }
.section-title { font-size: var(--text-sm); font-weight: 700; margin-bottom: var(--space-sm); text-transform: uppercase; letter-spacing: 0.5px; }
.bool-grid { display: flex; gap: var(--space-lg); flex-wrap: wrap; }
.bool-item { display: flex; align-items: center; gap: var(--space-sm); font-size: var(--text-base); cursor: pointer; user-select: none; }
.bool-item input[type="checkbox"] { width: 18px; height: 18px; accent-color: var(--accent); cursor: pointer; }
.bool-item input:disabled { cursor: not-allowed; opacity: 0.6; }
.form-grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-md); }
.form-grid-3 { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: var(--space-md); }
.field { display: flex; flex-direction: column; gap: var(--space-xs); font-size: var(--text-sm); font-weight: 500; color: var(--text-secondary); }
.field input { padding: var(--space-sm); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-base); color: var(--text-primary); }
.field input:focus { outline: none; border-color: var(--color-manager); }
.field input:disabled { opacity: 0.6; background: var(--border-light); cursor: not-allowed; }
.section-actions { margin-top: var(--space-sm); display: flex; justify-content: flex-end; }
.btn-save { background: var(--color-manager); color: var(--text-inverse); border: none; padding: var(--space-xs) var(--space-md); border-radius: var(--radius-sm); font-size: var(--text-sm); font-weight: 600; cursor: pointer; }
.btn-save:hover { opacity: 0.9; }
.btn-save:disabled { opacity: 0.5; }
.telex-section { border-left-color: var(--color-telex); }
.telex-item { font-size: var(--text-lg); font-weight: 600; }
.telex-item .done { color: var(--color-telex); }
.telex-done { margin-top: var(--space-sm); font-size: var(--text-sm); color: var(--color-telex); font-weight: 500; }
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 200; }
.modal-box { background: var(--bg-card); border-radius: var(--radius-md); padding: var(--space-xl); max-width: 420px; box-shadow: var(--shadow-lg); }
.modal-box h3 { margin-bottom: var(--space-sm); }
.modal-box p { font-size: var(--text-sm); color: var(--text-secondary); margin-bottom: var(--space-lg); }
.modal-actions { display: flex; gap: var(--space-sm); justify-content: flex-end; }
.btn-cancel { background: var(--border-color); color: var(--text-primary); border: none; padding: var(--space-sm) var(--space-lg); border-radius: var(--radius-sm); font-size: var(--text-sm); font-weight: 600; cursor: pointer; }
.btn-danger { background: var(--color-admin); color: var(--text-inverse); border: none; padding: var(--space-sm) var(--space-lg); border-radius: var(--radius-sm); font-size: var(--text-sm); font-weight: 600; cursor: pointer; }
</style>
