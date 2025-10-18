<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { Menu, Search, Plus, Bell, CalendarClock, Sparkles } from 'lucide-vue-next'

import { useUiStore } from '@/stores/ui'

const uiStore = useUiStore()
const route = useRoute()

const pageTitle = computed(() => (typeof route.meta?.title === 'string' ? route.meta.title : 'LokalBuku'))
const subtitle = computed(() => route.meta?.description ?? 'Ringkas kondisi keuanganmu dalam satu panel intuitif.')

const summaryTags = [
  { label: 'Kas & Bank', value: 'Rp12,5 jt' },
  { label: 'Invoice Aktif', value: '5 klien' },
  { label: 'Tahun Buku', value: '2025' },
]

const toggleSidebar = () => uiStore.toggleSidebar()
</script>

<template>
  <header class="relative px-5 pt-6 sm:px-8 md:px-12">
    <div class="surface-card overflow-hidden px-5 py-5 sm:px-7 sm:py-6">
      <div class="flex flex-col gap-5 md:flex-row md:items-center md:justify-between">
        <div class="flex flex-col gap-4">
          <div class="flex items-center gap-3">
            <button
              type="button"
              class="inline-flex h-11 w-11 items-center justify-center rounded-full border border-brand-200 bg-white text-brand-600 transition hover:bg-brand-50"
              aria-label="Toggle sidebar"
              @click="toggleSidebar"
            >
              <Menu class="h-5 w-5" />
            </button>
            <div class="flex flex-col gap-1">
              <div class="flex items-center gap-2">
                <Sparkles class="h-4 w-4 text-accent-400" />
                <span class="pill">Live Snapshot</span>
              </div>
              <h1 class="text-2xl font-semibold text-brand-900 md:text-3xl">
                {{ pageTitle }}
              </h1>
              <p class="text-sm text-brand-600">
                {{ subtitle }}
              </p>
            </div>
          </div>
          <div class="flex flex-wrap items-center gap-3 text-xs text-brand-500">
            <div
              v-for="tag in summaryTags"
              :key="tag.label"
              class="inline-flex items-center gap-2 rounded-full border border-brand-100 bg-brand-50 px-3 py-1"
            >
              <span class="uppercase tracking-[0.25em] text-brand-400">{{ tag.label }}</span>
              <span class="text-brand-700">{{ tag.value }}</span>
            </div>
          </div>
        </div>

        <div class="flex w-full flex-col items-stretch gap-3 sm:w-auto sm:flex-row sm:items-center">
          <div class="relative w-full sm:w-64">
            <Search class="absolute left-4 top-1/2 h-4 w-4 -translate-y-1/2 text-brand-300" />
            <input
              type="search"
              placeholder="Cari transaksi, pelanggan, atau akun..."
              class="w-full rounded-full border border-brand-200 bg-white py-2.5 pl-11 pr-4 text-sm text-brand-800 placeholder:text-brand-300 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
            />
          </div>
          <button type="button" class="btn-muted">
            <CalendarClock class="h-4 w-4" />
            Periode: Jan - Jun
          </button>
          <button type="button" class="btn-primary">
            <Plus class="h-4 w-4" />
            Transaksi Baru
          </button>
        </div>
      </div>

      <div class="mt-6 flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
        <div class="flex items-center gap-3 text-brand-600">
          <div class="flex h-11 w-11 items-center justify-center rounded-3xl bg-brand-100 text-sm font-semibold text-brand-700">
            RU
          </div>
          <div>
            <p class="text-sm font-semibold text-brand-900">Raica Utama</p>
            <p class="text-xs text-brand-500">Administrator - Offline Mode</p>
          </div>
        </div>
        <div class="flex items-center gap-3 text-xs text-brand-500">
          <span class="inline-flex items-center gap-2 rounded-full border border-brand-200 px-3 py-1">
            <Bell class="h-4 w-4" />
            3 aktivitas terbaru belum dibaca
          </span>
          <span class="inline-flex items-center gap-2 rounded-full border border-brand-200 px-3 py-1">
            <CalendarClock class="h-4 w-4" />
            Sinkronisasi terakhir: 2 menit lalu
          </span>
        </div>
      </div>
    </div>
  </header>
</template>

