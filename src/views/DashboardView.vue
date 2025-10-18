<script setup lang="ts">
import type { ApexOptions } from 'apexcharts'
import { computed, ref } from 'vue'

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
  },
  colors: ['#326ffc', '#1d55d8'],
  stroke: {
    curve: 'smooth',
    width: 2,
  },
  dataLabels: { enabled: false },
  xaxis: {
    categories: ['Jan', 'Feb', 'Mar', 'Apr', 'Mei', 'Jun'],
    labels: { style: { colors: '#64748b' } },
  },
  yaxis: {
    labels: {
      formatter: (value: number) =>
        new Intl.NumberFormat('id-ID', { notation: 'compact', maximumFractionDigits: 1 }).format(
          value,
        ),
      style: { colors: '#64748b' },
    },
  },
  legend: {
    position: 'top',
    horizontalAlign: 'left',
  },
  fill: {
    type: 'gradient',
    gradient: {
      shadeIntensity: 1,
      opacityFrom: 0.4,
      opacityTo: 0.05,
      stops: [0, 90, 100],
    },
  },
  grid: {
    borderColor: '#e2e8f0',
    strokeDashArray: 4,
  },
}))
</script>

<template>
  <div class="space-y-6">
    <section class="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
      <article
        v-for="card in [
          { label: 'Pendapatan Bulan Ini', value: 'Rp 7,2 jt', trend: '+12,4%' },
          { label: 'Pengeluaran Bulan Ini', value: 'Rp 3,6 jt', trend: '-3,2%' },
          { label: 'Saldo Kas & Bank', value: 'Rp 12,5 jt', trend: '+1,9%' },
          { label: 'Invoice Belum Dibayar', value: 'Rp 4,8 jt', trend: '5 pelanggan' },
        ]"
        :key="card.label"
        class="rounded-xl border border-slate-200 bg-white p-5 shadow-sm"
      >
        <p class="text-xs font-medium uppercase tracking-wide text-slate-500">{{ card.label }}</p>
        <p class="mt-3 text-2xl font-semibold text-slate-900">{{ card.value }}</p>
        <p class="mt-2 text-sm text-brand-600">{{ card.trend }}</p>
      </article>
    </section>

    <section class="grid gap-4 lg:grid-cols-3">
      <article class="rounded-xl border border-slate-200 bg-white p-6 shadow-sm lg:col-span-2">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-lg font-semibold text-slate-900">Ringkasan Arus Kas</h2>
            <p class="text-sm text-slate-500">Perbandingan pendapatan dan pengeluaran 6 bulan</p>
          </div>
          <div class="text-sm text-slate-500">Semester 1 - 2025</div>
        </div>
        <apexchart
          class="mt-6"
          type="area"
          height="320"
          :options="chartOptions"
          :series="series"
        />
      </article>

      <article class="rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
        <h2 class="text-lg font-semibold text-slate-900">Aksi Cepat</h2>
        <p class="mt-1 text-sm text-slate-500">
          Langkah populer untuk memulai pekerjaan pembukuan.
        </p>
        <div class="mt-4 space-y-3">
          <button
            type="button"
            class="w-full rounded-lg border border-dashed border-brand-300 bg-brand-50/80 px-4 py-3 text-left text-sm font-medium text-brand-700 transition hover:bg-brand-100"
          >
            + Catat Penjualan Baru
          </button>
          <button
            type="button"
            class="w-full rounded-lg border border-slate-200 px-4 py-3 text-left text-sm text-slate-600 transition hover:border-brand-200 hover:text-brand-600"
          >
            + Catat Pembelian
          </button>
          <button
            type="button"
            class="w-full rounded-lg border border-slate-200 px-4 py-3 text-left text-sm text-slate-600 transition hover:border-brand-200 hover:text-brand-600"
          >
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
