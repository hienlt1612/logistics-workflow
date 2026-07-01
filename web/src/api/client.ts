// API Client for Logistics Workflow REST API

const BASE = 'http://127.0.0.1:19876';

// ── Auth ──
// Read API token from localStorage (set by auth store after login). Falls back to env var.
function getToken(): string | null {
  return localStorage.getItem('LW_API_TOKEN') || null;
}
function getRole(): string {
  const auth = localStorage.getItem('LW_AUTH');
  if (auth) {
    try { return JSON.parse(auth).role || 'user'; } catch { return 'user'; }
  }
  return 'user';
}
function authHeader(): Record<string, string> {
  const token = getToken();
  const headers: Record<string, string> = {};
  if (token) headers['Authorization'] = `Bearer ${token}`;
  headers['X-User-Role'] = getRole();
  return headers;
}

export interface Shipment {
  id: number;
  shipment_ref: string;
  status: string;
  created_at: string;
  updated_at: string;
  sc_po_id: string | null;
  sc_po_date: string | null;
  sc_po_by: string | null;
  buyer_name: string | null;
  booking_number: string | null;
  shipping_line: string | null;
  origin_port: string | null;
  warehouse_loc: string | null;
  loading_plan: string | null;
  shipper_name: string | null;
  consignee_name: string | null;
  etd: string | null;
  invoice_number: string | null;
  invoice_date: string | null;
  total_value_usd: string | null;
  drafts_date: string | null;
  bill_of_lading: string | null;
  customs_date: string | null;
  customs_number: string | null;
  customs_status: string | null;
  bl_received: boolean;
  charges_paid: boolean;
  co_received: boolean;
  phyto_received: boolean;
  docs_confirmed: boolean;
  prepayment_date: string | null;
  prepayment_amt: string | null;
  remaining_amt: string | null;
  originals_status: string | null;
  originals_sent: string | null;
  originals_description: string | null;
  telex_released: boolean;
  payment_received: boolean;
  shipping_call_id: number | null;
  containers_loaded: boolean;
}

export interface DashboardStats {
  total: number;
  draft: number;
  documents: number;
  customs: number;
  checklist: number;
  telex: number;
  calls_total: number;
  calls_open: number;
  calls_loading: number;
  calls_closed: number;
}

export interface CreateShipmentInput {
  sc_po_id?: string;
  sc_po_date?: string;
  sc_po_by?: string;
  buyer_name?: string;
  booking_number?: string;
  shipping_line?: string;
  origin_port?: string;
  warehouse_loc?: string;
  loading_plan?: string;
  shipping_call_id?: number;
}

export interface PaginatedShipments {
  data: Shipment[];
  pagination: {
    page: number;
    pageSize: number;
    totalItems: number;
    totalPages: number;
  };
}

export interface ApiError {
  error: {
    code: string;
    message: string;
  };
}

// ── API Client ──

class ApiClientError extends Error {
  code: string;
  constructor(code: string, message: string) {
    super(message);
    this.code = code;
    this.name = 'ApiClientError';
  }
}

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const method = options?.method || 'GET';
  const headers: Record<string, string> = { 'Content-Type': 'application/json' };
  if (method !== 'GET') Object.assign(headers, authHeader());
  const res = await fetch(`${BASE}${path}`, {
    ...options,
    headers: { ...headers, ...options?.headers },
  });

  if (!res.ok) {
    const body: ApiError = await res.json().catch(() => ({
      error: { code: 'UNKNOWN', message: res.statusText },
    }));
    throw new ApiClientError(body.error.code, body.error.message);
  }

  return res.json();
}

type FetchOptions = { page?: number; pageSize?: number; status?: string };

// ── Dashboard ──

export async function fetchDashboard(): Promise<DashboardStats> {
  return request('/api/dashboard');
}

// ── Shipments ──

export async function fetchShipments(opts?: FetchOptions): Promise<PaginatedShipments> {
  const params = new URLSearchParams();
  if (opts?.status) params.set('status', opts.status);
  if (opts?.page) params.set('page', String(opts.page));
  if (opts?.pageSize) params.set('pageSize', String(opts.pageSize));
  const qs = params.toString();
  return request(`/api/shipments${qs ? `?${qs}` : ''}`);
}

export async function fetchShipment(id: number): Promise<Shipment> {
  return request(`/api/shipments/${id}`);
}

export async function createShipment(data: CreateShipmentInput): Promise<Shipment> {
  return request('/api/shipments', {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export async function updateShipment(id: number, fields: Record<string, unknown>): Promise<Shipment> {
  return request(`/api/shipments/${id}`, {
    method: 'PATCH',
    body: JSON.stringify(fields),
  });
}

export async function toggleChecklist(id: number, field: string, value: boolean): Promise<Shipment> {
  return request(`/api/shipments/${id}/checklist`, {
    method: 'PATCH',
    body: JSON.stringify({ field, value }),
  });
}

export async function deleteShipment(id: number): Promise<void> {
  await fetch(`${BASE}/api/shipments/${id}`, { method: 'DELETE', headers: authHeader() });
}

export async function batchAdvanceStatus(ids: number[], status: string): Promise<number> {
  const res = await fetch(`${BASE}/api/shipments/batch`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json', ...authHeader() },
    body: JSON.stringify({ ids, status }),
  });
  if (!res.ok) throw new Error('Batch update failed');
  const data = await res.json();
  return data.updated;
}

// ── Export ──

export async function downloadExcel(): Promise<void> {
  const res = await fetch(`${BASE}/api/export/all`);
  if (!res.ok) throw new Error('Export failed');
  const json = await res.json();
  // Decode base64 data to binary
  const binary = atob(json.data);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  const blob = new Blob([bytes], { type: json.contentType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = json.filename || 'workbook1_export.xlsx';
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

// ── Shipping Calls ──
// ponytail: reuse request() wrapper from shipments.

export interface ShippingCall {
  id: number;
  call_ref: string;
  sc_po_id: string | null;
  sc_po_date: string | null;
  sc_po_by: string | null;
  buyer_name: string;
  incoterms: string;
  product_description: string | null;
  total_containers: number;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface CallWarehouse {
  id: number;
  shipping_call_id: number;
  warehouse_name: string;
  planned_containers: number;
  loaded_containers: number;
  status: string;
  notes: string | null;
}

export interface Container {
  id: number;
  shipment_id: number;
  container_number: string;
  seal_number: string | null;
  warehouse_name: string | null;
  loaded_date: string | null;
  status: string;
  notes: string | null;
}

export interface CreateShippingCallInput {
  sc_po_id?: string;
  sc_po_date?: string;
  sc_po_by?: string;
  buyer_name: string;
  incoterms: string;
  product_description?: string;
  total_containers: number;
  warehouses?: { warehouse_name: string; planned_containers: number }[];
  status?: string;
}

export interface CreateContainerInput {
  shipment_id: number;
  container_number: string;
  seal_number?: string;
  warehouse_name?: string;
  loaded_date?: string;
}

export async function fetchShippingCalls(): Promise<ShippingCall[]> {
  return request('/api/shipping-calls');
}

export async function fetchShippingCall(id: number): Promise<ShippingCall> {
  return request(`/api/shipping-calls/${id}`);
}

export async function createShippingCall(data: CreateShippingCallInput): Promise<ShippingCall> {
  return request('/api/shipping-calls', { method: 'POST', body: JSON.stringify(data) });
}

export async function updateShippingCall(id: number, data: Partial<CreateShippingCallInput>): Promise<ShippingCall> {
  return request(`/api/shipping-calls/${id}`, { method: 'PATCH', body: JSON.stringify(data) });
}

export async function deleteShippingCall(id: number): Promise<void> {
  await fetch(`${BASE}/api/shipping-calls/${id}`, { method: 'DELETE', headers: authHeader() });
}

export async function fetchCallWarehouses(callId: number): Promise<CallWarehouse[]> {
  return request(`/api/shipping-calls/${callId}/warehouses`);
}

export async function fetchContainers(shipmentId: number): Promise<Container[]> {
  return request(`/api/shipments/${shipmentId}/containers`);
}

export async function createContainer(data: CreateContainerInput): Promise<Container> {
  return request('/api/containers', { method: 'POST', body: JSON.stringify(data) });
}

export async function deleteContainer(id: number): Promise<void> {
  await fetch(`${BASE}/api/containers/${id}`, { method: 'DELETE', headers: authHeader() });
}

export async function updateContainer(id: number, data: Partial<CreateContainerInput>): Promise<Container> {
  return request(`/api/containers/${id}`, { method: 'PATCH', body: JSON.stringify(data) });
}
