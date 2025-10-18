<template>
  <div class="flex h-full flex-col items-center justify-center rounded-xl border border-dashed border-slate-300 bg-white/70 p-12 text-center shadow-inner">
    <h1 class="text-3xl font-semibold text-slate-900">Halaman tidak ditemukan</h1>
    <p class="mt-3 max-w-md text-sm text-slate-500">
      Rute yang kamu tuju belum tersedia. Gunakan menu di samping kiri untuk memilih modul yang
      siap digunakan.
    </p>
    <RouterLink
      to="/"
      class="mt-6 inline-flex items-center justify-center rounded-lg bg-brand-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-brand-700"
    >
      Kembali ke dashboard
    </RouterLink>
  </div>
</template>
