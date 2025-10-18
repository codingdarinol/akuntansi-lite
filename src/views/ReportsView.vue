<template>
  <div class="space-y-6">
    <header>
      <h1 class="text-2xl font-semibold text-slate-900">Laporan Keuangan</h1>
      <p class="mt-1 text-sm text-slate-500">
        Pilih laporan untuk melihat performa bisnis secara menyeluruh.
      </p>
    </header>

    <section class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
      <article
        v-for="report in [
          { title: 'Neraca', description: 'Posisi keuangan per tanggal tertentu.' },
          { title: 'Laba Rugi', description: 'Ringkasan pendapatan dan beban per periode.' },
          { title: 'Arus Kas', description: 'Pergerakan kas berdasarkan aktivitas operasi, investasi, dan pendanaan.' },
          { title: 'Perubahan Ekuitas', description: 'Perubahan saldo modal pemilik sepanjang periode.' },
          { title: 'Analisis Rasio', description: 'Rasio penting seperti current ratio, ROE, dan lainnya.' },
          { title: 'Analisis Beban', description: 'Visualisasi struktur biaya untuk mengendalikan pengeluaran.' },
        ]"
        :key="report.title"
        class="flex flex-col rounded-xl border border-slate-200 bg-white p-5 shadow-sm transition hover:border-brand-200 hover:shadow-md"
      >
        <h2 class="text-lg font-semibold text-slate-900">{{ report.title }}</h2>
        <p class="mt-2 text-sm text-slate-500">{{ report.description }}</p>
        <button
          type="button"
          class="mt-auto inline-flex w-fit items-center justify-center rounded-lg bg-brand-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-brand-700"
        >
          Lihat Laporan
        </button>
      </article>
    </section>
  </div>
</template>
