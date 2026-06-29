import { createRouter, createWebHistory } from 'vue-router';
import DashboardView from '@/views/DashboardView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'dashboard', component: DashboardView },
    {
      path: '/workflow',
      name: 'workflow',
      component: () => import('@/views/WorkflowView.vue'),
    },
    {
      path: '/workflow/:id',
      name: 'workflow-detail',
      component: () => import('@/views/WorkflowView.vue'),
    },
    {
      path: '/shipment/:id',
      name: 'shipment-detail',
      component: () => import('@/views/ShipmentDetailView.vue'),
    },
    {
      path: '/export',
      name: 'export',
      component: () => import('@/views/ExportView.vue'),
    },
  ],
});

export default router;
