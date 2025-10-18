<script setup lang="ts">
interface DataCard {
  title: string
  description: string
  badge: string
}

const dataCards: DataCard[] = [
  { title: 'Informasi Umum', description: 'Profil perusahaan, preferensi format, dan tahun buku.', badge: 'Prioritas awal' },
  { title: 'Data Kas', description: 'Daftar kas kecil dan posisi saldo awal.', badge: 'Draft' },
  { title: 'Data Bank', description: 'Rekening bank dan nomor akun untuk rekonsiliasi.', badge: 'Draft' },
  { title: 'Data Piutang', description: 'Saldo piutang dan termin penagihan pelanggan.', badge: 'MVP' },
  { title: 'Data Utang', description: 'Saldo utang dan jadwal pembayaran pemasok.', badge: 'MVP' },
  { title: 'Data Aset Tetap', description: 'Lampiran aset tetap beserta metode penyusutan.', badge: 'Sprint 4' },
  { title: 'Data Inventori', description: 'Daftar stok barang dan satuan cost.', badge: 'Sprint 4' },
  { title: 'Data Pelanggan', description: 'Kontak pelanggan yang digunakan modul penjualan.', badge: 'MVP' },
  { title: 'Data Pemasok', description: 'Kontak pemasok untuk modul pembelian.', badge: 'MVP' },
]
</script>

<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Master Data</h1>
      <p class="text-sm text-brand-600">
        Kelola daftar pelanggan, pemasok, dan data referensi lain sebagai fondasi modul transaksi.
      </p>
    </header>

    <section class="surface-card space-y-5 p-6">
      <div class="glass-panel bg-brand-50 p-5 text-brand-800">
        <p class="text-sm font-semibold text-brand-900">Blueprint modul master data</p>
        <p class="mt-1 text-xs text-brand-600">
          Setiap kartu berikut mewakili tabel SQLite yang akan dibangun bertahap. UI ini masih placeholder.
        </p>
      </div>
      <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
        <div
          v-for="card in dataCards"
          :key="card.title"
          class="glass-panel bg-brand-50 p-4 text-brand-700"
        >
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">{{ card.badge }}</p>
          <p class="mt-2 text-lg font-semibold text-brand-900">{{ card.title }}</p>
          <p class="text-xs text-brand-600">{{ card.description }}</p>
        </div>
      </div>
    </section>
  </div>
</template>
