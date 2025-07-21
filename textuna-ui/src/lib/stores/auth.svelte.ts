import { BROWSER } from 'esm-env';

let _token = $state(BROWSER ? localStorage.getItem('textura_access_token') || '' : '');

export const authStore = {
  get token() {
    return _token;
  },
  get isAuthenticated() {
    return _token !== '';
  },
  setToken(token: string) {
    _token = token;
    if (BROWSER) {
      localStorage.setItem('textura_access_token', token);
    }
  },
  clearToken() {
    _token = '';
    if (BROWSER) {
      localStorage.removeItem('textura_access_token');
    }
  }
};