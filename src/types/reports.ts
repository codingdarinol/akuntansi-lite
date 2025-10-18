import type { AccountType, NormalBalance } from './account'

export interface ReportAccountBalance {
  account_id: number
  parent_id: number | null
  code: string
  name: string
  account_type: AccountType
  normal_balance: NormalBalance
  balance: number
}

export type BalanceSheetSectionKey = 'ASSET' | 'LIABILITY' | 'EQUITY'

export interface BalanceSheetSection {
  key: BalanceSheetSectionKey
  label: string
  total: number
  accounts: ReportAccountBalance[]
}

export interface BalanceSheetReport {
  as_of: string
  currency: string
  sections: BalanceSheetSection[]
  total_liabilities_and_equity: number
  generated_at: string
}

export interface BalanceSheetParams {
  as_of?: string
}

export type IncomeStatementSectionKey = 'REVENUE' | 'EXPENSE'

export interface IncomeStatementSection {
  key: IncomeStatementSectionKey
  label: string
  total: number
  accounts: ReportAccountBalance[]
}

export interface IncomeStatementTotals {
  total_revenue: number
  total_expenses: number
  net_income: number
}

export interface IncomeStatementReport {
  start_date: string
  end_date: string
  currency: string
  sections: IncomeStatementSection[]
  totals: IncomeStatementTotals
  generated_at: string
}

export interface IncomeStatementParams {
  start_date?: string
  end_date?: string
}
