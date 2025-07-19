import { browser } from '$app/environment';

export class AuthStore {
  private static instance: AuthStore;
  private _token = $state('');
  private _initialized = false;

  private constructor() {
    // 브라우저 환경에서만 초기화
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

  // 서버 사이드에서 안전하게 호출할 수 있도록
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

// 싱글톤 인스턴스 export
export const authStore = AuthStore.getInstance();