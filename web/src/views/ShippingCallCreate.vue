<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useShippingCallsStore } from '@/stores/shipping-calls';

const router = useRouter();
const store = useShippingCallsStore();

const form = ref({
  buyer_name: '',
  incoterms: 'FOB',
  product_description: '',
  total_containers: 0,
  sc_po_id: '',
  sc_po_date: '',
  sc_po_by: '',
});

const warehouses = ref<{ warehouse_name: string; planned_containers: number }[]>([]);
const newWh = ref({ name: '', count: 0 });

const saving = ref(false);
const error = ref('');

function addWarehouse() {
  if (!newWh.value.name || newWh.value.count < 1) return;
  warehouses.value.push({ warehouse_name: newWh.value.name, planned_containers: newWh.value.count });
  newWh.value = { name: '', count: 0 };
  form.value.total_containers = warehouses.value.reduce((s, w) => s + w.planned_containers, 0);
}

function removeWarehouse(idx: number) {
  warehouses.value.splice(idx, 1);
  form.value.total_containers = warehouses.value.reduce((s, w) => s + w.planned_containers, 0);
}

async function handleSubmit() {
  if (!form.value.buyer_name || !form.value.incoterms) {
    error.value = 'Buyer and Incoterms are required';
    return;
  }
  saving.value = true;
  error.value = '';
  const result = await store.create({
    buyer_name: form.value.buyer_name,
    incoterms: form.value.incoterms,
    product_description: form.value.product_description || undefined,
    total_containers: form.value.total_containers,
    sc_po_id: form.value.sc_po_id || undefined,
    sc_po_date: form.value.sc_po_date || undefined,
    sc_po_by: form.value.sc_po_by || undefined,
    warehouses: warehouses.value.length ? warehouses.value : undefined,
  });
  saving.value = false;
  if (result) router.push(`/calls/${result.id}`);
  else error.value = store.error || 'Failed to create call';
}
</script>

<template>
  <div class="create-page">
    <button class="back-btn" @click="router.push('/calls')">← Back to Calls</button>
    <h1>New Shipping Call</h1>

    <form class="create-form" @submit.prevent="handleSubmit">
      <!-- Call Header -->
      <div class="form-section">
        <h2>Call Details</h2>
        <div class="form-grid">
          <label class="field">
            Buyer *
            <input v-model="form.buyer_name" required placeholder="e.g. Kotor" />
          </label>
          <label class="field">
            Incoterms *
            <select v-model="form.incoterms">
              <option>FOB</option><option>CIF</option><option>CFR</option><option>EXW</option>
              <option>FCA</option><option>FAS</option><option>CPT</option><option>CIP</option>
              <option>DAP</option><option>DPU</option><option>DDP</option>
            </select>
          </label>
          <label class="field full">
            Product Description
            <input v-model="form.product_description" placeholder="e.g. Cassava Starch" />
          </label>
          <label class="field">
            SC/PO ID
            <input v-model="form.sc_po_id" placeholder="e.g. #42" />
          </label>
          <label class="field">
            SC/PO Date
            <input v-model="form.sc_po_date" type="date" />
          </label>
          <label class="field">
            SC/PO Made By
            <input v-model="form.sc_po_by" placeholder="e.g. Tuan" />
          </label>
        </div>
      </div>

      <!-- Warehouses -->
      <div class="form-section">
        <h2>Warehouse Loading Plan</h2>
        <div v-if="warehouses.length" class="wh-list">
          <div v-for="(w, i) in warehouses" :key="i" class="wh-item">
            <span>{{ w.warehouse_name }}</span>
            <span>{{ w.planned_containers }} containers</span>
            <button type="button" class="btn-remove" @click="removeWarehouse(i)">✕</button>
          </div>
        </div>
        <div class="add-wh">
          <input v-model="newWh.name" placeholder="Warehouse name" class="wh-input" />
          <input v-model.number="newWh.count" type="number" min="1" placeholder="Containers" class="wh-input-num" />
          <button type="button" class="btn-add" @click="addWarehouse">+ Add</button>
        </div>
        <div class="total-line">
          Total containers: <strong>{{ form.total_containers }}</strong>
        </div>
      </div>

      <div v-if="error" class="error-msg">{{ error }}</div>

      <div class="form-actions">
        <button type="button" class="btn-cancel" @click="router.push('/calls')">Cancel</button>
        <button type="submit" class="btn-submit" :disabled="saving">
          {{ saving ? 'Creating...' : 'Create Shipping Call' }}
        </button>
      </div>
    </form>
  </div>
</template>

<style scoped>
.create-page { padding: var(--space-xl); max-width: 700px; }
.back-btn { background: none; border: none; color: var(--color-manager); cursor: pointer; font-weight: 600; margin-bottom: var(--space-md); }
h1 { font-size: var(--text-2xl); font-weight: 700; margin-bottom: var(--space-lg); }

.form-section { background: var(--bg-card); border-radius: var(--radius-md); padding: var(--space-lg); margin-bottom: var(--space-md); box-shadow: var(--shadow-sm); }
.form-section h2 { font-size: var(--text-base); font-weight: 700; margin-bottom: var(--space-md); }
.form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-md); }
.field { display: flex; flex-direction: column; gap: var(--space-xs); font-size: var(--text-sm); font-weight: 500; color: var(--text-secondary); }
.field.full { grid-column: 1 / -1; }
.field input, .field select { padding: var(--space-sm); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-base); }

.wh-list { display: flex; flex-direction: column; gap: var(--space-xs); margin-bottom: var(--space-sm); }
.wh-item { display: flex; align-items: center; gap: var(--space-md); font-size: var(--text-sm); padding: var(--space-xs) var(--space-sm); background: var(--bg-surface); border-radius: var(--radius-sm); }
.wh-item span:first-child { flex: 1; font-weight: 600; }
.wh-item span:last-child { color: var(--text-secondary); }
.add-wh { display: flex; gap: var(--space-sm); }
.wh-input { flex: 1; padding: var(--space-xs); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-sm); }
.wh-input-num { width: 80px; padding: var(--space-xs); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-sm); }
.btn-add, .btn-remove { background: none; border: none; cursor: pointer; font-weight: 600; font-size: var(--text-sm); }
.btn-add { color: var(--color-manager); }
.btn-remove { color: var(--color-admin); }
.total-line { margin-top: var(--space-sm); font-size: var(--text-sm); color: var(--text-secondary); }

.error-msg { background: #FDF0ED; color: #E74C3C; padding: var(--space-sm); border-radius: var(--radius-sm); font-size: var(--text-sm); margin-bottom: var(--space-md); }
.form-actions { display: flex; gap: var(--space-md); justify-content: flex-end; }
.btn-cancel { background: var(--border-color); border: none; padding: var(--space-sm) var(--space-xl); border-radius: var(--radius-sm); font-weight: 600; cursor: pointer; }
.btn-submit { background: var(--color-manager); color: var(--text-inverse); border: none; padding: var(--space-sm) var(--space-xl); border-radius: var(--radius-sm); font-weight: 600; cursor: pointer; }
.btn-submit:disabled { opacity: 0.5; }
</style>
