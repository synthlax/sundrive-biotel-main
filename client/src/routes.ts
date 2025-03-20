import { lazy } from 'solid-js';
import type { RouteDefinition } from '@solidjs/router';

import Home from './pages/home';
import Contact from './pages/contact';

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
    path: '**',
    component: lazy(() => import('./errors/404')),
  },
];
