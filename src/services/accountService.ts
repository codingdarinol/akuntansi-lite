import { invoke } from '@tauri-apps/api/tauri'

import type { Account, CreateAccountPayload } from '@/types/account'

export async function fetchAccounts(): Promise<Account[]> {
  const result = await invoke<Account[]>('list_accounts')
  return result
}

export async function createAccount(payload: CreateAccountPayload): Promise<Account> {
  const result = await invoke<Account>('create_account', { payload })
  return result
}
