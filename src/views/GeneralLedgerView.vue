<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Buku Besar</h1>
      <p class="text-sm text-brand-600">
        Lihat ringkasan saldo per akun setelah jurnal diposting. Implementasi penuh akan menampilkan drill-down per akun.
      </p>
    </header>

    <section class="surface-card space-y-5 p-6">
      <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
        <p class="text-sm font-semibold">Sedang disiapkan</p>
        <p class="mt-1 text-xs text-brand-600">
          Buku besar akan memanfaatkan query agregasi di Rust untuk menjaga performa. Sementara itu gunakan laporan dummy ini.
        </p>
      </div>
      <div class="grid gap-4 md:grid-cols-2">
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Aktiva Lancar</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 18.400.000</p>
          <p class="text-xs text-brand-600">Kas, bank, piutang</p>
        </div>
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Modal</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 52.000.000</p>
          <p class="text-xs text-brand-600">Saldo akhir tahun berjalan</p>
        </div>
      </div>
    </section>
  </div>
</template>

