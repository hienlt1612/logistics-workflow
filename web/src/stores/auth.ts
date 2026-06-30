import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface AuthUser {
  username: string;
  role: string;
  token: string;
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<AuthUser | null>(null);

  const isLoggedIn = computed(() => user.value !== null);
  const role = computed(() => user.value?.role ?? '');

  function loadFromStorage() {
    const stored = localStorage.getItem('LW_AUTH');
    if (stored) {
      try { user.value = JSON.parse(stored); } catch { user.value = null; }
    }
  }

  async function login(username: string, password: string): Promise<string | null> {
    const res = await fetch('http://127.0.0.1:19876/api/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    if (!res.ok) {
      const err = await res.json();
      return err?.error?.message || 'Login failed';
    }
    const data = await res.json();
    user.value = { username: data.username, role: data.role, token: data.token };
    localStorage.setItem('LW_AUTH', JSON.stringify(user.value));
    // Store the real API token for write operations (returned by backend login)
    localStorage.setItem('LW_API_TOKEN', data.token);
    return null; // success
  }

  function logout() {
    user.value = null;
    localStorage.removeItem('LW_AUTH');
    localStorage.removeItem('LW_API_TOKEN');
  }

  // Init from localStorage on store creation
  loadFromStorage();

  return { user, isLoggedIn, role, login, logout, loadFromStorage };
});
