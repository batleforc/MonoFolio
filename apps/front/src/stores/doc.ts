import { DocCategory, getDocSidebar, PageShort } from '@portfolio/api-client';
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
      if (this.inited) return Promise.reject();
      this.docLoading = true;
      return getDocSidebar()
        .then((body) => {
          if (body.status === 200) {
            this.docContent = body.data;
          }
          return body.data;
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
    searchCategory(path: string): DocCategory | undefined {
      if (!this.docContent) return;
      return this.searchCategoryRecursive(`content/${path}`, this.docContent);
    },
    searchCategoryRecursive(
      path: string,
      category: DocCategory,
    ): DocCategory | undefined {
      const path_split = path.split('/');
      if (path_split.length === 1 && category.name === path_split[0]) {
        return category;
      }
      for (const subCategory of category.sub_categories) {
        if (subCategory.name === path_split[1]) {
          const found = this.searchCategoryRecursive(
            path_split.slice(1).join('/'),
            subCategory,
          );
          if (found) return found;
        }
      }
      return;
    },
    technoExists(techno: string): PageShort | undefined {
      if (!this.docContent) return;
      return this.technoExistsRecursive(techno, this.docContent);
    },
    technoExistsRecursive(
      techno: string,
      category: DocCategory,
    ): PageShort | undefined {
      for (const page of category.pages) {
        if (page.name.toLowerCase() === techno.toLowerCase()) {
          return page;
        }
      }
      for (const subCategory of category.sub_categories) {
        const found = this.technoExistsRecursive(techno, subCategory);
        if (found) return found;
      }
      return;
    },
  },
});
