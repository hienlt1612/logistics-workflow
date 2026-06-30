import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useDashboardStore } from '@/stores/dashboard';

// Mock the API client
vi.mock('@/api/client', () => ({
  fetchDashboard: vi.fn(),
}));

import * as api from '@/api/client';

describe('Dashboard Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  describe('startPolling', () => {
    it('calls fetchDashboard via load()', async () => {
      const mockStats = { total: 10, draft: 3, documents: 4, customs: 2, checklist: 1, telex: 0 };
      vi.mocked(api.fetchDashboard).mockResolvedValue(mockStats);

      const store = useDashboardStore();
      await store.startPolling(30000);

      expect(api.fetchDashboard).toHaveBeenCalledTimes(1);
      expect(store.stats.total).toBe(10);
      expect(store.loading).toBe(false);
      expect(store.lastUpdated).not.toBeNull();
    });
  });

  describe('stopPolling', () => {
    it('clears the interval', async () => {
      const mockStats = { total: 5, draft: 1, documents: 2, customs: 1, checklist: 1, telex: 0 };
      vi.mocked(api.fetchDashboard).mockResolvedValue(mockStats);

      const store = useDashboardStore();
      await store.startPolling(1000);

      // After startPolling, load() has been called once
      expect(api.fetchDashboard).toHaveBeenCalledTimes(1);

      store.stopPolling();

      // Advance time past one interval
      await vi.advanceTimersByTimeAsync(2000);

      // Should still only be 1 call (polling stopped)
      expect(api.fetchDashboard).toHaveBeenCalledTimes(1);
    });

    it('is safe to call stopPolling when not polling', () => {
      const store = useDashboardStore();
      // Should not throw
      expect(() => store.stopPolling()).not.toThrow();
    });
  });

  describe('secondsAgo', () => {
    it('returns 999 when never updated', () => {
      const store = useDashboardStore();
      expect(store.secondsAgo()).toBe(999);
    });

    it('returns correct elapsed seconds', async () => {
      const mockStats = { total: 8, draft: 2, documents: 3, customs: 2, checklist: 1, telex: 0 };
      vi.mocked(api.fetchDashboard).mockResolvedValue(mockStats);

      const store = useDashboardStore();
      await store.load();

      // Immediately after load, should be ~0
      expect(store.secondsAgo()).toBeLessThanOrEqual(1);

      // Advance time by 50 seconds
      vi.advanceTimersByTime(50_000);

      expect(store.secondsAgo()).toBe(50);
    });

    it('returns correct elapsed seconds after loader sets lastUpdated', async () => {
      const store = useDashboardStore();
      const mockStats = { total: 3, draft: 0, documents: 1, customs: 1, checklist: 0, telex: 1 };
      vi.mocked(api.fetchDashboard).mockResolvedValue(mockStats);

      await store.load();
      // Should be very recent
      const first = store.secondsAgo();
      expect(first).toBeGreaterThanOrEqual(0);

      vi.advanceTimersByTime(120_000);
      expect(store.secondsAgo()).toBe(first + 120);
    });
  });
});
