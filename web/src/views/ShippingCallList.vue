<script setup lang="ts">
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useShippingCallsStore } from '@/stores/shipping-calls';

const router = useRouter();
const store = useShippingCallsStore();

onMounted(() => store.loadAll());

function fmtDate(iso: string): string {
  return new Date(iso).toLocaleDateString();
}

const incotermsMap: Record<string, string> = {
  FOB: 'FOB', CIF: 'CIF', CFR: 'CFR', EXW: 'EXW',
  FCA: 'FCA', FAS: 'FAS', CPT: 'CPT', CIP: 'CIP',
  DAP: 'DAP', DPU: 'DPU', DDP: 'DDP',
};
</script>

<template>
  <div class="calls-page">
    <div class="page-header">
      <h1>Shipping Calls</h1>
      <button class="btn-new" @click="router.push('/calls/new')">+ New Shipping Call</button>
    </div>

    <div v-if="store.loading" class="loading">Loading...</div>

    <div v-else-if="!store.calls.length" class="empty">
      <p>No shipping calls yet.</p>
      <button class="btn-new" @click="router.push('/calls/new')">Create your first shipping call</button>
    </div>

    <div v-else class="card-grid">
      <div
        v-for="c in store.calls"
        :key="c.id"
        class="call-card"
        @click="router.push(`/calls/${c.id}`)"
      >
        <div class="card-header">
          <span class="call-ref">{{ c.call_ref }}</span>
          <span class="status-badge" :class="c.status.toLowerCase()">{{ c.status }}</span>
        </div>
        <div class="card-body">
          <div><strong>{{ c.buyer_name }}</strong></div>
          <div class="meta">{{ incotermsMap[c.incoterms] ?? c.incoterms }} · {{ c.total_containers }} containers</div>
          <div class="meta" v-if="c.product_description">{{ c.product_description }}</div>
        </div>
        <div class="card-footer">
          <span class="date">{{ fmtDate(c.created_at) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.calls-page { padding: var(--space-xl); max-width: 1000px; }
.page-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-lg); }
.page-header h1 { font-size: var(--text-2xl); font-weight: 700; }
.btn-new { background: var(--color-manager); color: var(--text-inverse); border: none; padding: var(--space-sm) var(--space-lg); border-radius: var(--radius-sm); font-weight: 600; cursor: pointer; }
.btn-new:hover { opacity: 0.9; }
.loading, .empty { text-align: center; padding: var(--space-2xl); color: var(--text-secondary); }
.empty p { margin-bottom: var(--space-md); }
.card-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: var(--space-md); }
.call-card { background: var(--bg-card); border-radius: var(--radius-md); box-shadow: var(--shadow-sm); padding: var(--space-md); cursor: pointer; transition: box-shadow 0.2s; }
.call-card:hover { box-shadow: var(--shadow-md); }
.card-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--space-sm); }
.call-ref { font-family: var(--font-mono); font-size: var(--text-base); font-weight: 700; }
.status-badge { padding: 2px 10px; border-radius: var(--radius-sm); font-size: var(--text-xs); font-weight: 700; text-transform: uppercase; }
.status-badge.open { background: rgba(52,152,219,0.15); color: var(--color-manager); }
.status-badge.partial { background: rgba(243,156,18,0.15); color: var(--color-accounting); }
.status-badge.complete { background: rgba(39,174,96,0.15); color: var(--color-checklist); }
.card-body { margin-bottom: var(--space-sm); font-size: var(--text-sm); color: var(--text-primary); }
.meta { color: var(--text-secondary); font-size: var(--text-xs); margin-top: 2px; }
.card-footer { border-top: 1px solid var(--border-light); padding-top: var(--space-xs); }
.date { font-size: var(--text-xs); color: var(--text-secondary); }
</style>
