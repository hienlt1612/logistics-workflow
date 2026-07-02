<script setup lang="ts">
import { onMounted, ref, computed, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useShippingCallsStore } from '@/stores/shipping-calls';
import type { Shipment } from '@/api/client';
import * as api from '@/api/client';

const route = useRoute();
const router = useRouter();
const callStore = useShippingCallsStore();
// ponytail: load ALL shipments for this call locally — the paginated shipments
// store (20/page, sticky status filter) hid linked bookings → "No bookings".
const allShips = ref<Shipment[]>([]);

const callId = computed(() => Number(route.params.id));
const editing = ref(false);
const editForm = ref({ buyer_name: '', incoterms: 'FOB', product_description: '', total_containers: 0, sc_po_id: '', sc_po_date: '', sc_po_by: '' });
const editWarehouses = ref<{ warehouse_name: string; planned_containers: number }[]>([]);
const newWh = ref({ name: '', count: 0 });
const saving = ref(false);

async function loadShips() {
  allShips.value = (await api.fetchShipments({ pageSize: 1000 })).data;
}

// ponytail: same-component route change — reload on callId change
async function loadCallDetail() {
  await callStore.loadOne(callId.value);
  await loadShips();
  for (const s of callShipments.value) await loadContainers(s.id);
}

onMounted(() => loadCallDetail());
watch(callId, () => loadCallDetail());

function startEdit() {
  const c = callStore.selected;
  if (!c) return;
  editForm.value = {
    buyer_name: c.buyer_name,
    incoterms: c.incoterms,
    product_description: c.product_description ?? '',
    total_containers: c.total_containers,
    sc_po_id: c.sc_po_id ?? '',
    sc_po_date: c.sc_po_date ?? '',
    sc_po_by: c.sc_po_by ?? '',
  };
  editWarehouses.value = callStore.warehouses.map(w => ({
    warehouse_name: w.warehouse_name,
    planned_containers: w.planned_containers,
  }));
  editing.value = true;
}

// ponytail: auto-computed from warehouse total
const totalContainers = computed(() => editWarehouses.value.reduce((s, w) => s + w.planned_containers, 0));

async function addEditWarehouse() {
  if (!newWh.value.name || newWh.value.count < 1) return;
  editWarehouses.value.push({ warehouse_name: newWh.value.name, planned_containers: newWh.value.count });
  newWh.value = { name: '', count: 0 };
  // ponytail: save warehouse list immediately via PATCH
  await saveWarehouses();
}

function removeEditWarehouse(idx: number) {
  editWarehouses.value.splice(idx, 1);
  void saveWarehouses(); // ponytail: immediate save, fire-and-forget
}

// ponytail: PATCH warehouses atomically without waiting for full Save
async function saveWarehouses() {
  const wh = editWarehouses.value.length ? editWarehouses.value : undefined;
  await callStore.update(callId.value, {
    buyer_name: editForm.value.buyer_name,
    incoterms: editForm.value.incoterms,
    total_containers: totalContainers.value,
    warehouses: wh,
  });
  await callStore.loadOne(callId.value);
}

async function saveEdit() {
  saving.value = true;
  // ponytail: warehouses already saved individually, just save remaining fields
  await callStore.update(callId.value, {
    buyer_name: editForm.value.buyer_name,
    incoterms: editForm.value.incoterms,
    total_containers: totalContainers.value,
    sc_po_id: editForm.value.sc_po_id || undefined,
    sc_po_date: editForm.value.sc_po_date || undefined,
    sc_po_by: editForm.value.sc_po_by || undefined,
    product_description: editForm.value.product_description || undefined,
  });
  saving.value = false;
  editing.value = false;
  await callStore.loadOne(callId.value);
}

const callShipments = computed(() =>
  allShips.value.filter(s => s.shipping_call_id == callId.value)
);

// ── Container add per booking ──
const containerMap = ref<Record<number, api.Container[]>>({});
const newCtn = ref({ container_number: '', seal_number: '', warehouse_name: '', loaded_date: '' });
const editCid = ref<number | null>(null);
const editCtn = ref({ container_number: '', seal_number: '', warehouse_name: '', loaded_date: '' });

async function loadContainers(shipmentId: number) {
  containerMap.value[shipmentId] = await api.fetchContainers(shipmentId);
}

async function addContainer(shipmentId: number) {
  const c = newCtn.value;
  // ponytail: all fields required
  if (!c.container_number) { alert('Container number required.'); return; }
  if (!c.seal_number) { alert('Seal number required.'); return; }
  if (!c.warehouse_name) { alert('Warehouse required.'); return; }
  if (!c.loaded_date) { alert('Loaded date required.'); return; }
  // ponytail: total container cap — sum across all bookings for this call
  const totalContainers = callStore.selected?.total_containers ?? 0;
  let loadedTotal = 0;
  for (const list of Object.values(containerMap.value)) loadedTotal += list.length;
  if (loadedTotal >= totalContainers && totalContainers > 0) {
    alert(`Maximum ${totalContainers} containers reached.`);
    return;
  }
  await api.createContainer({
    shipment_id: shipmentId,
    container_number: newCtn.value.container_number,
    seal_number: newCtn.value.seal_number || undefined,
    warehouse_name: newCtn.value.warehouse_name || undefined,
    loaded_date: newCtn.value.loaded_date || undefined,
  });
  newCtn.value = { container_number: '', seal_number: '', warehouse_name: '', loaded_date: '' };
  await loadContainers(shipmentId);
  await callStore.loadOne(callId.value);
}

async function deleteCtn(container: api.Container, shipmentId: number) {
  if (!confirm(`Delete container ${container.container_number}?`)) return;
  await api.deleteContainer(container.id);
  await loadContainers(shipmentId);
  await callStore.loadOne(callId.value);
}

function startEditCtn(c: api.Container) {
  editCid.value = c.id;
  editCtn.value = {
    container_number: c.container_number,
    seal_number: c.seal_number ?? '',
    warehouse_name: c.warehouse_name ?? '',
    loaded_date: c.loaded_date ?? '',
  };
}

async function saveEditCtn(shipmentId: number) {
  await api.updateContainer(editCid.value!, {
    container_number: editCtn.value.container_number,
    seal_number: editCtn.value.seal_number || undefined,
    warehouse_name: editCtn.value.warehouse_name || undefined,
    loaded_date: editCtn.value.loaded_date || undefined,
  });
  editCid.value = null;
  await loadContainers(shipmentId);
}

async function toggleLoaded(shipmentId: number, value: boolean) {
  await api.toggleChecklist(shipmentId, 'containers_loaded', value);
  await loadShips();
  await loadContainers(shipmentId);
}

// ponytail: all bookings locked → can finish call
const allBookingsLocked = computed(() =>
  callShipments.value.length > 0 && callShipments.value.every(s => s.containers_loaded)
);
// ponytail: CLOSED revert follows auto-sync rule
const revertStatus = computed(() => callShipments.value.length > 0 ? 'ON_LOADING' : 'OPEN');

async function toggleCallClosed(close: boolean) {
  const c = callStore.selected;
  if (!c) return;
  await callStore.update(callId.value, {
    buyer_name: c.buyer_name,
    incoterms: c.incoterms,
    total_containers: c.total_containers,
    status: close ? 'CLOSED' : revertStatus.value,
  });
  await callStore.loadOne(callId.value);
}

async function deleteCall() {
  if (!window.confirm('Delete this shipping call?')) return;
  await api.deleteShippingCall(callStore.selected!.id);
  router.push('/calls');
}
const incotermsMap: Record<string, string> = { FOB: 'FOB', CIF: 'CIF', CFR: 'CFR', EXW: 'EXW', FCA: 'FCA', FAS: 'FAS', CPT: 'CPT', CIP: 'CIP', DAP: 'DAP', DPU: 'DPU', DDP: 'DDP' };
const loadedTotal = computed(() => callStore.warehouses.reduce((sum, w) => sum + w.loaded_containers, 0));
</script>

<template>
  <div class="detail-page" v-if="callStore.selected">
    <button class="back-btn" @click="router.push('/calls')">← Back to Calls</button>

    <div class="call-header">
      <div class="header-main">
        <h1>{{ callStore.selected.call_ref }}</h1>
        <span class="status-badge" :class="callStore.selected.status.toLowerCase()">{{ callStore.selected.status }}</span>
        <button v-if="!editing" class="btn-edit" @click="startEdit">Edit</button>
        <button v-if="!editing && callShipments.length === 0" class="btn-del" @click="deleteCall">Delete</button>
      </div>

      <div v-if="!editing" class="header-meta">
        <div><strong>{{ callStore.selected.buyer_name }}</strong> · {{ incotermsMap[callStore.selected.incoterms] }}</div>
        <div v-if="callStore.selected.product_description">{{ callStore.selected.product_description }}</div>
        <div>{{ loadedTotal }} / {{ callStore.selected.total_containers }} containers loaded</div>
        <div class="meta-extra">
          <span v-if="callStore.selected.sc_po_id">SC/PO: {{ callStore.selected.sc_po_id }}</span>
          <span v-if="callStore.selected.sc_po_date"> · {{ callStore.selected.sc_po_date }}</span>
          <span v-if="callStore.selected.sc_po_by"> · by {{ callStore.selected.sc_po_by }}</span>
        </div>
      </div>

      <div v-else class="edit-form">
        <div class="form-grid">
          <label>Buyer *<input v-model="editForm.buyer_name" /></label>
          <label>Incoterms *
            <select v-model="editForm.incoterms">
              <option v-for="(_, k) in incotermsMap" :key="k" :value="k">{{ k }}</option>
            </select>
          </label>
          <label class="full">Product<input v-model="editForm.product_description" /></label>
          <label>Total Containers<span class="val">{{ totalContainers }} (auto)</span></label>
          <label>SC/PO ID<input v-model="editForm.sc_po_id" /></label>
          <label>SC/PO Date<input v-model="editForm.sc_po_date" type="date" /></label>
          <label>SC/PO By<input v-model="editForm.sc_po_by" /></label>
        </div>
        <div class="edit-actions">
          <button class="btn-cancel" @click="editing = false">Cancel</button>
          <button class="btn-save" @click="saveEdit" :disabled="saving">{{ saving ? 'Saving...' : 'Save' }}</button>
        </div>

        <div class="edit-wh-section">
          <h3>Warehouses</h3>
          <div v-if="editWarehouses.length" class="edit-wh-list">
            <div v-for="(w, i) in editWarehouses" :key="i" class="edit-wh-row">
              <span>{{ w.warehouse_name }} — {{ w.planned_containers }} containers</span>
              <button class="btn-remove" @click="removeEditWarehouse(i)">✕</button>
            </div>
          </div>
          <div class="add-wh-row">
            <input v-model="newWh.name" placeholder="Warehouse name" class="wh-inp" />
            <input v-model.number="newWh.count" type="number" min="1" placeholder="Qty" class="wh-inp-num" />
            <button class="btn-add-wh" @click="addEditWarehouse">+ Add</button>
          </div>
        </div>
      </div>
    </div>

    <section class="section">
      <h2>Warehouse Loading Plan</h2>
      <div v-if="callStore.warehouses.length" class="wh-grid">
        <div v-for="w in callStore.warehouses" :key="w.id" class="wh-row">
          <span class="wh-name">{{ w.warehouse_name }}</span>
          <div class="wh-bar-wrap"><div class="wh-bar" :style="{ width: w.planned_containers ? (w.loaded_containers / w.planned_containers * 100) + '%' : '0%' }" /></div>
          <span class="wh-count">{{ w.loaded_containers }} / {{ w.planned_containers }}</span>
        </div>
      </div>
    </section>

    <section class="section">
      <h2>Bookings</h2>
      <div v-if="callShipments.length" class="booking-list">
        <div v-for="s in callShipments" :key="s.id" class="booking-card">
          <div class="booking-header">
            <span class="booking-ref">{{ s.shipment_ref }}</span>
            <span class="booking-meta">{{ s.shipping_line }} · {{ s.booking_number }}</span>
          </div>
          <div class="booking-body">
            <div>Status: {{ s.status }} · ETD: {{ s.etd ?? '—' }}</div>
            <label class="loaded-check" v-if="containerMap[s.id]">
              {{ containerMap[s.id]?.length ?? 0 }} / {{ callStore.selected?.total_containers ?? '?' }} loaded
              <input type="checkbox" :checked="s.containers_loaded"
                @change="toggleLoaded(s.id, ($event.target as HTMLInputElement).checked)" />
              Mark for locking the loading
            </label>
            <div class="container-section">
              <h3>Containers
                <button class="btn-sm" @click="loadContainers(s.id)">↻</button>
              </h3>
              <div v-if="containerMap[s.id]?.length" class="container-grid">
                <div v-for="c in containerMap[s.id]" :key="c.id" class="container-row">
                  <template v-if="editCid === c.id">
                    <input v-model="editCtn.container_number" class="ctn-inp-edit" :disabled="s.containers_loaded" />
                    <input v-model="editCtn.seal_number" placeholder="Seal" class="ctn-inp-edit" :disabled="s.containers_loaded" />
                    <select v-model="editCtn.warehouse_name" class="ctn-inp-edit" :disabled="s.containers_loaded">
                      <option value="">— WH —</option>
                      <option v-for="w in callStore.warehouses" :key="w.id" :value="w.warehouse_name">{{ w.warehouse_name }}</option>
                    </select>
                    <input v-model="editCtn.loaded_date" type="date" class="ctn-inp-edit" :disabled="s.containers_loaded" />
                    <template v-if="!s.containers_loaded">
                      <button class="btn-save-sm" @click="saveEditCtn(s.id)">✓</button>
                      <button class="btn-cancel-sm" @click="editCid = null">✕</button>
                    </template>
                  </template>
                  <template v-else>
                    <span class="ctn-num">{{ c.container_number }}</span>
                    <span>{{ c.seal_number ?? '—' }}</span>
                    <span>{{ c.warehouse_name ?? '—' }}</span>
                    <span class="ctn-date">{{ c.loaded_date ?? '—' }}</span>
                    <button v-if="!s.containers_loaded" class="btn-edit-sm" @click="startEditCtn(c)">✎</button>
                    <button v-if="!s.containers_loaded" class="btn-del-sm" @click="deleteCtn(c, s.id)">✕</button>
                  </template>
                </div>
              </div>
              <div v-else-if="!containerMap[s.id]" class="ctn-empty">Click ↻ to load containers</div>
              <div v-else class="ctn-empty">No containers yet.</div>

              <!-- ponytail: same add pattern as warehouse edit -->
              <div v-if="!s.containers_loaded" class="add-ctn-row">
                <input v-model="newCtn.container_number" placeholder="Container #" class="ctn-inp" />
                <input v-model="newCtn.seal_number" placeholder="Seal #" class="ctn-inp" />
                <select v-model="newCtn.warehouse_name" class="ctn-inp">
                  <option value="">— Warehouse —</option>
                  <option v-for="w in callStore.warehouses" :key="w.id" :value="w.warehouse_name">{{ w.warehouse_name }}</option>
                </select>
                <input v-model="newCtn.loaded_date" type="date" class="ctn-inp" title="Date loaded" />
                <button class="btn-add-sm" @click="addContainer(s.id)">+ Add</button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div v-else class="empty">No bookings linked to this call.</div>

      <!-- ponytail: finish call checkbox when all bookings locked -->
      <div v-if="allBookingsLocked" class="finish-section">
        <label class="loaded-check">
          <input type="checkbox" :checked="callStore.selected?.status === 'CLOSED'"
            @change="toggleCallClosed(($event.target as HTMLInputElement).checked)" />
          Finish the shipping call
        </label>
      </div>
    </section>
  </div>
  <div v-else-if="callStore.loading" class="loading">Loading...</div>
  <div v-else class="loading">Call not found.</div>
</template>

<style scoped>
.detail-page { padding: var(--space-xl); max-width: 1000px; }
.back-btn { background: none; border: none; color: var(--color-manager); cursor: pointer; font-weight: 600; margin-bottom: var(--space-md); }
.call-header { background: var(--bg-card); border-radius: var(--radius-md); padding: var(--space-lg); box-shadow: var(--shadow-sm); margin-bottom: var(--space-lg); }
.header-main { display: flex; gap: var(--space-md); align-items: center; margin-bottom: var(--space-sm); }
.header-main h1 { font-family: var(--font-mono); font-size: var(--text-xl); }
.header-meta { font-size: var(--text-sm); color: var(--text-secondary); line-height: 1.6; }
.meta-extra { font-size: var(--text-xs); color: var(--text-secondary); margin-top: var(--space-xs); }
.status-badge { padding: 2px 10px; border-radius: var(--radius-sm); font-size: var(--text-xs); font-weight: 700; text-transform: uppercase; }
.status-badge.open { background: rgba(52,152,219,0.15); color: var(--color-manager); }
.status-badge.on_loading { background: rgba(39,174,96,0.15); color: var(--color-checklist); }
.status-badge.partial { background: rgba(243,156,18,0.15); color: var(--color-accounting); }
.status-badge.complete { background: rgba(39,174,96,0.15); color: var(--color-checklist); }
.status-badge.closed { background: rgba(149,165,166,0.15); color: #7f8c8d; }
.btn-edit { margin-left: auto; background: var(--color-manager); color: var(--text-inverse); border: none; padding: var(--space-xs) var(--space-md); border-radius: var(--radius-sm); font-weight: 600; cursor: pointer; font-size: var(--text-sm); }
.btn-edit:hover { opacity: 0.9; }
.btn-del { background: var(--color-admin); color: var(--text-inverse); border: none; padding: var(--space-xs) var(--space-md); border-radius: var(--radius-sm); font-weight: 600; cursor: pointer; font-size: var(--text-sm); }
.btn-del:hover { opacity: 0.9; }
.edit-form { margin-top: var(--space-md); border-top: 1px solid var(--border-light); padding-top: var(--space-md); }
.form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-md); margin-bottom: var(--space-md); }
.form-grid label { display: flex; flex-direction: column; gap: var(--space-xs); font-size: var(--text-sm); font-weight: 500; color: var(--text-secondary); }
.form-grid label.full { grid-column: 1 / -1; }
.form-grid input, .form-grid select { padding: var(--space-sm); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-base); }
.edit-actions { display: flex; gap: var(--space-md); justify-content: flex-end; }
.btn-cancel { background: var(--border-color); border: none; padding: var(--space-sm) var(--space-lg); border-radius: var(--radius-sm); font-weight: 600; cursor: pointer; }
.btn-save { background: var(--color-checklist); color: var(--text-inverse); border: none; padding: var(--space-sm) var(--space-lg); border-radius: var(--radius-sm); font-weight: 600; cursor: pointer; }
.btn-save:disabled { opacity: 0.5; }

.edit-wh-section { margin-top: var(--space-md); border-top: 1px solid var(--border-light); padding-top: var(--space-md); }
.edit-wh-section h3 { font-size: var(--text-sm); font-weight: 600; margin-bottom: var(--space-sm); }
.edit-wh-list { display: flex; flex-direction: column; gap: var(--space-xs); margin-bottom: var(--space-sm); }
.edit-wh-row { display: flex; align-items: center; gap: var(--space-md); font-size: var(--text-sm); padding: var(--space-xs) var(--space-sm); background: var(--bg-surface); border-radius: var(--radius-sm); }
.edit-wh-row span { flex: 1; }
.add-wh-row { display: flex; gap: var(--space-sm); }
.wh-inp { flex: 1; padding: var(--space-xs); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-sm); }
.wh-inp-num { width: 70px; padding: var(--space-xs); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-sm); }
.btn-add-wh, .btn-remove { background: none; border: none; cursor: pointer; font-weight: 600; font-size: var(--text-sm); }
.btn-add-wh { color: var(--color-manager); }
.btn-remove { color: var(--color-admin); }

.section { margin-bottom: var(--space-lg); }
.section h2 { font-size: var(--text-lg); font-weight: 700; margin-bottom: var(--space-md); }
.wh-grid { display: flex; flex-direction: column; gap: var(--space-sm); }
.wh-row { display: flex; align-items: center; gap: var(--space-md); background: var(--bg-card); padding: var(--space-sm) var(--space-md); border-radius: var(--radius-sm); }
.wh-name { width: 140px; font-weight: 600; font-size: var(--text-sm); }
.wh-bar-wrap { flex: 1; height: 8px; background: var(--border-light); border-radius: 4px; overflow: hidden; }
.wh-bar { height: 100%; background: var(--color-checklist); border-radius: 4px; transition: width 0.3s; }
.wh-count { font-size: var(--text-xs); color: var(--text-secondary); min-width: 60px; text-align: right; }

.booking-list { display: flex; flex-direction: column; gap: var(--space-md); }
.booking-card { background: var(--bg-card); border-radius: var(--radius-md); padding: var(--space-md); box-shadow: var(--shadow-sm); }
.booking-header { display: flex; justify-content: space-between; margin-bottom: var(--space-sm); }
.booking-ref { font-family: var(--font-mono); font-weight: 700; }
.booking-meta { font-size: var(--text-sm); color: var(--text-secondary); }
.booking-body { font-size: var(--text-sm); color: var(--text-primary); }
.loaded-info { font-size: var(--text-xs); color: var(--text-secondary); }
.loaded-check { display: flex; align-items: center; gap: var(--space-xs); font-size: var(--text-xs); color: var(--text-secondary); margin: var(--space-xs) 0; cursor: pointer; }
.loaded-check input { cursor: pointer; }
.loaded-ok { color: var(--color-checklist); font-weight: 700; }

.container-section { margin-top: var(--space-md); border-top: 1px solid var(--border-light); padding-top: var(--space-sm); }
.container-section h3 { font-size: var(--text-sm); font-weight: 600; margin-bottom: var(--space-xs); display: flex; justify-content: space-between; }
.container-grid { display: flex; flex-direction: column; gap: 2px; margin: var(--space-xs) 0; }
.container-row { display: flex; gap: var(--space-md); font-size: var(--text-xs); color: var(--text-secondary); padding: 2px 0; }
.ctn-num { font-family: var(--font-mono); font-weight: 600; color: var(--text-primary); }
.ctn-empty { color: var(--text-secondary); font-size: var(--text-xs); padding: var(--space-xs) 0; }

/* ponytail: container add row — same pattern as add-wh-row */
.add-ctn-row { display: flex; gap: var(--space-sm); margin-top: var(--space-sm); flex-wrap: wrap; }
.ctn-inp { padding: var(--space-xs); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-xs); width: 110px; }
.ctn-inp-num { padding: var(--space-xs); border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-xs); width: 70px; }
.btn-sm { background: none; border: none; color: var(--text-secondary); font-size: var(--text-xs); cursor: pointer; }
.btn-add-sm { background: var(--color-checklist); color: var(--text-inverse); border: none; padding: 4px 12px; border-radius: var(--radius-sm); font-size: var(--text-xs); cursor: pointer; white-space: nowrap; }

/* container edit/delete buttons */
.ctn-inp-edit { padding: 2px 6px; border: 1px solid var(--border-color); border-radius: 3px; font-size: var(--text-xs); width: 90px; }
.ctn-inp-num-edit { padding: 2px 6px; border: 1px solid var(--border-color); border-radius: 3px; font-size: var(--text-xs); width: 55px; }
.btn-save-sm { background: var(--color-checklist); color: var(--text-inverse); border: none; padding: 2px 8px; border-radius: 3px; font-size: var(--text-xs); cursor: pointer; }
.btn-cancel-sm { background: none; border: none; color: var(--color-admin); font-size: var(--text-xs); cursor: pointer; font-weight: 700; }
.btn-edit-sm, .btn-del-sm { background: none; border: none; cursor: pointer; font-size: var(--text-xs); padding: 0 2px; }
.btn-edit-sm { color: var(--color-manager); }
.btn-del-sm { color: var(--color-admin); }

.empty { color: var(--text-secondary); font-size: var(--text-sm); padding: var(--space-md); }
.finish-section { border-top: 1px solid var(--border-light); padding: var(--space-md); margin-top: var(--space-sm); }
.loading { text-align: center; padding: var(--space-2xl); color: var(--text-secondary); }
</style>
