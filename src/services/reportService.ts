import { invoke } from '@tauri-apps/api/core'

import type {
  BalanceSheetParams,
  BalanceSheetReport,
  IncomeStatementParams,
  IncomeStatementReport,
} from '@/types/reports'

function buildArgs<TParams extends object>(params: TParams | undefined) {
  if (!params) {
    return {}
  }

  const hasValues = Object.values(params as Record<string, unknown>).some(
    (value) => value !== undefined && value !== null && value !== '',
  )
  return hasValues ? { params } : {}
}

export async function fetchBalanceSheet(params: BalanceSheetParams = {}): Promise<BalanceSheetReport> {
  const args = buildArgs(params)
  return invoke<BalanceSheetReport>('generate_balance_sheet', args)
}

export async function fetchIncomeStatement(
  params: IncomeStatementParams = {},
): Promise<IncomeStatementReport> {
  const args = buildArgs(params)
  return invoke<IncomeStatementReport>('generate_income_statement', args)
}
