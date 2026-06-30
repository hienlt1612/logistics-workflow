import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useAuthStore } from '@/stores/auth';

describe('Auth Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
  });

  it('starts with no user', () => {
    const auth = useAuthStore();
    expect(auth.isLoggedIn).toBe(false);
    expect(auth.user).toBeNull();
  });

  it('loads user from localStorage', () => {
    localStorage.setItem('LW_AUTH', JSON.stringify({ username: 'test', role: 'admin', token: 'abc' }));
    const auth = useAuthStore();
    auth.loadFromStorage();
    expect(auth.isLoggedIn).toBe(true);
    expect(auth.role).toBe('admin');
  });

  it('logout clears state', () => {
    localStorage.setItem('LW_AUTH', JSON.stringify({ username: 'test', role: 'admin', token: 'abc' }));
    const auth = useAuthStore();
    auth.loadFromStorage();
    auth.logout();
    expect(auth.isLoggedIn).toBe(false);
    expect(localStorage.getItem('LW_AUTH')).toBeNull();
  });
});
