import '@/styles/tokens.css'
import '@/styles/global.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

// ── Auth init: set API token in localStorage if not already present ──
// Change token here or in config.toml. Clear localStorage to force re-init.
const TOKEN_KEY = 'LW_API_TOKEN'
const DEFAULT_TOKEN = 'lw-secret-token-change-me'
if (!localStorage.getItem(TOKEN_KEY)) {
  localStorage.setItem(TOKEN_KEY, DEFAULT_TOKEN)
}

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
