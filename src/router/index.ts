import { createRouter, createWebHashHistory } from 'vue-router';
import Dashboard from '../components/Dashboard.vue';
import ExportReport from '../components/ExportReport.vue';
import Settings from '../components/Settings.vue';
import { navItems } from '../constants/nav';

const routes = [
  {
    path: '/',
    redirect: `/${navItems[0].route}`
  },
  {
    path: `/${navItems[0].route}`,
    name: navItems[0].route,
    component: Dashboard
  },
  {
    path: `/${navItems[1].route}`,
    name: navItems[1].route,
    component: ExportReport
  },
  {
    path: `/${navItems[2].route}`,
    name: navItems[2].route,
    component: Settings
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

export default router;
