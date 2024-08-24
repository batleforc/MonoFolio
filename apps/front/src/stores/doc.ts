import { DocCategory, getDocSidebar } from '@portfolio/api-client';
import { defineStore } from 'pinia';

export interface DocState {
  inited: boolean;
  docLoading: boolean;
  loadingError?: string;
  docContent?: DocCategory;
}

export const useDocStore = defineStore({
  id: 'doc',
  state: (): DocState => ({
    inited: false,
    docLoading: false,
  }),
  actions: {
    init() {
      if (this.inited) return;
      this.docLoading = true;
      getDocSidebar()
        .then((body) => {
          if (body.status === 200) {
            this.docContent = body.data;
          }
        })
        .catch((err) => {
          console.error(err);
          this.loadingError = 'Failed to load doc sidebar';
        })
        .finally(() => {
          this.docLoading = false;
          this.inited = true;
        });
    },
  },
});
