<script setup lang="ts">
import type { Component } from 'vue'
import { computed } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import {
  LayoutDashboard,
  ShoppingBag,
  Package,
  FolderKanban,
  ListTree,
  Notebook,
  BookOpen,
  TrendingUp,
  PieChart,
  Table,
  Receipt,
  Settings,
  LifeBuoy,
  Database,
  Keyboard,
  Sparkles,
  ArrowRightLeft,
  ChevronRight,
} from 'lucide-vue-next'

const props = defineProps<{
  expanded: boolean
}>()

const navGroups = [
  {
    title: 'Navigasi Utama',
    items: [
      { label: 'Dashboard', icon: 'LayoutDashboard', to: { name: 'dashboard' } },
      { label: 'Penjualan', icon: 'ShoppingBag', to: { name: 'sales' }, badge: 'new' },
      { label: 'Pembelian', icon: 'Package', to: { name: 'purchases' } },
      { label: 'Master Data', icon: 'FolderKanban', to: { name: 'common' } },
    ],
  },
  {
    title: 'Akuntansi',
    items: [
      { label: 'Bagan Akun', icon: 'ListTree', to: { name: 'accounts' } },
      { label: 'Jurnal Umum', icon: 'Notebook', to: { name: 'transactions' } },
      { label: 'Buku Besar', icon: 'BookOpen', to: { name: 'general-ledger' } },
    ],
  },
  {
    title: 'Pelaporan',
    items: [
      { label: 'Laba Rugi', icon: 'TrendingUp', to: { name: 'profit-loss' } },
      { label: 'Neraca', icon: 'PieChart', to: { name: 'balance-sheet' } },
      { label: 'Trial Balance', icon: 'Table', to: { name: 'trial-balance' } },
      { label: 'GST', icon: 'Receipt', to: { name: 'gst' } },
    ],
  },
  {
    title: 'Utilitas',
    items: [{ label: 'Pengaturan', icon: 'Settings', to: { name: 'settings' } }],
  },
]

const quickLinks = [
  { label: 'Pusat Bantuan', description: 'Butuh panduan cepat?', icon: 'LifeBuoy' },
  { label: 'Shortcut Keyboard', description: 'Lihat daftar pintasan', icon: 'Keyboard' },
  { label: 'Ganti Database', description: 'Kelola file data lokal', icon: 'Database' },
]

const iconMap: Record<string, Component> = {
  LayoutDashboard,
  ShoppingBag,
  Package,
  FolderKanban,
  ListTree,
  Notebook,
  BookOpen,
  TrendingUp,
  PieChart,
  Table,
  Receipt,
  Settings,
  LifeBuoy,
  Database,
  Keyboard,
  Sparkles,
  ArrowRightLeft,
  ChevronRight,
}

const route = useRoute()
const currentRouteName = computed(() => route.name?.toString())
const isExpanded = computed(() => props.expanded)
const todayLabel = new Date().toLocaleDateString('id-ID')
</script>

<template>
  <aside
    :class="[
      'flex min-h-screen flex-col border-r border-brand-200 bg-brand-900 text-brand-50 transition-all duration-300 ease-in-out',
      isExpanded ? 'w-[260px]' : 'w-24',
    ]"
  >
    <div class="flex items-center gap-3 px-5 pb-4 pt-8">
      <div
        class="flex h-12 w-12 items-center justify-center rounded-3xl bg-brand-700 text-xl font-semibold text-white shadow-card"
      >
        LB
      </div>
      <div v-if="isExpanded">
        <p class="text-lg font-semibold text-white">LokalBuku</p>
        <p class="text-xs uppercase tracking-[0.3em] text-brand-200/80">Akuntansi Harian</p>
      </div>
    </div>

    <div class="px-5" v-if="isExpanded">
      <div class="glass-panel flex items-center gap-3 px-4 py-3">
        <Sparkles class="h-5 w-5 text-brand-500" />
        <div>
          <p class="text-sm font-medium text-brand-900">Status Database</p>
          <p class="text-xs text-brand-700">Terhubung - {{ todayLabel }}</p>
        </div>
      </div>
    </div>

    <nav class="mt-6 flex-1 space-y-6 px-3 pb-6">
      <section
        v-for="group in navGroups"
        :key="group.title"
        class="space-y-2"
      >
        <p v-if="isExpanded" class="px-2 text-[0.68rem] uppercase tracking-[0.4em] text-brand-200/80">
          {{ group.title }}
        </p>
        <div class="space-y-2">
          <RouterLink
            v-for="item in group.items"
            :key="item.label"
            :to="item.to"
            class="group relative flex items-center gap-3 rounded-3xl px-3 py-2.5 text-sm font-medium text-brand-100 transition-all duration-200 hover:bg-brand-700 hover:text-white"
            :class="currentRouteName === (item.to.name as string) ? 'bg-brand-700 text-white shadow-card' : ''"
          >
            <component
              :is="iconMap[item.icon]"
              class="h-5 w-5 flex-shrink-0 text-brand-200 transition group-hover:text-white"
            />
            <div v-if="isExpanded" class="flex w-full items-center justify-between gap-2">
              <span>{{ item.label }}</span>
              <span
                v-if="item.badge"
                class="ml-auto rounded-full bg-accent-100 px-2 py-0.5 text-[10px] uppercase tracking-wide text-brand-700"
              >
                {{ item.badge }}
              </span>
            </div>
          </RouterLink>
        </div>
      </section>
    </nav>

    <div class="mt-auto space-y-4 px-5 pb-8">
      <div class="rounded-3xl border border-brand-200 bg-white px-4 py-3 text-brand-800 shadow-card">
        <div>
          <p class="text-xs uppercase tracking-[0.4em] text-brand-400">Periode Buku</p>
          <p class="text-sm font-semibold text-brand-900">FY 2025 - Semester 1</p>
        </div>
        <ArrowRightLeft class="h-5 w-5 text-brand-500" />
      </div>

      <div
        v-for="link in quickLinks"
        :key="link.label"
        class="flex items-center gap-3 rounded-3xl px-4 py-3 transition hover:bg-brand-800/80"
      >
        <component :is="iconMap[link.icon]" class="h-5 w-5 text-brand-200" />
        <div v-if="isExpanded">
          <p class="text-sm font-medium text-brand-50">{{ link.label }}</p>
          <p class="text-xs text-brand-200/80">{{ link.description }}</p>
        </div>
      </div>

      <div class="flex items-center justify-between rounded-3xl border border-brand-700 px-4 py-3 text-xs text-brand-200">
        <span class="font-medium">LokalBuku v0.1.0</span>
        <span class="hidden items-center gap-1 sm:flex">
          <ChevronRight class="h-4 w-4" />
          Stable
        </span>
      </div>
    </div>
  </aside>
</template>





