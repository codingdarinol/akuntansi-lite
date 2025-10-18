<script setup lang="ts">
import { computed } from 'vue'

import { useUiStore } from '@/stores/ui'

import AppHeader from './AppHeader.vue'
import AppSidebar from './AppSidebar.vue'

const uiStore = useUiStore()

const isSidebarExpanded = computed(() => uiStore.sidebarExpanded)
</script>

<template>
  <div class="flex min-h-screen bg-slate-50">
    <AppSidebar :expanded="isSidebarExpanded" />
    <div class="flex flex-1 flex-col overflow-hidden">
      <AppHeader />
      <main class="flex-1 overflow-y-auto bg-slate-50 p-6">
        <slot />
      </main>
    </div>
  </div>
</template>
