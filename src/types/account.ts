export type AccountType = 'ASSET' | 'LIABILITY' | 'EQUITY' | 'REVENUE' | 'EXPENSE'
export type NormalBalance = 'DEBIT' | 'CREDIT'

export interface Account {
  id: number
  code: string
  name: string
  account_type: AccountType
  sub_type?: string | null
  normal_balance: NormalBalance
  parent_id?: number | null
  description?: string | null
  is_active: boolean
}

export interface CreateAccountPayload {
  code: string
  name: string
  account_type: AccountType
  sub_type?: string | null
  normal_balance: NormalBalance
  parent_id?: number | null
  description?: string | null
  is_active?: boolean
}

