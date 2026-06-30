<script setup lang="ts">
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useShipmentsStore } from '@/stores/shipments';
import { useAuthStore } from '@/stores/auth';
import StatusBadge from '@/components/shared/StatusBadge.vue';

const store = useShipmentsStore();
const auth = useAuthStore();
const router = useRouter();
const search = ref('');
const statusFilter = ref('');
const selectedIds = ref<Set<number>>(new Set());
const batchStatus = ref('');

const statusOptions = [
  { value: '', label: 'All Statuses' },
  { value: 'DRAFT', label: 'Draft' },
  { value: 'DOCUMENTS_READY', label: 'Documents Ready' },
  { value: 'CUSTOMS_CLEARED', label: 'Customs Cleared' },
  { value: 'CHECKLIST_IN_PROGRESS', label: 'Checklist' },
  { value: 'TELEX_RELEASED', label: 'Telex Released' },
];

const batchStatusOptions = [
  { value: '', label: '— Set status —' },
  ...statusOptions.filter(o => o.value !== ''),
];

const filtered = computed(() => {
  const q = search.value.toLowerCase();
  let list = store.shipments;
  if (q) {
    list = list.filter(
      (s) =>
        s.shipment_ref.toLowerCase().includes(q) ||
        (s.buyer_name ?? '').toLowerCase().includes(q)
    );
  }
  if (statusFilter.value) {
    list = list.filter((s) => s.status === statusFilter.value);
  }
  return list;
});

const selectCount = computed(() => selectedIds.value.size);

const allSelected = computed(() =>
  filtered.value.length > 0 && filtered.value.every(s => selectedIds.value.has(s.id))
);

function toggleSelect(id: number) {
  const next = new Set(selectedIds.value);
  if (next.has(id)) next.delete(id);
  else next.add(id);
  selectedIds.value = next;
}

function toggleSelectAll() {
  if (allSelected.value) {
    selectedIds.value = new Set();
  } else {
    selectedIds.value = new Set(filtered.value.map(s => s.id));
  }
}

async function handleStatusChange(status: string) {
  statusFilter.value = status;
  selectedIds.value = new Set();
  await store.loadAll(status || undefined);
}

async function handleNew() {
  await store.create({});
  await store.loadAll();
}

async function handleDelete(id: number, ref: string) {
  if (!confirm(`Delete ${ref}? This cannot be undone.`)) return;
  await store.remove(id);
  // Clean up selection
  const next = new Set(selectedIds.value);
  next.delete(id);
  selectedIds.value = next;
}

function goDetail(id: number, event: Event) {
  event.stopPropagation();
  router.push(`/shipment/${id}`);
}

async function handleBatchApply() {
  if (!batchStatus.value || selectedIds.value.size === 0) return;
  const count = await store.batchAdvance([...selectedIds.value], batchStatus.value);
  if (count > 0) {
    selectedIds.value = new Set();
    batchStatus.value = '';
  }
}
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-actions">
      <button class="btn-new" @click="handleNew" :disabled="store.loading">
        + New Shipment
      </button>
      <select
        class="status-filter"
        :value="statusFilter"
        @change="handleStatusChange(($event.target as HTMLSelectElement).value)"
      >
        <option v-for="opt in statusOptions" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
      <input
        v-model="search"
        type="search"
        placeholder="Search ref or buyer..."
        class="search-input"
      />
    </div>

    <!-- Batch action bar -->
    <div v-if="selectCount > 0" class="batch-bar">
      <span class="batch-count">{{ selectCount }} selected</span>
      <select v-model="batchStatus" class="batch-status-select">
        <option v-for="opt in batchStatusOptions" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
      <button
        class="batch-apply-btn"
        :disabled="!batchStatus || store.loading"
        @click="handleBatchApply"
      >
        Apply
      </button>
    </div>

    <div class="shipment-list" v-if="filtered.length">
      <!-- Select All row -->
      <label class="select-all-row">
        <input
          type="checkbox"
          :checked="allSelected"
          @change="toggleSelectAll"
        />
        <span v-if="allSelected">Deselect All</span>
        <span v-else>Select All ({{ filtered.length }})</span>
      </label>

      <div
        v-for="s in filtered"
        :key="s.id"
        class="shipment-row"
        :class="{ active: s.id === store.selectedId }"
      >
        <div class="row-main">
          <input
            type="checkbox"
            class="row-check"
            :checked="selectedIds.has(s.id)"
            @click.stop
            @change="toggleSelect(s.id)"
          />
          <a
            class="ref"
            :href="`/shipment/${s.id}`"
            @click.prevent="goDetail(s.id, $event)"
            :title="`View details for ${s.shipment_ref}`"
            @dblclick.stop
          >{{ s.shipment_ref }}</a>
          <StatusBadge :label="s.status" size="sm" />
          <button
            v-if="auth.isAdmin"
            class="btn-delete"
            title="Delete shipment"
            @click.stop="handleDelete(s.id, s.shipment_ref)"
          >×</button>
        </div>
        <div class="row-sub" @click="store.select(s.id)">
          {{ s.buyer_name ?? 'No buyer' }}
        </div>
      </div>
    </div>

    <div v-else class="empty-list">
      {{ search || statusFilter ? 'No shipments match filters' : 'No shipments yet' }}
    </div>

    <!-- Pagination -->
    <div v-if="store.totalPages > 1" class="pagination">
      <button
        class="page-btn"
        :disabled="store.currentPage <= 1"
        @click="store.goToPage(store.currentPage - 1)"
      >← Prev</button>
      <span class="page-info">
        {{ store.currentPage }} / {{ store.totalPages }}
      </span>
      <button
        class="page-btn"
        :disabled="store.currentPage >= store.totalPages"
        @click="store.goToPage(store.currentPage + 1)"
      >Next →</button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  background: var(--bg-sidebar);
  color: var(--text-sidebar);
  display: flex;
  flex-direction: column;
  height: calc(100vh - var(--header-height));
  overflow: hidden;
}

.sidebar-actions {
  padding: var(--space-md);
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.btn-new {
  width: 100%;
  background: var(--color-checklist);
  color: var(--text-inverse);
  border: none;
  padding: var(--space-sm) var(--space-md);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  font-weight: 600;
}
.btn-new:hover { opacity: 0.9; }
.btn-new:disabled { opacity: 0.5; }

.status-filter {
  width: 100%;
  padding: var(--space-xs) var(--space-sm);
  border: none;
  border-radius: var(--radius-sm);
  background: var(--bg-sidebar-hover);
  color: var(--text-sidebar);
  font-size: var(--text-sm);
  cursor: pointer;
}
.status-filter:focus { outline: 1px solid var(--color-manager); }

.search-input {
  width: 100%;
  padding: var(--space-xs) var(--space-sm);
  border: none;
  border-radius: var(--radius-sm);
  background: var(--bg-sidebar-hover);
  color: var(--text-sidebar);
  font-size: var(--text-sm);
}
.search-input::placeholder { color: var(--text-sidebar-muted); }

/* Batch bar */
.batch-bar {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-manager);
  color: var(--text-inverse);
  font-size: var(--text-sm);
}
.batch-count {
  font-weight: 700;
  white-space: nowrap;
}
.batch-status-select {
  flex: 1;
  padding: 2px var(--space-xs);
  border: 1px solid rgba(255,255,255,0.3);
  border-radius: var(--radius-sm);
  background: rgba(255,255,255,0.15);
  color: var(--text-inverse);
  font-size: var(--text-xs);
}
.batch-status-select option { color: var(--text-primary); }
.batch-apply-btn {
  background: var(--text-inverse);
  color: var(--color-manager);
  border: none;
  padding: 2px var(--space-md);
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 700;
  cursor: pointer;
}
.batch-apply-btn:hover { opacity: 0.9; }
.batch-apply-btn:disabled { opacity: 0.5; }

/* Select all */
.select-all-row {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-xs) var(--space-md);
  font-size: var(--text-xs);
  color: var(--text-sidebar-muted);
  cursor: pointer;
  border-bottom: 1px solid rgba(255,255,255,0.05);
}
.select-all-row input { cursor: pointer; }

.shipment-list {
  flex: 1;
  overflow-y: auto;
}

.shipment-row {
  width: 100%;
  display: block;
  padding: var(--space-sm) var(--space-md);
  border: none;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  background: transparent;
  color: var(--text-sidebar);
  text-align: left;
  cursor: pointer;
  transition: background 0.15s;
}
.shipment-row:hover { background: var(--bg-sidebar-hover); }
.shipment-row.active { background: var(--color-manager); color: var(--text-inverse); }

.row-main {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.row-check {
  flex-shrink: 0;
  width: 15px;
  height: 15px;
  cursor: pointer;
  accent-color: var(--color-checklist);
}

.ref {
  font-size: var(--text-sm);
  font-weight: 600;
  font-family: var(--font-mono);
  color: inherit;
  text-decoration: none;
  cursor: pointer;
  transition: color 0.15s;
}
.ref:hover { color: var(--color-checklist); text-decoration: underline; }

.row-sub {
  font-size: var(--text-xs);
  color: var(--text-sidebar-muted);
  margin-top: 2px;
  margin-left: 23px; /* align with text after checkbox */
  cursor: pointer;
}
.shipment-row.active .row-sub { color: rgba(255, 255, 255, 0.7); }

.btn-delete {
  margin-left: auto;
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.3);
  font-size: 1.1rem;
  font-weight: 700;
  padding: 0 4px;
  line-height: 1;
  transition: color 0.15s;
}
.btn-delete:hover { color: var(--color-admin); }
.shipment-row.active .btn-delete { color: rgba(255, 255, 255, 0.6); }
.shipment-row.active .btn-delete:hover { color: #fff; }

.empty-list {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-sidebar-muted);
  font-size: var(--text-sm);
  padding: var(--space-lg);
  text-align: center;
}

/* Pagination */
.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md);
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  font-size: var(--text-xs);
}
.page-btn {
  background: var(--bg-sidebar-hover);
  color: var(--text-sidebar);
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 2px var(--space-sm);
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s;
}
.page-btn:hover:not(:disabled) { background: var(--color-manager); color: var(--text-inverse); }
.page-btn:disabled { opacity: 0.3; cursor: default; }
.page-info {
  color: var(--text-sidebar-muted);
  font-weight: 600;
}

@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    left: 0;
    top: var(--header-height);
    bottom: 0;
    z-index: 100;
    width: 280px;
    transform: translateX(-100%);
    transition: transform 0.25s ease;
  }
  .sidebar.open {
    transform: translateX(0);
  }
}
</style>
