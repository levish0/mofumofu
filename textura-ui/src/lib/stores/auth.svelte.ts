export class AuthStore {
  token: string;
  constructor() {
    this.token = $state(localStorage.getItem('textura_access_token') || '');
  }

  setToken(token: string) {
    this.token = token;
    localStorage.setItem('textura_access_token', token);
  }

  clearToken() {
    this.token = '';
    localStorage.removeItem('textura_access_token');
  }
}

export const authStore = new AuthStore();
