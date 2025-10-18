import { defineStore } from 'pinia'

import { createJournal, fetchJournals, type JournalFilterParams } from '@/services/journalService'
import type { CreateJournalPayload, JournalSummary } from '@/types/journal'

interface JournalsState {
  items: JournalSummary[]
  loading: boolean
  error: string | null
}

export const useJournalsStore = defineStore('journals', {
  state: (): JournalsState => ({
    items: [],
    loading: false,
    error: null,
  }),
  getters: {
    journals: (state) => state.items,
  },
  actions: {
    async load(params: JournalFilterParams = {}) {
      if (this.loading) return
      this.loading = true
      this.error = null

      try {
        const result = await fetchJournals(params)
        this.items = result
      } catch (error) {
        this.error = (error as Error).message ?? 'Gagal memuat jurnal.'
        throw error
      } finally {
        this.loading = false
      }
    },
    async create(payload: CreateJournalPayload) {
      this.error = null
      try {
        const created = await createJournal(payload)
        this.items = [created, ...this.items].sort(
          (a, b) => new Date(b.journal_date).getTime() - new Date(a.journal_date).getTime(),
        )
        return created
      } catch (error) {
        this.error = (error as Error).message ?? 'Gagal menyimpan jurnal.'
        throw error
      }
    },
  },
})
