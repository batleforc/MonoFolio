import { getPage, Page } from '@portfolio/api-client';
import { defineStore } from 'pinia';
import { useDocStore } from './doc';

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

      if (this.pagePath === path) {
        this.pageLoading = false;
        return new Promise<void>((resolve) => resolve());
      }
      const doc = useDocStore();
      if (!doc.inited) {
        return doc.init().then(() => this.fetchPage(path));
      }
      this.pageLoading = true;
      const category = doc.searchCategory(path);
      if (category && category.has_index) {
        path = `${path}/index`;
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
          this.pagePath = '';
          this.page = undefined;
        })
        .finally(() => {
          this.pageLoading = false;
        });
    },
  },
});
