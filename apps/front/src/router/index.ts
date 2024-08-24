import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      meta: {
        hero: true,
      },
    },
    {
      path: '/doc',
      children: [
        {
          name: 'doc',
          path: '',
          component: () => import('../views/DocHomeView.vue'),
        },
        {
          name: 'doccontent',
          path: ':page+',
          component: () => import('../views/DocView.vue'),
        },
      ],
    },
    {
      path: '/blog',
      name: 'blog',
      component: () => import('../views/BlogView.vue'),
    },
  ],
});

export default router;
