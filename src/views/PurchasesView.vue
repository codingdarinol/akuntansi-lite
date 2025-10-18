<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Pembelian</h1>
      <p class="text-sm text-brand-600">
        Catat faktur pemasok, kelola jadwal pembayaran, dan sinkronkan otomatis dengan jurnal beban.
      </p>
    </header>

    <section class="surface-card space-y-5 p-6">
      <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
        <p class="text-sm font-semibold">Dalam pengembangan</p>
        <p class="mt-1 text-xs text-brand-600">
          Modul pembelian akan hadir setelah MVP akuntansi inti selesai. Gunakan catatan manual sementara waktu.
        </p>
      </div>
      <div class="grid gap-4 md:grid-cols-3">
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Faktur jatuh tempo</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 1.120.000</p>
          <p class="text-xs text-brand-600">3 pemasok menunggu pembayaran</p>
        </div>
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Beban bulan ini</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Rp 2.450.000</p>
          <p class="text-xs text-brand-600">Estimasi sebelum penutupan buku</p>
        </div>
        <div class="glass-panel border-brand-200 bg-brand-50 p-5 text-brand-900">
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Penanggung jawab</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">Tim Keuangan</p>
          <p class="text-xs text-brand-600">Verifikasi setiap Rabu</p>
        </div>
      </div>
      <div class="flex justify-end">
        <button type="button" class="btn-muted">Lihat template faktur</button>
      </div>
    </section>
  </div>
</template>

