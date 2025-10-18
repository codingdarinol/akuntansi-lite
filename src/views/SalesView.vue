<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Penjualan</h1>
      <p class="text-sm text-brand-600">
        Pantau invoice dan pembayaran pelanggan. Modul penuh akan memuat pipeline penagihan dan integrasi jurnal otomatis.
      </p>
    </header>

    <section class="surface-card space-y-4 p-6">
      <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
        <p class="text-sm font-semibold">Status</p>
        <p class="mt-1 text-xs text-brand-600">
          Modul Penjualan sedang dalam perancangan. UI ini menjadi placeholder untuk menjaga konsistensi navigasi.
        </p>
      </div>
      <div class="grid gap-4 md:grid-cols-2">
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-700">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Terakhir</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Invoice #INV-024</p>
          <p class="mt-1 text-sm text-brand-600">Jatuh tempo: 02 Feb 2025</p>
        </div>
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-700">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Agenda</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Hubungi 2 pelanggan overdue</p>
          <p class="mt-1 text-sm text-brand-600">Siapkan template pengingat pembayaran.</p>
        </div>
      </div>
      <div class="flex justify-end">
        <button type="button" class="btn-primary">Tambah Invoice Baru</button>
      </div>
    </section>
  </div>
</template>

