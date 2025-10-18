<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Neraca</h1>
      <p class="text-sm text-brand-600">
        Komposisi aset, kewajiban, dan ekuitas akan dihitung otomatis. Sementara ini ditampilkan sebagai mock data.
      </p>
    </header>

    <section class="surface-card space-y-5 p-6">
      <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
        <p class="text-sm font-semibold">Preview struktur neraca</p>
        <p class="mt-1 text-xs text-brand-600">
          Setelah modul laporan selesai, tabel ini akan menarik data langsung dari buku besar teragregasi.
        </p>
      </div>
      <div class="grid gap-4 md:grid-cols-3">
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Total Aset</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 86.450.000</p>
        </div>
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Total Kewajiban</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 31.800.000</p>
        </div>
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Total Ekuitas</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 54.650.000</p>
        </div>
      </div>
    </section>
  </div>
</template>

