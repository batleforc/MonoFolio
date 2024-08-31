import { getPage, Page } from '@portfolio/api-client';
import { defineStore } from 'pinia';

export interface PageState {
  pageLoading: boolean;
  loadingError?: string;
  page?: Page;
  pagePath: string;
}

export const usePageStore = defineStore({
  id: 'page',
  state: (): PageState => ({
    pageLoading: false,
    pagePath: '',
  }),
  actions: {
    fetchPage(path: string) {
      if (this.pageLoading) return new Promise<void>((resolve) => resolve());
      this.pageLoading = true;
      if (this.pagePath === path) {
        this.pageLoading = false;
        return new Promise<void>((resolve) => resolve());
      }
      return getPage({ query: { path } })
        .then((body) => {
          if (body.status === 200) {
            this.page = body.data;
            this.pagePath = path;
          }
        })
        .catch((err) => {
          console.error(err);
          this.loadingError = 'Failed to load page';
        })
        .finally(() => {
          this.pageLoading = false;
        });
    },
  },
});
