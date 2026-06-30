import { describe, it, expect } from 'vitest';
import { fmtDateDisplay, fmtDateISO, fmtCurrency } from '@/utils/format';

describe('Format Utilities', () => {
  describe('fmtDateDisplay', () => {
    it('converts ISO to dd/MM/yyyy', () => {
      expect(fmtDateDisplay('2026-07-03')).toBe('03/07/2026');
    });
    it('handles ISO with time', () => {
      expect(fmtDateDisplay('2026-07-03T12:00:00Z')).toBe('03/07/2026');
    });
    it('returns empty for null', () => {
      expect(fmtDateDisplay(null)).toBe('');
    });
    it('returns empty for undefined', () => {
      expect(fmtDateDisplay(undefined)).toBe('');
    });
    it('returns empty for empty string', () => {
      expect(fmtDateDisplay('')).toBe('');
    });
  });

  describe('fmtDateISO', () => {
    it('converts dd/MM/yyyy to ISO', () => {
      expect(fmtDateISO('03/07/2026')).toBe('2026-07-03');
    });
    it('pads single-digit day/month', () => {
      expect(fmtDateISO('1/5/2026')).toBe('2026-05-01');
    });
    it('returns empty for empty', () => {
      expect(fmtDateISO('')).toBe('');
    });
  });

  describe('fmtCurrency', () => {
    it('formats to 2 decimal places', () => {
      expect(fmtCurrency('8888.5')).toBe('8888.50');
    });
    it('handles integer string', () => {
      expect(fmtCurrency('5000')).toBe('5000.00');
    });
    it('returns empty for null', () => {
      expect(fmtCurrency(null)).toBe('');
    });
    it('returns empty for undefined', () => {
      expect(fmtCurrency(undefined)).toBe('');
    });
    it('returns empty for empty string', () => {
      expect(fmtCurrency('')).toBe('');
    });
  });
});
