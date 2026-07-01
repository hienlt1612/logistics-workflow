import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount, flushPromises } from '@vue/test-utils';
import { setActivePinia, createPinia } from 'pinia';

let mockAuthStore: any;
const mockPush = vi.fn();

vi.mock('@/stores/auth', () => ({
  useAuthStore: () => mockAuthStore,
}));

vi.mock('@/api/client', () => ({
  downloadExcel: vi.fn(),
}));

vi.mock('vue-router', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
  useRoute: () => ({
    path: '/',
  }),
}));

import AppHeader from '@/components/layout/AppHeader.vue';
import * as api from '@/api/client';

describe('AppHeader', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();

    mockAuthStore = {
      isLoggedIn: true,
      isAdmin: true,
      user: { username: 'admin', role: 'admin' },
      logout: vi.fn(),
    };
  });

  it('renders navigation links (Dashboard, Workflow, Export)', () => {
    const wrapper = mount(AppHeader, {
      global: {
        stubs: {
          'router-link': {
            props: ['to'],
            template: '<a :href="to"><slot /></a>',
          },
        },
      },
    });

    const links = wrapper.findAll('a');
    const linkTexts = links.map(l => l.text());
    expect(linkTexts).toContain('📊 Dashboard');
    expect(linkTexts).toContain('🚢 Shipping Calls');
    expect(linkTexts).toContain('📋 Shipments');
  });

  it('renders logout button', () => {
    const wrapper = mount(AppHeader, {
      global: {
        stubs: { 'router-link': true },
      },
    });

    const logoutBtn = wrapper.find('.logout-btn');
    expect(logoutBtn.exists()).toBe(true);
    expect(logoutBtn.text()).toContain('Logout');
  });

  it('logout button calls auth.logout and redirects to /login', async () => {
    const wrapper = mount(AppHeader, {
      global: {
        stubs: { 'router-link': true },
      },
    });

    const logoutBtn = wrapper.find('.logout-btn');
    await logoutBtn.trigger('click');

    expect(mockAuthStore.logout).toHaveBeenCalledTimes(1);
    expect(mockPush).toHaveBeenCalledWith('/login');
  });

  it('export button is disabled while exporting', async () => {
    vi.mocked(api.downloadExcel).mockImplementation(
      () => new Promise(resolve => setTimeout(resolve, 5000))
    );

    const wrapper = mount(AppHeader, {
      global: {
        stubs: { 'router-link': true },
      },
    });

    const exportBtn = wrapper.find('.export-btn');
    expect(exportBtn.attributes('disabled')).toBeUndefined();

    // Click to start export
    await exportBtn.trigger('click');
    await flushPromises();

    // Button should now be disabled and show "Exporting..."
    expect(exportBtn.attributes('disabled')).toBeDefined();
    expect(exportBtn.text()).toBe('Exporting...');
  });

  it('export button text returns to normal after export completes', async () => {
    vi.mocked(api.downloadExcel).mockResolvedValue(undefined);

    const wrapper = mount(AppHeader, {
      global: {
        stubs: { 'router-link': true },
      },
    });

    const exportBtn = wrapper.find('.export-btn');
    await exportBtn.trigger('click');
    await flushPromises();

    // After export resolves, button should be re-enabled
    expect(exportBtn.attributes('disabled')).toBeUndefined();
    expect(exportBtn.text()).toBe('📥 Export Excel');
  });

  it('export button handles errors gracefully', async () => {
    vi.mocked(api.downloadExcel).mockRejectedValue(new Error('Export failed'));
    const alertSpy = vi.spyOn(window, 'alert').mockImplementation(() => {});

    const wrapper = mount(AppHeader, {
      global: {
        stubs: { 'router-link': true },
      },
    });

    const exportBtn = wrapper.find('.export-btn');
    await exportBtn.trigger('click');
    await flushPromises();

    // Should alert and re-enable button
    expect(alertSpy).toHaveBeenCalledWith('Export failed');
    expect(exportBtn.attributes('disabled')).toBeUndefined();
  });
});
