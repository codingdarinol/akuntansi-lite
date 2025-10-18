<script setup lang="ts">
import type { Component } from 'vue'
import { computed } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import {
  LayoutDashboard,
  ListTree,
  Notebook,
  PieChart,
  Settings,
} from 'lucide-vue-next'

import routes from '@/router/routes'

const props = defineProps<{
  expanded: boolean
}>()

const isExpanded = computed(() => props.expanded)

const iconMap: Record<string, Component> = {
  LayoutDashboard,
  ListTree,
  Notebook,
  PieChart,
  Settings,
}

const route = useRoute()

const navigation = computed(() =>
  routes.filter((item) => item.meta?.hiddenInMenu !== true && item.name !== 'not-found'),
)

const currentRouteName = computed(() => route.name?.toString())
</script>

<template>
  <aside
    :class="[
      'flex min-h-screen flex-col border-r border-slate-200 bg-white transition-[width] duration-200 ease-in-out',
      isExpanded ? 'w-64' : 'w-20',
    ]"
  >
    <div class="flex h-16 items-center gap-2 px-4">
      <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-brand-500 text-lg font-semibold text-white">
        LB
      </div>
      <span v-if="isExpanded" class="text-lg font-semibold text-slate-900">LokalBuku</span>
    </div>

    <nav class="flex-1 space-y-1 px-2 py-4">
      <RouterLink
        v-for="item in navigation"
        :key="item.path"
        :to="item.path"
        class="group flex items-center gap-3 rounded-lg px-2 py-2 text-sm font-medium text-slate-600 transition hover:bg-brand-50 hover:text-brand-600"
        :class="currentRouteName === item.name ? 'bg-brand-50 text-brand-600 shadow-sm' : ''"
      >
        <component
          :is="iconMap[item.meta?.icon as keyof typeof iconMap] ?? LayoutDashboard"
          class="h-5 w-5 flex-shrink-0 text-slate-500 transition group-hover:text-brand-600"
        />
        <span v-if="isExpanded">{{ item.meta?.title ?? item.name }}</span>
      </RouterLink>
    </nav>

    <footer v-if="isExpanded" class="px-4 py-4 text-xs text-slate-400">
      <p>LokalBuku v0.1.0</p>
      <p>Data tersimpan di perangkatmu.</p>
    </footer>
  </aside>
</template>
