import type { RouteRecordRaw } from 'vue-router'

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'dashboard',
    component: () => import('@/views/DashboardView.vue'),
    meta: {
      title: 'Dashboard',
      icon: 'LayoutDashboard',
    },
  },
  {
    path: '/accounts',
    name: 'accounts',
    component: () => import('@/views/AccountsView.vue'),
    meta: {
      title: 'Bagan Akun',
      icon: 'ListTree',
    },
  },
  {
    path: '/transactions',
    name: 'transactions',
    component: () => import('@/views/TransactionsView.vue'),
    meta: {
      title: 'Jurnal Umum',
      icon: 'Notebook',
    },
  },
  {
    path: '/reports',
    name: 'reports',
    component: () => import('@/views/ReportsView.vue'),
    meta: {
      title: 'Laporan',
      icon: 'PieChart',
    },
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('@/views/SettingsView.vue'),
    meta: {
      title: 'Pengaturan',
      icon: 'Settings',
    },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: () => import('@/views/NotFoundView.vue'),
    meta: {
      title: 'Halaman tidak ditemukan',
      hiddenInMenu: true,
    },
  },
]

export default routes
