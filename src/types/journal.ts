import type { Account } from './account'

export interface JournalLineInput {
  account_id: number | null
  description?: string | null
  debit: number
  credit: number
  contact_id?: number | null
}

export interface CreateJournalPayload {
  journal_number?: string | null
  journal_date: string
  memo?: string | null
  source?: string | null
  lines: JournalLineInput[]
}

export interface JournalSummary {
  id: number
  journal_number?: string | null
  journal_date: string
  memo?: string | null
  source?: string | null
  total_debit: number
  total_credit: number
}

export type AccountLookup = Pick<Account, 'id' | 'code' | 'name' | 'account_type'>

