import { createRouter, createWebHistory } from 'vue-router';
import { useAuthStore } from '@/stores/auth';
import DashboardView from '@/views/DashboardView.vue';
import LoginView from '@/views/LoginView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/login', name: 'login', component: LoginView, meta: { public: true } },
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
    {
      path: '/calls',
      name: 'calls',
      component: () => import('@/views/ShippingCallList.vue'),
    },
    {
      path: '/calls/new',
      name: 'call-create',
      component: () => import('@/views/ShippingCallCreate.vue'),
    },
    {
      path: '/calls/:id',
      name: 'call-detail',
      component: () => import('@/views/ShippingCallDetail.vue'),
    },
  ],
});

// Auth guard: redirect to /login if not authenticated
router.beforeEach((to, _from, next) => {
  // Initialize auth store inside the guard to avoid import-order issues
  const auth = useAuthStore();
  if (to.meta.public || auth.isLoggedIn) {
    next();
  } else {
    next('/login');
  }
});

export default router;
