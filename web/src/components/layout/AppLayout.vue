<script setup lang="ts">
import { ref } from 'vue';
import AppHeader from './AppHeader.vue';
import AppSidebar from './AppSidebar.vue';

const sidebarOpen = ref(false);
</script>

<template>
  <div class="app-layout">
    <AppHeader>
      <template #mobile-toggle>
        <button class="menu-btn" @click="sidebarOpen = !sidebarOpen" aria-label="Toggle menu">
          ☰
        </button>
      </template>
    </AppHeader>
    <div class="app-body">
      <AppSidebar :class="{ open: sidebarOpen }" />
      <div v-if="sidebarOpen" class="sidebar-overlay" @click="sidebarOpen = false" />
      <main class="main-content">
        <router-view />
      </main>
    </div>
  </div>
</template>

<style scoped>
.app-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-body {
  display: flex;
  flex: 1;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-primary);
}

.menu-btn {
  display: none;
  background: none;
  border: none;
  color: var(--text-inverse);
  font-size: 1.25rem;
  padding: var(--space-xs);
  cursor: pointer;
}

.sidebar-overlay {
  display: none;
}

@media (max-width: 768px) {
  .app-body {
    flex-direction: column;
  }

  .menu-btn {
    display: block;
    margin-right: var(--space-sm);
  }

  .sidebar-overlay {
    display: block;
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    z-index: 90;
  }
}
</style>
