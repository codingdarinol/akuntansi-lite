<template>
  <div class="space-y-7">
    <header class="flex flex-wrap items-center justify-between gap-4">
      <div>
        <h1 class="text-3xl font-semibold text-brand-900">Jurnal Umum</h1>
        <p class="text-sm text-brand-600">
          Lihat dan catat transaksi double-entry di seluruh modul. Setiap entri terhubung langsung dengan buku besar.
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <button type="button" class="btn-muted">Impor CSV</button>
        <button type="button" class="btn-primary">+ Transaksi Manual</button>
      </div>
    </header>

    <section class="surface-card space-y-6 p-6">
      <div class="flex flex-wrap items-center gap-3">
        <input
          type="search"
          placeholder="Cari nomor jurnal..."
          class="w-full rounded-full border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-800 placeholder:text-brand-300 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60 sm:w-64"
        />
        <select
          class="rounded-full border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-800 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
        >
          <option>Periode berjalan</option>
          <option>Bulan lalu</option>
          <option>Tahun berjalan</option>
        </select>
        <button type="button" class="btn-muted text-xs">Filter lanjutan</button>
      </div>

      <div class="overflow-hidden rounded-5xl border border-brand-200">
        <div class="hidden grid-cols-[120px,1.2fr,1fr,1fr,1fr] gap-4 bg-brand-50 px-6 py-3 text-xs uppercase tracking-[0.25em] text-brand-400 md:grid">
          <span>Tanggal</span>
          <span>No. Jurnal</span>
          <span>Deskripsi</span>
          <span>Debit</span>
          <span>Kredit</span>
        </div>

        <div class="divide-y divide-brand-100 bg-white text-sm text-brand-700">
          <div class="grid gap-4 px-6 py-4 md:grid-cols-[120px,1.2fr,1fr,1fr,1fr]">
            <span class="font-medium text-brand-900">02 Jan 2025</span>
            <span>JU-00015</span>
            <div>
              Penjualan layanan implementasi
              <p class="text-xs text-brand-500">Dari modul Penjualan</p>
            </div>
            <span class="font-semibold text-brand-600">Rp 6.200.000</span>
            <span class="font-semibold text-brand-600">Rp 6.200.000</span>
          </div>
          <div class="px-6 py-4 text-sm text-brand-500">
            ?? Data transaksi riil akan terhubung setelah modul inti akuntansi selesai. Saat ini hanya mock layout.
          </div>
        </div>
      </div>
    </section>
  </div>
</template>


