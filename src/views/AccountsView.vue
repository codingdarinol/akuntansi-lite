<script setup lang="ts">
const accountRows = [
  {
    code: '1001',
    name: 'Kas Besar',
    category: 'Aktiva Lancar',
    balance: 'Rp 5.200.000',
    status: 'Aktif',
  },
  {
    code: '1102',
    name: 'Piutang Usaha',
    category: 'Aktiva Lancar',
    balance: 'Rp 3.150.000',
    status: 'Aktif',
  },
  {
    code: '4101',
    name: 'Pendapatan Jasa',
    category: 'Pendapatan',
    balance: 'Rp 7.850.000',
    status: 'Aktif',
  },
]
</script>

<template>
  <div class="space-y-7">
    <header class="flex flex-wrap items-center justify-between gap-4">
      <div class="space-y-2">
        <h1 class="text-3xl font-semibold text-brand-900">Bagan Akun</h1>
        <p class="max-w-xl text-sm text-brand-600">
          Kelola struktur akun perusahaan dengan mudah. Setiap perubahan langsung tersimpan ke database lokal.
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <button type="button" class="btn-muted">
          Ekspor CSV
        </button>
        <button type="button" class="btn-primary">
          + Tambah Akun
        </button>
      </div>
    </header>

    <section class="surface-card space-y-6 p-6">
      <div class="flex flex-wrap items-center gap-3">
        <input
          type="search"
          placeholder="Cari akun atau kategori..."
          class="w-full rounded-full border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-800 placeholder:text-brand-300 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60 sm:w-72"
        />
        <select
          class="rounded-full border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-800 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
        >
          <option>Semua kategori</option>
          <option>Aktiva</option>
          <option>Kewajiban</option>
          <option>Modal</option>
          <option>Pendapatan</option>
          <option>Beban</option>
        </select>
        <button type="button" class="btn-muted text-xs">Filter lanjutan</button>
      </div>

      <div class="overflow-hidden rounded-5xl border border-brand-200">
        <table class="min-w-full divide-y divide-brand-100 text-sm text-brand-700">
          <thead class="bg-brand-50 text-xs uppercase tracking-[0.25em] text-brand-400">
            <tr>
              <th class="px-6 py-4 text-left font-medium">Kode</th>
              <th class="px-6 py-4 text-left font-medium">Nama Akun</th>
              <th class="px-6 py-4 text-left font-medium">Kategori</th>
              <th class="px-6 py-4 text-right font-medium">Saldo</th>
              <th class="px-6 py-4 text-left font-medium">Status</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-brand-100 bg-white">
            <tr
              v-for="row in accountRows"
              :key="row.code"
              class="transition hover:bg-brand-50"
            >
              <td class="px-6 py-4 font-semibold text-brand-900">{{ row.code }}</td>
              <td class="px-6 py-4 text-brand-700">{{ row.name }}</td>
              <td class="px-6 py-4 text-brand-600">{{ row.category }}</td>
              <td class="px-6 py-4 text-right font-semibold text-brand-600">{{ row.balance }}</td>
              <td class="px-6 py-4">
                <span class="rounded-full bg-accent-100 px-3 py-1 text-xs font-medium text-brand-700">
                  {{ row.status }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <p class="text-xs text-brand-500">
        Catatan: Struktur hierarki akun akan dikembangkan pada sprint modul Akun. Saat ini tabel ini hanya mock UI untuk validasi gaya visual.
      </p>
    </section>
  </div>
</template>


