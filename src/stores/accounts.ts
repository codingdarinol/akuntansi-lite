import { defineStore } from 'pinia'

import { createAccount, fetchAccounts } from '@/services/accountService'
import type { Account, CreateAccountPayload } from '@/types/account'

interface AccountsState {
  items: Account[]
  loading: boolean
  error: string | null
}

export const useAccountsStore = defineStore('accounts', {
  state: (): AccountsState => ({
    items: [],
    loading: false,
    error: null,
  }),
  getters: {
    accounts: (state) => state.items,
    activeAccounts: (state) => state.items.filter((account) => account.is_active),
  },
  actions: {
    async load(force = false) {
      if (this.loading) return
      if (!force && this.items.length > 0) return

      this.loading = true
      this.error = null
      try {
        const result = await fetchAccounts()
        this.items = [...result].sort((a, b) => a.code.localeCompare(b.code))
      } catch (error) {
        this.error = (error as Error).message ?? 'Gagal memuat data akun.'
        throw error
      } finally {
        this.loading = false
      }
    },
    async create(payload: CreateAccountPayload) {
      this.error = null
      try {
        const account = await createAccount(payload)
        this.items = [...this.items, account].sort((a, b) => a.code.localeCompare(b.code))
        return account
      } catch (error) {
        this.error = (error as Error).message ?? 'Gagal menambahkan akun.'
        throw error
      }
    },
  },
})
