import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount, flushPromises } from '@vue/test-utils';
import { setActivePinia, createPinia } from 'pinia';

// Mock the shipments store
const mockStore = {
  selected: null as any,
  selectedId: null as number | null,
  loading: false,
  lastToast: null as { text: string; type: string } | null,
  updateCurrent: vi.fn(),
};

vi.mock('@/stores/shipments', () => ({
  useShipmentsStore: () => mockStore,
}));

import Step1Create from '@/components/workflow/Step1Create.vue';
import Step2Draft from '@/components/workflow/Step2Draft.vue';
import Step3Customs from '@/components/workflow/Step3Customs.vue';

// Helper: set up a mock selected shipment
function setSelected(overrides: Record<string, unknown> = {}) {
  mockStore.selected = {
    id: 1,
    shipment_ref: 'REF-001',
    status: 'DRAFT',
    sc_po_id: null,
    sc_po_date: null,
    sc_po_by: null,
    buyer_name: null,
    booking_number: null,
    shipping_line: null,
    origin_port: null,
    warehouse_loc: null,
    loading_plan: null,
    shipper_name: null,
    consignee_name: null,
    etd: null,
    invoice_number: null,
    invoice_date: null,
    total_value_usd: null,
    drafts_date: null,
    bill_of_lading: null,
    customs_date: null,
    customs_number: null,
    customs_status: null,
    telex_released: false,
    ...overrides,
  };
  mockStore.selectedId = 1;
}

describe('Step Form Validation', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
    mockStore.selected = null;
    mockStore.selectedId = null;
    mockStore.loading = false;
    mockStore.lastToast = null;
    mockStore.updateCurrent = vi.fn().mockResolvedValue(true);
  });

  describe('Step1Create', () => {
    it('validates required fields: sc_po_id, buyer_name, booking_number, shipping_line, origin_port', async () => {
      setSelected({
        sc_po_id: '',
        buyer_name: '',
        booking_number: '',
        shipping_line: '',
        origin_port: '',
      });

      const wrapper = mount(Step1Create);
      await flushPromises();

      // Trigger save
      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      // Should have set a toast error and not called updateCurrent
      expect(mockStore.lastToast).not.toBeNull();
      expect(mockStore.lastToast!.type).toBe('error');
      expect(mockStore.updateCurrent).not.toHaveBeenCalled();
    });

    it('passes validation when all required fields are filled', async () => {
      setSelected({
        sc_po_id: 'SC-001',
        buyer_name: 'Buyer Inc.',
        booking_number: 'BK001',
        shipping_line: 'Maersk',
        origin_port: 'Haiphong',
      });

      const wrapper = mount(Step1Create);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      expect(mockStore.updateCurrent).toHaveBeenCalled();
      const callArgs = mockStore.updateCurrent.mock.calls[0][0];
      expect(callArgs.sc_po_id).toBe('SC-001');
      expect(callArgs.buyer_name).toBe('Buyer Inc.');
    });

    it('uses String(v ?? "").trim() helper — number fields do not throw TypeError', async () => {
      // Simulate a number being passed as a field value (shouldn't throw)
      setSelected({
        sc_po_id: 12345 as any,        // number, not string
        buyer_name: 'Buyer',
        booking_number: 'BK001',
        shipping_line: 'Maersk',
        origin_port: 'Haiphong',
      });

      const wrapper = mount(Step1Create);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      // Should have passed validation - number was coerced to string
      expect(mockStore.updateCurrent).toHaveBeenCalled();
    });

    it('validates each missing required field individually', async () => {
      setSelected({
        sc_po_id: 'SC-001',
        buyer_name: 'Buyer',
        booking_number: 'BK001',
        shipping_line: 'Maersk',
        origin_port: '', // missing
      });

      const wrapper = mount(Step1Create);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      expect(mockStore.lastToast!.type).toBe('error');
      expect(mockStore.lastToast!.text).toContain('ORIGIN PORT');
      expect(mockStore.updateCurrent).not.toHaveBeenCalled();
    });
  });

  describe('Step2Draft', () => {
    it('validates required fields: shipper_name, consignee_name, etd, invoice_number, total_value_usd', async () => {
      setSelected({
        shipper_name: '',
        consignee_name: '',
        etd: '',
        invoice_number: '',
        total_value_usd: '',
      });

      const wrapper = mount(Step2Draft);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      expect(mockStore.lastToast).not.toBeNull();
      expect(mockStore.lastToast!.type).toBe('error');
      expect(mockStore.updateCurrent).not.toHaveBeenCalled();
    });

    it('passes validation when all required fields are filled', async () => {
      setSelected({
        shipper_name: 'Shipper Co.',
        consignee_name: 'Consignee Ltd.',
        etd: '2026-08-15',
        invoice_number: 'INV-001',
        total_value_usd: '50000',
      });

      const wrapper = mount(Step2Draft);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      expect(mockStore.updateCurrent).toHaveBeenCalled();
      const callArgs = mockStore.updateCurrent.mock.calls[0][0];
      expect(callArgs.shipper_name).toBe('Shipper Co.');
      expect(callArgs.consignee_name).toBe('Consignee Ltd.');
      expect(callArgs.etd).toBe('2026-08-15');
      expect(callArgs.invoice_number).toBe('INV-001');
      expect(callArgs.total_value_usd).toBe('50000');
    });

    it('uses String(v ?? "").trim() — number values do not throw TypeError', async () => {
      setSelected({
        shipper_name: 'Shipper',
        consignee_name: 'Consignee',
        etd: '2026-08-15',
        invoice_number: 'INV-001',
        total_value_usd: 50000 as any, // number, not string
      });

      const wrapper = mount(Step2Draft);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      expect(mockStore.updateCurrent).toHaveBeenCalled();
    });
  });

  describe('Step3Customs', () => {
    it('validates required fields: customs_date and customs_number', async () => {
      setSelected({
        customs_date: '',
        customs_number: '',
        customs_status: '',
      });

      const wrapper = mount(Step3Customs);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      expect(mockStore.lastToast).not.toBeNull();
      expect(mockStore.lastToast!.type).toBe('error');
      expect(mockStore.updateCurrent).not.toHaveBeenCalled();
    });

    it('passes validation when customs_date and customs_number are filled', async () => {
      setSelected({
        customs_date: '2026-09-01',
        customs_number: 'CUS-001',
        customs_status: 'green',
      });

      const wrapper = mount(Step3Customs);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      expect(mockStore.updateCurrent).toHaveBeenCalled();
      const callArgs = mockStore.updateCurrent.mock.calls[0][0];
      expect(callArgs.customs_date).toBe('2026-09-01');
      expect(callArgs.customs_number).toBe('CUS-001');
    });

    it('validates customs_number is required even when date is present', async () => {
      setSelected({
        customs_date: '2026-09-01',
        customs_number: '',
        customs_status: 'red',
      });

      const wrapper = mount(Step3Customs);
      await flushPromises();

      await wrapper.find('form').trigger('submit.prevent');
      await flushPromises();

      expect(mockStore.lastToast!.type).toBe('error');
      expect(mockStore.lastToast!.text).toContain('Customs Number');
      expect(mockStore.updateCurrent).not.toHaveBeenCalled();
    });
  });
});
