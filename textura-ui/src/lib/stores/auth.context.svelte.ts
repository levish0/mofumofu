// auth.context.svelte.ts
import { getContext, setContext } from 'svelte';

const AUTH_CONTEXT_KEY = Symbol('auth');

export class AuthStore {
  token: string;

  constructor() {
    this.token = $state('');
  }

  init() {
    if (typeof window !== 'undefined') {
      this.token = localStorage.getItem('textura_access_token') || '';
    }
  }

  setToken(token: string) {
    this.token = token;
    if (typeof window !== 'undefined') {
      localStorage.setItem('textura_access_token', token);
    }
  }

  clearToken() {
    this.token = '';
    if (typeof window !== 'undefined') {
      localStorage.removeItem('textura_access_token');
    }
  }

  get isAuthenticated(): boolean {
    return this.token !== '';
  }
}

export function createAuthContext(): AuthStore {
  const authStore = new AuthStore();
  setContext(AUTH_CONTEXT_KEY, authStore);
  return authStore;
}

export function getAuthStore(): AuthStore {
  const store = getContext<AuthStore>(AUTH_CONTEXT_KEY);
  if (!store) {
    throw new Error('AuthStore not found. Make sure to call createAuthContext() in a parent component.');
  }
  return store;
}
