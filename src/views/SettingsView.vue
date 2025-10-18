<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Pengaturan Aplikasi</h1>
      <p class="text-sm text-brand-600">
        Sesuaikan identitas perusahaan, periode akuntansi, serta utilitas penting seperti backup dan restore database.
      </p>
    </header>

    <section class="grid gap-5 lg:grid-cols-2">
      <form class="surface-card space-y-5 p-6">
        <div>
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Profil Perusahaan</p>
          <h2 class="mt-2 text-lg font-semibold text-brand-900">Informasi dasar</h2>
        </div>
        <div class="grid gap-4 md:grid-cols-2">
          <label class="space-y-2 text-sm text-brand-600">
            Nama Perusahaan
            <input
              type="text"
              placeholder="Contoh: PT Maju Bersama"
              class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 placeholder:text-brand-400 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-400/40"
            />
          </label>
          <label class="space-y-2 text-sm text-brand-600">
            NPWP
            <input
              type="text"
              placeholder="00.000.000.0-000.000"
              class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 placeholder:text-brand-400 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-400/40"
            />
          </label>
        </div>
        <label class="space-y-2 text-sm text-brand-600">
          Alamat
          <textarea
            rows="4"
            placeholder="Alamat lengkap kantor"
            class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 placeholder:text-brand-400 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-400/40"
          ></textarea>
        </label>
        <div class="flex justify-end">
          <button type="button" class="btn-primary">Simpan perubahan</button>
        </div>
      </form>

      <section class="surface-card space-y-5 p-6">
        <div>
          <p class="text-xs uppercase tracking-[0.3em] text-brand-400">Utilitas Database</p>
          <h2 class="mt-2 text-lg font-semibold text-brand-900">Backup & Restore</h2>
        </div>
        <div class="space-y-4">
          <div class="glass-panel bg-white p-5 text-brand-900">
            <h3 class="text-sm font-semibold">Backup Database</h3>
            <p class="mt-1 text-xs text-brand-600">Buat salinan database lokal ke lokasi yang kamu pilih.</p>
            <button type="button" class="mt-4 btn-muted">Buat backup sekarang</button>
          </div>
          <div class="glass-panel bg-white p-5 text-brand-900">
            <h3 class="text-sm font-semibold">Restore Database</h3>
            <p class="mt-1 text-xs text-brand-600">Pulihkan dari file .sqlite atau arsip zip backup.</p>
            <button type="button" class="mt-4 btn-muted">Pilih file backup</button>
          </div>
        </div>
      </section>
    </section>
  </div>
</template>



