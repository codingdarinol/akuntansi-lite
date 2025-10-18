import { invoke } from '@tauri-apps/api/core'
import Database from '@tauri-apps/plugin-sql'

type SqlDatabase = Awaited<ReturnType<typeof Database.load>>

interface DatabaseInfo {
  url: string
  description: string
  path: string
}

let connection: SqlDatabase | null = null
let metadata: DatabaseInfo | null = null

export async function getDatabase(): Promise<SqlDatabase> {
  if (!connection) {
    metadata = await invoke<DatabaseInfo>('get_database_info')
    connection = await Database.load(metadata.url)
  }
  return connection
}

export function getDatabaseMetadata(): DatabaseInfo | null {
  return metadata
}

export async function closeDatabase(): Promise<void> {
  if (connection) {
    await connection.close(metadata?.url)
    connection = null
  }
}
