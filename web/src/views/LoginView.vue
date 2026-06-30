<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '@/stores/auth';

const router = useRouter();
const auth = useAuthStore();

const username = ref('');
const password = ref('');
const error = ref('');
const loading = ref(false);

async function handleLogin() {
  error.value = '';
  if (!username.value || !password.value) {
    error.value = 'Please enter username and password';
    return;
  }
  loading.value = true;
  const err = await auth.login(username.value, password.value);
  loading.value = false;
  if (err) {
    error.value = err;
  } else {
    router.push('/');
  }
}
</script>

<template>
  <div class="login-page">
    <div class="login-card">
      <h1 class="login-title">Logistics Workflow</h1>
      <p class="login-sub">Sign in to continue</p>

      <form @submit.prevent="handleLogin" class="login-form">
        <label class="field">
          Username
          <input v-model="username" type="text" placeholder="Username" autocomplete="username" />
        </label>
        <label class="field">
          Password
          <input v-model="password" type="password" placeholder="Enter password" autocomplete="current-password" />
        </label>

        <div v-if="error" class="error-msg">{{ error }}</div>

        <button type="submit" class="btn-login" :disabled="loading">
          {{ loading ? 'Signing in...' : 'Sign In' }}
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #2C3E50 0%, #3498DB 100%);
}
.login-card {
  background: var(--bg-card, #fff);
  border-radius: 12px;
  padding: 48px 40px;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 20px 60px rgba(0,0,0,0.3);
}
.login-title {
  font-size: 24px;
  font-weight: 700;
  text-align: center;
  color: #2C3E50;
  margin-bottom: 4px;
}
.login-sub {
  text-align: center;
  color: var(--text-secondary, #666);
  font-size: 14px;
  margin-bottom: 28px;
}
.login-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary, #555);
}
.field input {
  padding: 12px 14px;
  border: 1px solid var(--border-color, #ddd);
  border-radius: 8px;
  font-size: 15px;
  color: var(--text-primary, #333);
  transition: border-color 0.2s;
}
.field input:focus {
  outline: none;
  border-color: #3498DB;
  box-shadow: 0 0 0 3px rgba(52,152,219,0.15);
}
.error-msg {
  background: #FDF0ED;
  color: #E74C3C;
  padding: 10px 14px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
}
.btn-login {
  padding: 13px;
  background: #2C3E50;
  color: #fff;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
}
.btn-login:hover { background: #34495E; }
.btn-login:disabled { opacity: 0.6; cursor: not-allowed; }
.login-hint {
  text-align: center;
  font-size: 11px;
  color: var(--text-secondary, #999);
  margin-top: 20px;
  line-height: 1.5;
}
</style>
