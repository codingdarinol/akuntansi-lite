<script setup lang="ts">
import { computed } from 'vue'

import { useUiStore } from '@/stores/ui'

import AppHeader from './AppHeader.vue'
import AppSidebar from './AppSidebar.vue'

const uiStore = useUiStore()

const isSidebarExpanded = computed(() => uiStore.sidebarExpanded)
</script>

<template>
  <div class="flex min-h-screen overflow-hidden bg-brand-50">
    <AppSidebar :expanded="isSidebarExpanded" />
    <div class="flex flex-1 flex-col overflow-hidden">
      <AppHeader />
      <main class="flex-1 overflow-y-auto px-5 pb-8 pt-4 sm:px-8 md:px-12 md:pt-6">
        <div class="mx-auto flex w-full max-w-[1400px] flex-col gap-6 pb-10">
          <slot />
        </div>
      </main>
    </div>
  </div>
</template>
