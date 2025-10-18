<script setup lang="ts">
import type { ApexOptions } from 'apexcharts'
import { computed, ref } from 'vue'

const kpiCards = [
  {
    label: 'Pendapatan Kotor',
    value: 'Rp 7.200.000',
    delta: '+12,4% vs bulan lalu',
    accent: 'border-t-4 border-brand-500',
  },
  {
    label: 'Pengeluaran Operasional',
    value: 'Rp 3.640.000',
    delta: '-3,2% penghematan',
    accent: 'border-t-4 border-brand-300',
  },
  {
    label: 'Saldo Kas & Bank',
    value: 'Rp 12.500.000',
    delta: 'Likuiditas 6,4 bulan',
    accent: 'border-t-4 border-accent-400',
  },
  {
    label: 'Invoice Belum Dibayar',
    value: 'Rp 4.820.000',
    delta: '5 pelanggan menunggu',
    accent: 'border-t-4 border-accent-200',
  },
]

const series = ref([
  {
    name: 'Pendapatan',
    data: [4200000, 4800000, 5200000, 6800000, 6400000, 7200000],
  },
  {
    name: 'Pengeluaran',
    data: [2100000, 2400000, 2600000, 3200000, 3000000, 3600000],
  },
])

const chartOptions = computed<ApexOptions>(() => ({
  chart: {
    type: 'area',
    toolbar: { show: false },
    fontFamily: 'Inter, system-ui, sans-serif',
    foreColor: '#3d5a89',
  },
  colors: ['#3d5a89', '#f6b713'],
  stroke: {
    curve: 'smooth',
    width: 3,
  },
  dataLabels: { enabled: false },
  xaxis: {
    categories: ['Jan', 'Feb', 'Mar', 'Apr', 'Mei', 'Jun'],
    labels: { style: { colors: '#667ea6' } },
    axisBorder: { show: false },
    axisTicks: { show: false },
  },
  yaxis: {
    labels: {
      formatter: (value: number) => new Intl.NumberFormat('id-ID', { notation: 'compact', maximumFractionDigits: 1 }).format(value),
      style: { colors: '#92a2c1' },
    },
  },
  legend: {
    position: 'top',
    horizontalAlign: 'left',
    labels: { colors: '#3d5a89' },
  },
  fill: {
    opacity: 0.15,
  },
  grid: {
    borderColor: 'rgba(61, 90, 137, 0.12)',
    strokeDashArray: 6,
  },
  tooltip: {
    theme: 'light',
    y: {
      formatter: (value: number) => new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', maximumFractionDigits: 0 }).format(value),
    },
  },
}))

const timeline = [
  { title: 'Invoice #INV-023 dibayar', time: '5 menit lalu', amount: '+ Rp1.200.000', tone: 'text-brand-600' },
  { title: 'Pengeluaran kantor - ATK', time: '32 menit lalu', amount: '- Rp220.000', tone: 'text-accent-600' },
  { title: 'Reminder: Pajak bulanan', time: '1 jam lalu', amount: 'Jatuh tempo 5 hari lagi', tone: 'text-brand-500' },
]
</script>

<template>
  <div class="space-y-8">
    <section class="card-grid">
      <article
        v-for="card in kpiCards"
        :key="card.label"
        class="surface-card px-6 py-7"
        :class="card.accent"
      >
        <p class="text-xs uppercase tracking-[0.3em] text-brand-400">{{ card.label }}</p>
        <p class="mt-4 text-3xl font-semibold text-brand-900">{{ card.value }}</p>
        <p class="mt-2 text-sm text-brand-600">{{ card.delta }}</p>
      </article>
    </section>

    <section class="grid gap-6 xl:grid-cols-[2fr,1fr]">
      <article class="surface-card overflow-hidden p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Arus Kas 6 Bulan</p>
            <h2 class="mt-2 text-xl font-semibold text-brand-900">Tren pendapatan vs pengeluaran</h2>
          </div>
          <div class="flex items-center gap-2 text-xs text-brand-500">
            <span class="rounded-full border border-brand-200 px-3 py-1">Semester 1</span>
            <span class="rounded-full border border-brand-200 px-3 py-1">2025</span>
          </div>
        </div>
        <apexchart class="mt-6" type="area" height="320" :options="chartOptions" :series="series" />
      </article>

      <article class="surface-card flex flex-col gap-5 p-6">
        <header class="flex items-center justify-between">
          <div>
            <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Aktivitas Terkini</p>
            <h3 class="mt-2 text-lg font-semibold text-brand-900">Terakhir 2 jam</h3>
          </div>
          <button type="button" class="btn-muted text-xs">Lihat semua</button>
        </header>
        <div class="space-y-4">
          <div
            v-for="item in timeline"
            :key="item.title"
            class="flex items-center justify-between rounded-4xl border border-brand-100 bg-brand-50 px-4 py-3"
          >
            <div>
              <p class="text-sm font-medium text-brand-900">{{ item.title }}</p>
              <p class="text-xs text-brand-500">{{ item.time }}</p>
            </div>
            <span class="text-sm font-semibold" :class="item.tone">{{ item.amount }}</span>
          </div>
        </div>
      </article>
    </section>

    <section class="grid gap-6 lg:grid-cols-3">
      <article class="surface-card col-span-2 flex flex-col gap-6 p-6">
        <header class="flex items-center justify-between">
          <div>
            <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Proyeksi Cashflow</p>
            <h3 class="mt-2 text-lg font-semibold text-brand-900">Prioritas tindakan minggu ini</h3>
          </div>
          <button type="button" class="btn-muted text-xs">Unduh Excel</button>
        </header>
        <div class="grid gap-4 md:grid-cols-2">
          <div class="glass-panel border border-brand-100 bg-brand-50 p-5 text-brand-800">
            <p class="text-sm text-brand-600">Pembayaran supplier jatuh tempo</p>
            <p class="mt-3 text-2xl font-semibold text-brand-900">Rp 1.450.000</p>
            <p class="mt-2 text-xs uppercase tracking-[0.3em] text-brand-400">3 Faktur minggu ini</p>
          </div>
          <div class="glass-panel border border-brand-100 bg-brand-50 p-5 text-brand-800">
            <p class="text-sm text-brand-600">Invoice akan kedaluwarsa</p>
            <p class="mt-3 text-2xl font-semibold text-brand-900">Rp 980.000</p>
            <p class="mt-2 text-xs uppercase tracking-[0.3em] text-brand-400">Hubungi 2 pelanggan</p>
          </div>
        </div>
      </article>

      <article class="surface-card flex flex-col gap-4 p-6">
        <header>
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Quick Actions</p>
          <h3 class="mt-2 text-lg font-semibold text-brand-900">Mulai pekerjaan</h3>
        </header>
        <div class="space-y-3">
          <button type="button" class="w-full rounded-4xl border border-brand-200 bg-brand-50 px-4 py-3 text-left text-sm font-semibold text-brand-800 transition hover:bg-brand-100">
            + Catat Penjualan Tunai
          </button>
          <button type="button" class="w-full rounded-4xl border border-brand-200 bg-white px-4 py-3 text-left text-sm text-brand-700 transition hover:bg-brand-50">
            + Input Pembelian Supplier
          </button>
          <button type="button" class="w-full rounded-4xl border border-brand-200 bg-white px-4 py-3 text-left text-sm text-brand-700 transition hover:bg-brand-50">
            + Jurnal Penyesuaian
          </button>
        </div>
      </article>
    </section>
  </div>
</template>

<style scoped>
:global(.apexcharts-tooltip) {
  font-family: 'Inter', system-ui, sans-serif !important;
}
</style>



