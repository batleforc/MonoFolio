import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';

declare global {
  interface Window {
    umami?: {
      track?: (object) => void;
    };
  }
}
import HomeViewV2 from '../viewsV2/HomeView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeViewV2,
      meta: {
        hero: true,
      },
    },
    {
      path: '/old_home',
      name: 'home_old',
      component: HomeView,
      meta: {
        hero: true,
      },
    },
    {
      path: '/doc',
      meta: {
        track: false,
      },
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
      meta: {
        track: false,
      },
      children: [
        {
          name: 'blog',
          path: '',
          component: () => import('../viewsV2/BlogHomeView.vue'),
        },
        {
          name: 'blogcontent',
          path: ':page+',
          component: () => import('../views/BlogView.vue'),
        },
      ],
    },
    {
      path: '/project',
      meta: {
        track: false,
      },
      children: [
        {
          name: 'project',
          path: '',
          component: () => import('../views/418View.vue'),
        },
        {
          name: 'projectcontent',
          path: ':page+',
          component: () => import('../views/418View.vue'),
        },
      ],
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
      component: () => import('../views/404View.vue'),
    },
  ],
});

router.beforeEach((to, from, next) => {
  if (window.umami?.track !== undefined && to.meta.track !== false) {
    window.umami.track((props) => ({
      ...props,
      url: to.path,
      title: to.name || props.title,
    }));
  }
  next();
});

export default router;
