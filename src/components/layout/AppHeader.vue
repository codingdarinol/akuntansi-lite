<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { Menu, Sun } from 'lucide-vue-next'

import { useUiStore } from '@/stores/ui'

const uiStore = useUiStore()
const route = useRoute()

const pageTitle = computed(() => (typeof route.meta?.title === 'string' ? route.meta.title : ''))

const toggleSidebar = () => uiStore.toggleSidebar()
</script>

<template>
  <header
    class="flex h-16 items-center justify-between border-b border-slate-200 bg-white px-6 shadow-sm"
  >
    <div class="flex items-center gap-3">
      <button
        type="button"
        class="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-slate-200 bg-white text-slate-600 shadow-sm transition hover:bg-slate-50"
        aria-label="Toggle sidebar"
        @click="toggleSidebar"
      >
        <Menu class="h-5 w-5" />
      </button>
      <div>
        <h1 class="text-lg font-semibold text-slate-900">
          {{ pageTitle || 'LokalBuku' }}
        </h1>
        <p class="text-xs text-slate-500">Ringkasan keuangan usaha dalam genggamanmu.</p>
      </div>
    </div>

    <div class="flex items-center gap-3">
      <button
        type="button"
        class="hidden items-center gap-2 rounded-full border border-slate-200 bg-white px-3 py-2 text-sm text-slate-600 shadow-sm transition hover:bg-slate-50 md:flex"
      >
        <Sun class="h-4 w-4" />
        Mode Terang
      </button>
      <div class="flex items-center gap-3 rounded-full border border-slate-200 bg-white px-3 py-1 shadow-sm">
        <span class="h-8 w-8 rounded-full bg-brand-600 text-sm font-semibold leading-8 text-white">
          RU
        </span>
        <div class="text-xs">
          <p class="font-semibold text-slate-900">Raica Utama</p>
          <p class="text-slate-500">Administrator</p>
        </div>
      </div>
    </div>
  </header>
</template>
