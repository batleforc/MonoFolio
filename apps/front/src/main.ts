import './styles.scss';
import { createApp, markRaw } from 'vue';
import { createPinia } from 'pinia';
import type { Router } from 'vue-router';
import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';

import App from './views/App.vue';
import router from './router';
import { client } from '@portfolio/api-client';

client.setConfig({
  baseURL: import.meta.env.VITE_API_URL as string,
});

const app = createApp(App);

const pinia = createPinia();

declare const RawSymbol: unique symbol;
declare module 'pinia' {
  export interface PiniaCustomProperties {
    // by using a setter we can allow both strings and refs
    router: Router & { [RawSymbol]?: true | undefined };
    language: string;
  }
}
pinia.use(({ store }) => {
  store.router = markRaw(router);
});

app.use(pinia);
app.use(router);

app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      cssLayer: {
        name: 'primevue',
        order: 'tailwind-base, primevue, tailwind-utilities',
      },
      darkModeSelector: '.dark-mode',
    },
  },
});

app.mount('#root');
