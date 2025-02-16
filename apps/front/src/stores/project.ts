import { getProjectList, PageShort, ProjectList } from '@portfolio/api-client';
import { defineStore } from 'pinia';

export interface ProjectState {
  isInitialized: boolean;
  projectLoading: boolean;
  loadingError?: string;
  projectContent: ProjectList;
}

export const useProjectStore = defineStore('project', {
  state: (): ProjectState => ({
    isInitialized: false,
    projectLoading: false,
    projectContent: {
      projects: {},
    },
  }),
  getters: {
    getProjectContentUnordered(): ProjectList | undefined {
      if (!this.projectContent) return;
      return this.projectContent.pages;
    },
    getProjectContent(): [string, PageShort][] | undefined {
      if (!this.projectContent) return;
      return Object.entries((this.projectContent as ProjectList).projects).sort(
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
      if (this.isInitialized) return Promise.reject();
      this.projectLoading = true;
      return getProjectList()
        .then((body) => {
          if (body.status === 200) {
            this.projectContent = body.data;
          }
          return body.data;
        })
        .catch((err) => {
          console.error(err);
          this.loadingError = 'Failed to load project list';
        })
        .finally(() => {
          this.projectLoading = false;
          this.isInitialized = true;
        });
    },
  },
});
