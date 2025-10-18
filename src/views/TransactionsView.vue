<template>
  <div class="space-y-6">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="text-2xl font-semibold text-slate-900">Jurnal Umum</h1>
        <p class="text-sm text-slate-500">
          Catat transaksi manual atau telusuri entri otomatis dari modul lain.
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <input
          type="search"
          placeholder="Cari nomor jurnal..."
          class="rounded-lg border border-slate-200 px-4 py-2 text-sm text-slate-700 shadow-sm focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-200 focus:ring-offset-1"
        />
        <button
          type="button"
          class="inline-flex items-center justify-center rounded-lg bg-brand-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-brand-700"
        >
          + Transaksi Baru
        </button>
      </div>
    </header>

    <section class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
      <div class="hidden grid-cols-[120px,1fr,1fr,1fr,1fr] gap-4 border-b border-slate-200 px-6 py-3 text-xs font-semibold uppercase tracking-wider text-slate-500 md:grid">
        <span>Tanggal</span>
        <span>No. Jurnal</span>
        <span>Deskripsi</span>
        <span>Debit</span>
        <span>Kredit</span>
      </div>

      <div class="divide-y divide-slate-100 text-sm text-slate-700">
        <div class="grid gap-4 px-6 py-4 md:grid-cols-[120px,1fr,1fr,1fr,1fr]">
          <span class="font-medium text-slate-900">02/01/2025</span>
          <span>JU-00015</span>
          <div>
            Penjualan layanan implementasi<br />
            <span class="text-xs text-slate-500">Dari modul Penjualan</span>
          </div>
          <span class="font-medium text-green-600">Rp 6.200.000</span>
          <span class="font-medium text-orange-500">Rp 6.200.000</span>
        </div>
        <div class="px-6 py-4 text-sm text-slate-500">
          📌 Integrasi nyata akan mengambil data langsung dari database. Ini mock untuk validasi UI.
        </div>
      </div>
    </section>
  </div>
</template>
