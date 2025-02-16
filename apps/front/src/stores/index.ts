import { defineStore } from 'pinia';
import { getHome, HomeContent, HomeHistory } from '@portfolio/api-client';
export interface IndexState {
  isInitialized: boolean;
  homeLoading: boolean;
  loadingError?: string;
  homeContent?: HomeContent;
  title: string;
}

export const useIndexStore = defineStore('index', {
  state: (): IndexState => ({
    isInitialized: false,
    homeLoading: false,
    title: 'Maxime Leriche - Portfolio',
  }),
  getters: {
    getHistory(): HomeHistory[] | undefined {
      return this.homeContent?.history.sort(
        (a: HomeHistory, b: HomeHistory) => b.weight - a.weight,
      );
    },
    getPresentation(): string | undefined {
      return this.homeContent?.presentation.split('\n');
    },
    getTitle(): string {
      return this.title;
    },
  },
  actions: {
    setTitle(title: string) {
      if (this.title === title) return;
      this.title = title;
      document.title = title;
    },
    init() {
      if (this.isInitialized) return;
      this.homeLoading = true;
      getHome()
        .then((body) => {
          if (body.status === 200) {
            this.homeContent = body.data;
          }
        })
        .catch((err) => {
          console.error(err);
          this.loadingError = 'Failed to load home content';
        })
        .finally(() => {
          this.homeLoading = false;
          this.isInitialized = true;
        });
    },
  },
});
