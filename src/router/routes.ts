import type { RouteRecordRaw } from 'vue-router'

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'dashboard',
    component: () => import('@/views/DashboardView.vue'),
    meta: {
      title: 'Dashboard',
      icon: 'LayoutDashboard',
      description: 'Snapshot keuangan dan aktivitas terbaru.',
    },
  },
  {
    path: '/sales',
    name: 'sales',
    component: () => import('@/views/SalesView.vue'),
    meta: {
      title: 'Penjualan',
      icon: 'ShoppingBag',
      description: 'Kelola invoice dan penagihan pelanggan.',
    },
  },
  {
    path: '/purchases',
    name: 'purchases',
    component: () => import('@/views/PurchasesView.vue'),
    meta: {
      title: 'Pembelian',
      icon: 'Package',
      description: 'Catat faktur pemasok dan pembayaran utang.',
    },
  },
  {
    path: '/common',
    name: 'common',
    component: () => import('@/views/CommonView.vue'),
    meta: {
      title: 'Master Data',
      icon: 'FolderKanban',
      description: 'Manajemen data pelanggan, pemasok, produk, dan aset.',
    },
  },
  {
    path: '/accounts',
    name: 'accounts',
    component: () => import('@/views/AccountsView.vue'),
    meta: {
      title: 'Bagan Akun',
      icon: 'ListTree',
      description: 'Struktur akun akuntansi perusahaan.',
    },
  },
  {
    path: '/transactions',
    name: 'transactions',
    component: () => import('@/views/TransactionsView.vue'),
    meta: {
      title: 'Jurnal Umum',
      icon: 'Notebook',
      description: 'Daftar entri jurnal manual dan otomatis.',
    },
  },
  {
    path: '/general-ledger',
    name: 'general-ledger',
    component: () => import('@/views/GeneralLedgerView.vue'),
    meta: {
      title: 'Buku Besar',
      icon: 'BookOpen',
      description: 'Ringkasan saldo per akun setelah posting.',
    },
  },
  {
    path: '/reports',
    name: 'reports',
    component: () => import('@/views/ReportsView.vue'),
    meta: {
      title: 'Laporan',
      icon: 'PieChart',
      description: 'Daftar laporan keuangan penting.',
    },
  },
  {
    path: '/profit-loss',
    name: 'profit-loss',
    component: () => import('@/views/ProfitLossView.vue'),
    meta: {
      title: 'Laba Rugi',
      icon: 'TrendingUp',
      description: 'Analisis profitabilitas per periode.',
    },
  },
  {
    path: '/balance-sheet',
    name: 'balance-sheet',
    component: () => import('@/views/BalanceSheetView.vue'),
    meta: {
      title: 'Neraca',
      icon: 'PieChart',
      description: 'Komposisi aset, kewajiban, dan ekuitas.',
    },
  },
  {
    path: '/trial-balance',
    name: 'trial-balance',
    component: () => import('@/views/TrialBalanceView.vue'),
    meta: {
      title: 'Trial Balance',
      icon: 'Table',
      description: 'Verifikasi keseimbangan debit dan kredit.',
    },
  },
  {
    path: '/gst',
    name: 'gst',
    component: () => import('@/views/GstView.vue'),
    meta: {
      title: 'GST',
      icon: 'Receipt',
      description: 'Ringkasan kewajiban pajak periode berjalan.',
    },
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('@/views/SettingsView.vue'),
    meta: {
      title: 'Pengaturan',
      icon: 'Settings',
      description: 'Preferensi aplikasi dan utilitas database.',
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
