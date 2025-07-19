import { browser } from '$app/environment';

export class AuthStore {
  private static instance: AuthStore;
  private _token = $state('');
  private _initialized = false;

  private constructor() {
    if (browser) {
      this.init();
    }
  }

  static getInstance(): AuthStore {
    if (!AuthStore.instance) {
      AuthStore.instance = new AuthStore();
    }
    return AuthStore.instance;
  }

  private init() {
    if (this._initialized) return;

    this._token = localStorage.getItem('textura_access_token') || '';
    this._initialized = true;
  }

  ensureInitialized() {
    if (browser && !this._initialized) {
      this.init();
    }
  }

  get token(): string {
    this.ensureInitialized();
    return this._token;
  }

  setToken(token: string) {
    this.ensureInitialized();
    this._token = token;
    if (browser) {
      localStorage.setItem('textura_access_token', token);
    }
  }

  clearToken() {
    this.ensureInitialized();
    this._token = '';
    if (browser) {
      localStorage.removeItem('textura_access_token');
    }
  }

  get isAuthenticated(): boolean {
    this.ensureInitialized();
    return this._token !== '';
  }
}

export const authStore = AuthStore.getInstance();
