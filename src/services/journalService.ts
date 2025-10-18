import { invoke } from '@tauri-apps/api/tauri'

import type { CreateJournalPayload, JournalSummary } from '@/types/journal'

export interface JournalFilterParams {
  limit?: number
  offset?: number
  start_date?: string
  end_date?: string
  search?: string
}

export async function fetchJournals(params: JournalFilterParams = {}): Promise<JournalSummary[]> {
  const result = await invoke<JournalSummary[]>('list_journals', { params })
  return result
}

export async function createJournal(payload: CreateJournalPayload): Promise<JournalSummary> {
  const result = await invoke<JournalSummary>('create_journal_entry', { payload })
  return result
}
