import { defineStore } from 'pinia';
import { getHome, HomeContent } from '@portfolio/api-client';
export interface IndexState {
  inited: boolean;
  homeLoading: boolean;
  loadingError?: string;
  homeContent?: HomeContent;
}

export const useIndexStore = defineStore({
  id: 'index',
  state: (): IndexState => ({
    inited: false,
    homeLoading: false,
  }),
  actions: {
    init() {
      if (this.inited) return;
      this.homeLoading = true;
      getHome()
        .then((body) => {
          if (body.status === 200) {
            this.homeContent = body.data;
          }
          console.log(body);
        })
        .catch((err) => {
          console.error(err);
          this.loadingError = 'Failed to load home content';
        })
        .finally(() => {
          this.homeLoading = false;
          this.inited = true;
        });
    },
  },
});
