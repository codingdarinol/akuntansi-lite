import { defineStore } from 'pinia'

import { fetchBalanceSheet } from '@/services/reportService'
import type { BalanceSheetParams, BalanceSheetReport } from '@/types/reports'

interface BalanceSheetState {
  report: BalanceSheetReport | null
  loading: boolean
  error: string | null
  lastParams: BalanceSheetParams | null
}

export const useBalanceSheetStore = defineStore('balanceSheet', {
  state: (): BalanceSheetState => ({
    report: null,
    loading: false,
    error: null,
    lastParams: null,
  }),
  actions: {
    async load(params: BalanceSheetParams = {}, force = false) {
      if (this.loading) return

      const hasReport = !!this.report
      const requestIsDefault = Object.keys(params).length === 0
      if (!force && hasReport && requestIsDefault) {
        return
      }

      this.loading = true
      this.error = null

      try {
        const data = await fetchBalanceSheet(params)
        this.report = data
        this.lastParams = { ...params }
      } catch (error) {
        this.error = (error as Error).message ?? 'Gagal memuat laporan neraca.'
        throw error
      } finally {
        this.loading = false
      }
    },
  },
})
