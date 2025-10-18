import { defineStore } from 'pinia'

import { fetchIncomeStatement } from '@/services/reportService'
import type { IncomeStatementParams, IncomeStatementReport } from '@/types/reports'

interface IncomeStatementState {
  report: IncomeStatementReport | null
  loading: boolean
  error: string | null
  lastParams: IncomeStatementParams | null
}

export const useIncomeStatementStore = defineStore('incomeStatement', {
  state: (): IncomeStatementState => ({
    report: null,
    loading: false,
    error: null,
    lastParams: null,
  }),
  actions: {
    async load(params: IncomeStatementParams = {}, force = false) {
      if (this.loading) return

      const hasReport = !!this.report
      const requestIsDefault = Object.keys(params).length === 0
      if (!force && hasReport && requestIsDefault) {
        return
      }

      this.loading = true
      this.error = null

      try {
        const data = await fetchIncomeStatement(params)
        this.report = data
        this.lastParams = { ...params }
      } catch (error) {
        this.error = (error as Error).message ?? 'Gagal memuat laporan laba rugi.'
        throw error
      } finally {
        this.loading = false
      }
    },
  },
})
