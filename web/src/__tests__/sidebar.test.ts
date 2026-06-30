import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount, flushPromises } from '@vue/test-utils';
import { setActivePinia, createPinia } from 'pinia';
import type { Shipment } from '@/api/client';

// Create mock shipments data
const mockShipments: Shipment[] = [
  { id: 1, shipment_ref: 'SHIP-001', status: 'DRAFT', buyer_name: 'Alpha Corp', created_at: '', updated_at: '', sc_po_id: null, sc_po_date: null, sc_po_by: null, booking_number: null, shipping_line: null, origin_port: null, warehouse_loc: null, loading_plan: null, shipper_name: null, consignee_name: null, etd: null, invoice_number: null, invoice_date: null, total_value_usd: null, drafts_date: null, bill_of_lading: null, customs_date: null, customs_number: null, customs_status: null, bl_received: false, charges_paid: false, co_received: false, phyto_received: false, docs_confirmed: false, prepayment_date: null, prepayment_amt: null, remaining_amt: null, originals_status: null, originals_sent: null, originals_description: null, telex_released: false, payment_received: false },
  { id: 2, shipment_ref: 'SHIP-002', status: 'DOCUMENTS_READY', buyer_name: 'Beta Ltd', created_at: '', updated_at: '', sc_po_id: null, sc_po_date: null, sc_po_by: null, booking_number: null, shipping_line: null, origin_port: null, warehouse_loc: null, loading_plan: null, shipper_name: null, consignee_name: null, etd: null, invoice_number: null, invoice_date: null, total_value_usd: null, drafts_date: null, bill_of_lading: null, customs_date: null, customs_number: null, customs_status: null, bl_received: false, charges_paid: false, co_received: false, phyto_received: false, docs_confirmed: false, prepayment_date: null, prepayment_amt: null, remaining_amt: null, originals_status: null, originals_sent: null, originals_description: null, telex_released: false, payment_received: false },
  { id: 3, shipment_ref: 'SHIP-003', status: 'CUSTOMS_CLEARED', buyer_name: 'Gamma Inc', created_at: '', updated_at: '', sc_po_id: null, sc_po_date: null, sc_po_by: null, booking_number: null, shipping_line: null, origin_port: null, warehouse_loc: null, loading_plan: null, shipper_name: null, consignee_name: null, etd: null, invoice_number: null, invoice_date: null, total_value_usd: null, drafts_date: null, bill_of_lading: null, customs_date: null, customs_number: null, customs_status: null, bl_received: false, charges_paid: false, co_received: false, phyto_received: false, docs_confirmed: false, prepayment_date: null, prepayment_amt: null, remaining_amt: null, originals_status: null, originals_sent: null, originals_description: null, telex_released: false, payment_received: false },
];

let mockShipmentStore: any;
let mockAuthStore: any;

// Mock stores
vi.mock('@/stores/shipments', () => ({
  useShipmentsStore: () => mockShipmentStore,
}));

vi.mock('@/stores/auth', () => ({
  useAuthStore: () => mockAuthStore,
}));

// Mock router
const mockPush = vi.fn();
vi.mock('vue-router', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
  useRoute: () => ({
    path: '/',
  }),
}));

// Mock StatusBadge component
vi.mock('@/components/shared/StatusBadge.vue', () => ({
  default: {
    name: 'StatusBadge',
    props: ['label', 'size'],
    template: '<span class="status-badge">{{ label }}</span>',
  },
}));

import AppSidebar from '@/components/layout/AppSidebar.vue';

function createDefaultMocks() {
  mockShipmentStore = {
    shipments: [...mockShipments],
    selectedId: null,
    loading: false,
    error: null,
    currentPage: 1,
    totalPages: 1,
    loadAll: vi.fn().mockResolvedValue(undefined),
    goToPage: vi.fn(),
    select: vi.fn(),
    create: vi.fn().mockResolvedValue({ id: 99, shipment_ref: 'SHIP-099' }),
    remove: vi.fn().mockResolvedValue(true),
    batchAdvance: vi.fn().mockResolvedValue(1),
  };

  mockAuthStore = {
    isLoggedIn: true,
    isAdmin: true,
    user: { username: 'admin', role: 'admin' },
    role: 'admin',
  };
}

describe('AppSidebar', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
    createDefaultMocks();
  });

  it('renders shipment list items', () => {
    const wrapper = mount(AppSidebar);
    const rows = wrapper.findAll('.shipment-row');
    expect(rows).toHaveLength(3);
    expect(wrapper.text()).toContain('SHIP-001');
    expect(wrapper.text()).toContain('SHIP-002');
    expect(wrapper.text()).toContain('SHIP-003');
  });

  it('search filters shipments by ref or buyer name', async () => {
    const wrapper = mount(AppSidebar);
    const searchInput = wrapper.find('.search-input');
    await searchInput.setValue('alpha');
    await flushPromises();

    const rows = wrapper.findAll('.shipment-row');
    expect(rows).toHaveLength(1);
    expect(rows[0].text()).toContain('SHIP-001');
  });

  it('search filters by buyer name', async () => {
    const wrapper = mount(AppSidebar);
    const searchInput = wrapper.find('.search-input');
    await searchInput.setValue('beta');
    await flushPromises();

    const rows = wrapper.findAll('.shipment-row');
    expect(rows).toHaveLength(1);
    expect(rows[0].text()).toContain('SHIP-002');
  });

  it('status dropdown filters by status', async () => {
    const wrapper = mount(AppSidebar);

    // Trigger status filter change
    const select = wrapper.find('.status-filter');
    await select.setValue('DOCUMENTS_READY');

    // The handleStatusChange calls store.loadAll with status param
    expect(mockShipmentStore.loadAll).toHaveBeenCalledWith('DOCUMENTS_READY');
  });

  it('delete button visible for admin user', () => {
    mockAuthStore.isAdmin = true;
    const wrapper = mount(AppSidebar);

    const deleteButtons = wrapper.findAll('.btn-delete');
    expect(deleteButtons).toHaveLength(3); // one per shipment
  });

  it('delete button hidden for non-admin user', () => {
    mockAuthStore.isAdmin = false;
    mockAuthStore.user = { username: 'user', role: 'user' };
    const wrapper = mount(AppSidebar);

    const deleteButtons = wrapper.findAll('.btn-delete');
    expect(deleteButtons).toHaveLength(0);
  });

  it('pagination footer shows Prev/Next when totalPages > 1', () => {
    mockShipmentStore.totalPages = 3;
    mockShipmentStore.currentPage = 2;

    const wrapper = mount(AppSidebar);
    const pagination = wrapper.find('.pagination');
    expect(pagination.exists()).toBe(true);

    const prevBtn = pagination.find('.page-btn:first-child');
    const nextBtn = pagination.find('.page-btn:last-child');
    expect(prevBtn.text()).toContain('Prev');
    expect(nextBtn.text()).toContain('Next');
    expect(pagination.text()).toContain('2 / 3');
  });

  it('pagination footer hidden when totalPages <= 1', () => {
    mockShipmentStore.totalPages = 1;
    const wrapper = mount(AppSidebar);

    expect(wrapper.find('.pagination').exists()).toBe(false);
  });

  it('shows empty message when no shipments', () => {
    mockShipmentStore.shipments = [];
    const wrapper = mount(AppSidebar);

    expect(wrapper.find('.empty-list').exists()).toBe(true);
    expect(wrapper.text()).toContain('No shipments yet');
  });

  it('shows filtered empty message when search active', async () => {
    const wrapper = mount(AppSidebar);
    const searchInput = wrapper.find('.search-input');
    await searchInput.setValue('nonexistent');
    await flushPromises();

    expect(wrapper.find('.empty-list').exists()).toBe(true);
    expect(wrapper.text()).toContain('No shipments match filters');
  });
});
