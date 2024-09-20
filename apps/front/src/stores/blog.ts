import { BlogTimeline, getTimeline, PageShort } from '@portfolio/api-client';
import { defineStore } from 'pinia';

export interface BlogState {
  inited: boolean;
  blogLoading: boolean;
  loadingError?: string;
  blogContent: BlogTimeline;
}

export const useBlogStore = defineStore({
  id: 'blog',
  state: (): BlogState => ({
    inited: false,
    blogLoading: false,
    blogContent: {
      pages: {},
    },
  }),
  getters: {
    getBlogContentUnordered(): BlogTimeline | undefined {
      if (!this.blogContent) return;
      return this.blogContent.pages;
    },
    getBlogContent(): [string, PageShort][] | undefined {
      if (!this.blogContent) return;
      return Object.entries((this.blogContent as BlogTimeline).pages).sort(
        (previousDate, nextDate) => {
          const aDate = new Date(previousDate[1].metadata.date);
          const bDate = new Date(nextDate[1].metadata.date);
          return aDate > bDate ? -1 : aDate < bDate ? 1 : 0;
        },
      );
    },
  },
  actions: {
    init() {
      if (this.inited) return Promise.reject();
      this.blogLoading = true;
      return getTimeline()
        .then((body) => {
          if (body.status === 200) {
            this.blogContent = body.data;
          }
          return body.data;
        })
        .catch((err) => {
          console.error(err);
          this.loadingError = 'Failed to load blog timeline';
        })
        .finally(() => {
          this.blogLoading = false;
          this.inited = true;
        });
    },
  },
});
