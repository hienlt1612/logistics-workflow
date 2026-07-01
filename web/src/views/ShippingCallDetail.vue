<script setup lang="ts">
import { onMounted, computed, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useShippingCallsStore } from '@/stores/shipping-calls';
import { useShipmentsStore } from '@/stores/shipments';
import * as api from '@/api/client';

const route = useRoute();
const router = useRouter();
const callStore = useShippingCallsStore();
const shipStore = useShipmentsStore();

const callId = computed(() => Number(route.params.id));
const showAddContainer = ref<Record<number, boolean>>({});
const newContainer = ref({
  container_number: '', seal_number: '', warehouse_name: '',
  weight_kg: '', cbm: '',
});

onMounted(async () => {
  await callStore.loadOne(callId.value);
  await shipStore.loadAll();
});

const callShipments = computed(() =>
  shipStore.shipments.filter(s => s.shipping_call_id == callId.value)
);

async function addContainer(shipmentId: number) {
  await api.createContainer({
    shipment_id: shipmentId,
    container_number: newContainer.value.container_number,
    seal_number: newContainer.value.seal_number || undefined,
    warehouse_name: newContainer.value.warehouse_name || undefined,
    weight_kg: newContainer.value.weight_kg || undefined,
    cbm: newContainer.value.cbm || undefined,
  });
  newContainer.value = { container_number: '', seal_number: '', warehouse_name: '', weight_kg: '', cbm: '' };
  showAddContainer.value[shipmentId] = false;
  await callStore.loadOne(callId.value);
}

const containerMap = ref<Record<number, api.Container[]>>({});

async function loadContainers(shipmentId: number) {
  containerMap.value[shipmentId] = await api.fetchContainers(shipmentId);
}

const incotermsMap: Record<string, string> = {
  FOB: 'FOB', CIF: 'CIF', CFR: 'CFR', EXW: 'EXW',
  FCA: 'FCA', FAS: 'FAS', CPT: 'CPT', CIP: 'CIP',
  DAP: 'DAP', DPU: 'DPU', DDP: 'DDP',
};

const loadedTotal = computed(() =>
  callStore.warehouses.reduce((sum, w) => sum + w.loaded_containers, 0)
);

const plannedTotal = computed(() =>
  callStore.warehouses.reduce((sum, w) => sum + w.planned_containers, 0)
);
</script>

<template>
  <div class="detail-page" v-if="callStore.selected">
    <button class="back-btn" @click="router.push('/calls')">← Back to Calls</button>

    <!-- Header -->
    <div class="call-header">
      <div class="header-main">
        <h1>{{ callStore.selected.call_ref }}</h1>
        <span class="status-badge" :class="callStore.selected.status.toLowerCase()">{{ callStore.selected.status }}</span>
      </div>
      <div class="header-meta">
        <div><strong>{{ callStore.selected.buyer_name }}</strong> · {{ incotermsMap[callStore.selected.incoterms] }}</div>
        <div v-if="callStore.selected.product_description">{{ callStore.selected.product_description }}</div>
        <div>{{ loadedTotal }} / {{ callStore.selected.total_containers }} containers loaded</div>
      </div>
    </div>

    <!-- Warehouse Progress -->
    <section class="section">
      <h2>Warehouse Loading Plan</h2>
      <div v-if="callStore.warehouses.length" class="wh-grid">
        <div v-for="w in callStore.warehouses" :key="w.id" class="wh-row">
          <span class="wh-name">{{ w.warehouse_name }}</span>
          <div class="wh-bar-wrap">
            <div class="wh-bar" :style="{ width: w.planned_containers ? (w.loaded_containers / w.planned_containers * 100) + '%' : '0%' }" />
          </div>
          <span class="wh-count">{{ w.loaded_containers }} / {{ w.planned_containers }}</span>
        </div>
      </div>
    </section>

    <!-- Bookings (Shipments) -->
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

            <!-- Containers -->
            <div class="container-section">
              <h3>Containers
                <button class="btn-sm" @click="showAddContainer[s.id] = !showAddContainer[s.id]; if (!containerMap[s.id]) loadContainers(s.id)">
                  {{ showAddContainer[s.id] ? 'Cancel' : '+ Add' }}
                </button>
              </h3>

              <div v-if="containerMap[s.id]?.length" class="container-grid">
                <div v-for="c in containerMap[s.id]" :key="c.id" class="container-row">
                  <span class="ctn-num">{{ c.container_number }}</span>
                  <span>{{ c.seal_number ?? '—' }}</span>
                  <span>{{ c.warehouse_name ?? '—' }}</span>
                  <span>{{ c.weight_kg ?? '—' }} kg</span>
                  <span>{{ c.cbm ?? '—' }} cbm</span>
                </div>
              </div>
              <div v-else-if="containerMap[s.id]">No containers yet.</div>

              <!-- Add Container Form -->
              <div v-if="showAddContainer[s.id]" class="add-ctn-form">
                <input v-model="newContainer.container_number" placeholder="Container #" class="ctn-input" />
                <input v-model="newContainer.seal_number" placeholder="Seal #" class="ctn-input" />
                <input v-model="newContainer.warehouse_name" placeholder="Warehouse" class="ctn-input" />
                <input v-model="newContainer.weight_kg" placeholder="Weight kg" type="number" class="ctn-input" />
                <input v-model="newContainer.cbm" placeholder="CBM" type="number" step="0.001" class="ctn-input" />
                <button class="btn-save" @click="addContainer(s.id)">Add Container</button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div v-else class="empty">No bookings linked to this call. Create a shipment with this shipping call.</div>
    </section>
  </div>

  <div v-else-if="callStore.loading" class="loading">Loading...</div>
</template>

<style scoped>
.detail-page { padding: var(--space-xl); max-width: 1000px; }
.back-btn { background: none; border: none; color: var(--color-manager); cursor: pointer; font-weight: 600; margin-bottom: var(--space-md); }

.call-header { background: var(--bg-card); border-radius: var(--radius-md); padding: var(--space-lg); box-shadow: var(--shadow-sm); margin-bottom: var(--space-lg); }
.header-main { display: flex; gap: var(--space-md); align-items: center; margin-bottom: var(--space-sm); }
.header-main h1 { font-family: var(--font-mono); font-size: var(--text-xl); }
.header-meta { font-size: var(--text-sm); color: var(--text-secondary); line-height: 1.6; }
.status-badge { padding: 2px 10px; border-radius: var(--radius-sm); font-size: var(--text-xs); font-weight: 700; text-transform: uppercase; }
.status-badge.open { background: rgba(52,152,219,0.15); color: var(--color-manager); }
.status-badge.partial { background: rgba(243,156,18,0.15); color: var(--color-accounting); }
.status-badge.complete { background: rgba(39,174,96,0.15); color: var(--color-checklist); }

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

.container-section { margin-top: var(--space-md); border-top: 1px solid var(--border-light); padding-top: var(--space-sm); }
.container-section h3 { font-size: var(--text-sm); font-weight: 600; margin-bottom: var(--space-xs); display: flex; justify-content: space-between; }
.container-grid { display: flex; flex-direction: column; gap: 2px; margin: var(--space-xs) 0; }
.container-row { display: flex; gap: var(--space-md); font-size: var(--text-xs); color: var(--text-secondary); padding: 2px 0; }
.ctn-num { font-family: var(--font-mono); font-weight: 600; color: var(--text-primary); }
.add-ctn-form { display: flex; gap: var(--space-xs); flex-wrap: wrap; margin-top: var(--space-sm); }
.ctn-input { padding: 4px 8px; border: 1px solid var(--border-color); border-radius: var(--radius-sm); font-size: var(--text-xs); width: 100px; }
.btn-sm { background: none; border: none; color: var(--color-manager); font-size: var(--text-xs); cursor: pointer; font-weight: 600; }
.btn-save { background: var(--color-checklist); color: var(--text-inverse); border: none; padding: 4px 12px; border-radius: var(--radius-sm); font-size: var(--text-xs); cursor: pointer; }
.empty { color: var(--text-secondary); font-size: var(--text-sm); padding: var(--space-md); }
.loading { text-align: center; padding: var(--space-2xl); color: var(--text-secondary); }
</style>
