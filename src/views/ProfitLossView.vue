<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Laporan Laba Rugi</h1>
      <p class="text-sm text-brand-600">
        Placeholder laporan untuk memvisualisasikan struktur pendapatan dan beban sebelum kalkulasi real diimplementasikan.
      </p>
    </header>

    <section class="surface-card space-y-5 p-6">
      <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
        <p class="text-sm font-semibold">Kalkulasi otomatis segera hadir</p>
        <p class="mt-1 text-xs text-brand-600">
          Engine perhitungan akan berjalan di Rust menggunakan data jurnal terposting. UI ini memastikan desain konsisten.
        </p>
      </div>
      <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Pendapatan</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 45.300.000</p>
        </div>
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Beban</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 22.850.000</p>
        </div>
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Laba Bersih</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 22.450.000</p>
        </div>
      </div>
    </section>
  </div>
</template>

