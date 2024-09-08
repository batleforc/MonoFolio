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
    {
      path: '/pong',
      name: 'pong',
      component: () => import('../views/PongView.vue'),
    },
    {
      path: '/418',
      name: '418',
      component: () => import('../views/418View.vue'),
    },
    {
      path: '/404',
      name: '404',
      component: () => import('../views/404View.vue'),
    },
    {
      path: '/:pathMatch(.*)*',
      redirect: { name: '404' },
    },
  ],
});

export default router;
