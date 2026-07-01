<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router';
import { downloadExcel } from '@/api/client';
import { useAuthStore } from '@/stores/auth';
import { ref } from 'vue';

const router = useRouter();
const route = useRoute();
const auth = useAuthStore();
const exporting = ref(false);

// ponytail: top nav ordered by operations flow — Dashboard → Calls → Shipments
const links = [
  { to: '/', label: '📊 Dashboard' },
  { to: '/calls', label: '🚢 Shipping Calls' },
  { to: '/shipments', label: '📋 Shipments' },
];

function isActive(path: string): boolean {
  if (path === '/') return route.path === '/';
  return route.path.startsWith(path);
}

async function handleExport() {
  exporting.value = true;
  try {
    await downloadExcel();
  } catch {
    alert('Export failed');
  } finally {
    exporting.value = false;
  }
}

function handleLogout() {
  auth.logout();
  router.push('/login');
}
</script>

<template>
  <header class="app-header">
    <slot name="mobile-toggle" />
    <div class="brand">
      <span class="logo">📦</span>
      <span class="title">Logistics Workflow</span>
    </div>

    <nav class="nav-links">
      <router-link
        v-for="link in links"
        :key="link.to"
        :to="link.to"
        class="nav-link"
        :class="{ active: isActive(link.to) }"
      >
        {{ link.label }}
      </router-link>
    </nav>

    <button class="export-btn" :disabled="exporting" @click="handleExport">
      {{ exporting ? 'Exporting...' : '📥 Export Excel' }}
    </button>
    <button class="logout-btn" @click="handleLogout">
      🚪 Logout
    </button>
  </header>
</template>

<style scoped>
.app-header {
  height: var(--header-height);
  background: var(--bg-sidebar);
  color: var(--text-inverse);
  display: flex;
  align-items: center;
  padding: 0 var(--space-lg);
  gap: var(--space-xl);
  position: sticky;
  top: 0;
  z-index: 100;
}

.brand {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  font-size: var(--text-lg);
  font-weight: 700;
}

.nav-links {
  display: flex;
  gap: var(--space-xs);
  flex: 1;
}

.nav-link {
  color: var(--text-sidebar-muted);
  padding: var(--space-xs) var(--space-md);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  font-weight: 500;
  transition: color 0.2s, background 0.2s;
  text-decoration: none;
}
.nav-link:hover {
  color: var(--text-sidebar);
  background: var(--bg-sidebar-hover);
  text-decoration: none;
}
.nav-link.active {
  color: var(--text-inverse);
  background: var(--color-manager);
}

.export-btn {
  background: var(--color-checklist);
  color: var(--text-inverse);
  border: none;
  padding: var(--space-xs) var(--space-md);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  font-weight: 600;
  transition: opacity 0.2s;
}
.new-call-btn {
  background: var(--color-manager);
  color: var(--text-inverse);
  border: none;
  padding: var(--space-xs) var(--space-md);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  font-weight: 600;
  cursor: pointer;
  white-space: nowrap;
}
.new-call-btn:hover { opacity: 0.9; }
.export-btn:hover { opacity: 0.9; }
.export-btn:disabled { opacity: 0.5; }

.logout-btn {
  background: var(--color-admin);
  color: var(--text-inverse);
  border: none;
  padding: var(--space-xs) var(--space-md);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
}
.logout-btn:hover { opacity: 0.8; }
</style>
