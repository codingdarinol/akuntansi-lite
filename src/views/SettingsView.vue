<template>
  <div class="space-y-6">
    <header>
      <h1 class="text-2xl font-semibold text-slate-900">Pengaturan Aplikasi</h1>
      <p class="mt-1 text-sm text-slate-500">
        Kelola identitas perusahaan, format penomoran, dan preferensi lain.
      </p>
    </header>

    <section class="grid gap-4 lg:grid-cols-2">
      <form class="space-y-4 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
        <div>
          <h2 class="text-lg font-semibold text-slate-900">Profil Perusahaan</h2>
          <p class="text-sm text-slate-500">Informasi ini muncul di laporan dan invoice.</p>
        </div>
        <div class="grid gap-4 md:grid-cols-2">
          <label class="space-y-1 text-sm font-medium text-slate-700">
            Nama Perusahaan
            <input
              type="text"
              placeholder="Contoh: PT Maju Bersama"
              class="w-full rounded-lg border border-slate-200 px-3 py-2 text-sm text-slate-700 shadow-sm focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-200 focus:ring-offset-1"
            />
          </label>
          <label class="space-y-1 text-sm font-medium text-slate-700">
            NPWP
            <input
              type="text"
              placeholder="00.000.000.0-000.000"
              class="w-full rounded-lg border border-slate-200 px-3 py-2 text-sm text-slate-700 shadow-sm focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-200 focus:ring-offset-1"
            />
          </label>
        </div>
        <label class="space-y-1 text-sm font-medium text-slate-700">
          Alamat
          <textarea
            rows="3"
            placeholder="Alamat lengkap kantor"
            class="w-full rounded-lg border border-slate-200 px-3 py-2 text-sm text-slate-700 shadow-sm focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-200 focus:ring-offset-1"
          ></textarea>
        </label>
        <div class="flex justify-end">
          <button
            type="button"
            class="inline-flex items-center justify-center rounded-lg bg-brand-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-brand-700"
          >
            Simpan Perubahan
          </button>
        </div>
      </form>

      <section class="space-y-4 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
        <div>
          <h2 class="text-lg font-semibold text-slate-900">Utilitas</h2>
          <p class="text-sm text-slate-500">
            Fitur penting untuk menjaga data keuangan tetap aman.
          </p>
        </div>
        <div class="space-y-3">
          <div class="rounded-lg border border-slate-200 p-4">
            <h3 class="text-sm font-semibold text-slate-900">Backup Database</h3>
            <p class="mt-1 text-xs text-slate-500">
              Buat salinan database lokal Anda untuk disimpan ke media lain.
            </p>
            <button
              type="button"
              class="mt-3 inline-flex items-center justify-center rounded-lg bg-slate-900 px-3 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-slate-700"
            >
              Buat Backup
            </button>
          </div>
          <div class="rounded-lg border border-slate-200 p-4">
            <h3 class="text-sm font-semibold text-slate-900">Restore Database</h3>
            <p class="mt-1 text-xs text-slate-500">
              Pulihkan data dari file backup (.sqlite atau .zip).
            </p>
            <button
              type="button"
              class="mt-3 inline-flex items-center justify-center rounded-lg border border-slate-200 px-3 py-2 text-sm font-medium text-slate-600 shadow-sm transition hover:border-brand-200 hover:text-brand-600"
            >
              Pilih File Backup
            </button>
          </div>
        </div>
      </section>
    </section>
  </div>
</template>
