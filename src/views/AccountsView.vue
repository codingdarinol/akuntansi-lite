<script setup lang="ts">
const sampleAccount = {
  code: '1001',
  name: 'Kas Besar',
  category: 'Aktiva Lancar',
  balance: 'Rp 5.200.000',
}
</script>

<template>
  <div class="space-y-6">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="text-2xl font-semibold text-slate-900">Bagan Akun</h1>
        <p class="text-sm text-slate-500">
          Atur struktur akun dan pastikan pencatatan tetap konsisten.
        </p>
      </div>
      <button
        type="button"
        class="inline-flex items-center justify-center rounded-lg bg-brand-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-brand-700"
      >
        + Tambah Akun
      </button>
    </header>

    <section class="rounded-xl border border-slate-200 bg-white shadow-sm">
      <div class="border-b border-slate-200 px-6 py-4">
        <div class="flex flex-wrap items-center gap-3">
          <input
            type="search"
            placeholder="Cari akun atau kategori..."
            class="w-full rounded-lg border border-slate-200 px-4 py-2 text-sm text-slate-700 shadow-sm focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-200 focus:ring-offset-1 sm:w-72"
          />
          <select
            class="rounded-lg border border-slate-200 px-3 py-2 text-sm text-slate-700 shadow-sm focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-200 focus:ring-offset-1"
          >
            <option>Semua Kategori</option>
            <option>Aktiva</option>
            <option>Kewajiban</option>
            <option>Modal</option>
            <option>Pendapatan</option>
            <option>Beban</option>
          </select>
        </div>
      </div>

      <div class="divide-y divide-slate-100">
        <div
          class="grid grid-cols-5 gap-4 px-6 py-4 text-xs font-semibold uppercase tracking-wider text-slate-500"
        >
          <span>Kode</span>
          <span class="col-span-2">Nama Akun</span>
          <span>Kategori</span>
          <span>Saldo</span>
        </div>

        <div class="divide-y divide-slate-100 text-sm text-slate-700">
          <div class="grid grid-cols-5 gap-4 px-6 py-4">
            <span class="font-medium text-slate-900">{{ sampleAccount.code }}</span>
            <span class="col-span-2">{{ sampleAccount.name }}</span>
            <span>{{ sampleAccount.category }}</span>
            <span class="text-right font-medium text-green-600">{{ sampleAccount.balance }}</span>
          </div>
          <div class="px-6 py-4 text-sm text-slate-500">
            📌 Struktur detail akun akan dikelola di sprint selanjutnya. Ini hanya mock UI sebagai
            referensi.
          </div>
        </div>
      </div>
    </section>
  </div>
</template>
