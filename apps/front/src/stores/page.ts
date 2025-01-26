import { getPage, getPageV2, Page, PageV2 } from '@portfolio/api-client';
import { defineStore } from 'pinia';
import { useDocStore } from './doc';

export interface PageState {
  pageLoading: boolean;
  loadingError?: string;
  page?: Page;
  pageV2?: PageV2;
  pagePath: string;
  v2Switch: boolean;
}

export const usePageStore = defineStore('page', {
  state: (): PageState => ({
    pageLoading: false,
    pagePath: '',
    v2Switch: true,
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
      if (this.v2Switch) {
        return getPageV2({ query: { path } })
          .then((body) => {
            if (body.status === 200) {
              this.pageV2 = body.data;
              this.page = undefined;
            }
            this.pagePath = path;
          })
          .catch((err) => {
            console.error(err);
            this.loadingError = 'Failed to load page';
            this.pagePath = '';
            this.page = undefined;
            this.pageV2 = undefined;
          })
          .finally(() => {
            this.pageLoading = false;
          });
      }
      return getPage({ query: { path } })
        .then((body) => {
          if (body.status === 200) {
            this.pageV2 = undefined;
            this.page = body.data;
            this.pagePath = path;
          }
        })
        .catch((err) => {
          console.error(err);
          this.loadingError = 'Failed to load page';
          this.pagePath = '';
          this.page = undefined;
          this.pageV2 = undefined;
        })
        .finally(() => {
          this.pageLoading = false;
        });
    },
  },
});
