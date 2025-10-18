<script setup lang="ts">
interface ReportCard {
  title: string
  description: string
  accent: string
}

const reportCards: ReportCard[] = [
  {
    title: 'Laporan Laba Rugi',
    description: 'Melihat profitabilitas dalam periode tertentu.',
    accent: 'border-l-4 border-brand-500',
  },
  {
    title: 'Neraca',
    description: 'Posisi aset, kewajiban, dan ekuitas terbaru.',
    accent: 'border-l-4 border-brand-400',
  },
  {
    title: 'Arus Kas',
    description: 'Aliran kas operasional, investasi, dan pendanaan.',
    accent: 'border-l-4 border-accent-400',
  },
  {
    title: 'Perubahan Ekuitas',
    description: 'Pergerakan modal pemilik sepanjang periode.',
    accent: 'border-l-4 border-brand-300',
  },
  {
    title: 'Analisis Rasio',
    description: 'Pantau rasio likuiditas dan profitabilitas penting.',
    accent: 'border-l-4 border-brand-200',
  },
  {
    title: 'Analisis Beban',
    description: 'Visualisasi komposisi beban operasional utama.',
    accent: 'border-l-4 border-accent-200',
  },
]
</script>

<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Laporan Keuangan</h1>
      <p class="text-sm text-brand-600">
        Pilih laporan untuk menganalisis performa bisnis. Semua laporan akan dihitung di sisi Rust untuk memastikan akurasi.
      </p>
    </header>

    <section class="grid gap-5 md:grid-cols-2 xl:grid-cols-3">
      <article
        v-for="report in reportCards"
        :key="report.title"
        class="surface-card relative overflow-hidden p-6 text-brand-800"
        :class="report.accent"
      >
        <h2 class="text-lg font-semibold text-brand-900">{{ report.title }}</h2>
        <p class="mt-3 text-sm text-brand-600">{{ report.description }}</p>
        <button type="button" class="mt-6 btn-muted text-xs">Buka laporan</button>
      </article>
    </section>
  </div>
</template>

