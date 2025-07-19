import { writable } from 'svelte/store';

const createAuthStore = () => {
  const { subscribe, set } = writable<string | null>(null);

  return {
    subscribe,
    setToken: (token: string) => {
      if (typeof window !== 'undefined') {
        localStorage.setItem('textura_access_token', token);
      }
      set(token);
    },
    clearToken: () => {
      if (typeof window !== 'undefined') {
        localStorage.removeItem('textura_access_token');
      }
      set(null);
    },
    initialize: () => {
      if (typeof window !== 'undefined') {
        const token = localStorage.getItem('textura_access_token');
        set(token);
      }
    }
  };
};

export const auth = createAuthStore();
auth.initialize();
