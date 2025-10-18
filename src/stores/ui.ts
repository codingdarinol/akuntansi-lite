import { defineStore } from 'pinia'

interface UiState {
  sidebarExpanded: boolean
}

export const useUiStore = defineStore('ui', {
  state: (): UiState => ({
    sidebarExpanded: true,
  }),
  actions: {
    toggleSidebar() {
      this.sidebarExpanded = !this.sidebarExpanded
    },
    setSidebar(expanded: boolean) {
      this.sidebarExpanded = expanded
    },
  },
})
