<script setup lang="ts">
import { ref, watch } from 'vue';
import { RouterView } from 'vue-router';
import AppLayout from '@/components/layout/AppLayout.vue';
import Toast from '@/components/shared/Toast.vue';
import { useShipmentsStore } from '@/stores/shipments';

const ship = useShipmentsStore();
const toastRef = ref<InstanceType<typeof Toast> | null>(null);

watch(
  () => ship.lastToast,
  (t) => {
    if (t && toastRef.value) {
      toastRef.value.show(t.text, t.type);
      ship.clearToast();
    }
  }
);
</script>

<template>
  <AppLayout>
    <RouterView />
  </AppLayout>
  <Toast ref="toastRef" />
</template>
