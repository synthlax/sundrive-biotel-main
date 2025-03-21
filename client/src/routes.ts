import { lazy } from 'solid-js';
import type { RouteDefinition } from '@solidjs/router';

import Home from './pages/home';
import Contact from './pages/contact';
import Login from './pages/login';
import Docs from './pages/docs';

export const routes: RouteDefinition[] = [
  {
    path: '/',
    component: Home,
  },
  {
    path: '/contact',
    component: Contact,
  },
  {
    path: '/login',
    component: Login
  },
  {
    path: '/docs',
    component: Docs
  },
  {
    path: '**',
    component: lazy(() => import('./errors/404')),
  },
];
