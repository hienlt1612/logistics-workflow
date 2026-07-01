/**
 * Date/currency formatters for logistics-workflow UI.
 * API/DB uses ISO format (yyyy-MM-dd). UI displays dd/MM/yyyy.
 */

/** Convert ISO date string (yyyy-MM-dd) → display (dd/MM/yyyy). Empty/null → ''. */
export function fmtDateDisplay(iso: string | null | undefined): string {
  if (!iso) return '';
  const d = iso.includes('T') ? iso.slice(0, 10) : iso;
  const [y, m, day] = d.split('-');
  if (!y || !m || !day) return iso; // fallback
  return `${day}/${m}/${y}`;
}

/** Convert display date (dd/MM/yyyy) → ISO (yyyy-MM-dd) for API. Empty → ''. */
export function fmtDateISO(display: string): string {
  if (!display) return '';
  const parts = display.split('/');
  if (parts.length === 3) {
    const d = parts[0]!, m = parts[1]!, y = parts[2]!;
    return `${y}-${m.padStart(2, '0')}-${d.padStart(2, '0')}`;
  }
  return display; // fallback: return as-is
}

/** Format currency value with VN locale. null/empty → ''. */
export function fmtCurrency(val: string | null | undefined): string {
  if (val === null || val === undefined || val === '') return '';
  const n = parseFloat(String(val));
  if (isNaN(n)) return String(val);
  // ponytail: VN locale — 1,234.56 → 1.234,56
  return new Intl.NumberFormat('vi-VN', { minimumFractionDigits: 2, maximumFractionDigits: 2 }).format(n);
}
