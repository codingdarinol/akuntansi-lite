<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'

import { useAccountsStore } from '@/stores/accounts'
import type { CreateAccountPayload } from '@/types/account'

const accountsStore = useAccountsStore()
const { accounts, loading, error } = storeToRefs(accountsStore)

const showCreate = ref(false)
const submitting = ref(false)
const feedback = ref<string | null>(null)

const form = reactive<CreateAccountPayload>({
  code: '',
  name: '',
  account_type: 'ASSET',
  sub_type: '',
  normal_balance: 'DEBIT',
  parent_id: null,
  description: '',
  is_active: true,
})

watch(
  () => form.account_type,
  (type) => {
    form.normal_balance = type === 'ASSET' || type === 'EXPENSE' ? 'DEBIT' : 'CREDIT'
  },
  { immediate: true },
)

const accountTypeOptions = [
  { value: 'ASSET', label: 'Aset' },
  { value: 'LIABILITY', label: 'Kewajiban' },
  { value: 'EQUITY', label: 'Ekuitas' },
  { value: 'REVENUE', label: 'Pendapatan' },
  { value: 'EXPENSE', label: 'Beban' },
]

const normalBalanceOptions = [
  { value: 'DEBIT', label: 'Debit' },
  { value: 'CREDIT', label: 'Kredit' },
]

const accountTypeLabel: Record<string, string> = {
  ASSET: 'Aset',
  LIABILITY: 'Kewajiban',
  EQUITY: 'Ekuitas',
  REVENUE: 'Pendapatan',
  EXPENSE: 'Beban',
}

const parentOptions = computed(() =>
  accounts.value
    .filter((account) => account.is_active)
    .map((account) => ({ value: account.id, label: `${account.code} - ${account.name}` })),
)

const sortedAccounts = computed(() => [...accounts.value].sort((a, b) => a.code.localeCompare(b.code)))

onMounted(() => {
  accountsStore.load()
})

function resetForm() {
  form.code = ''
  form.name = ''
  form.account_type = 'ASSET'
  form.sub_type = ''
  form.normal_balance = 'DEBIT'
  form.parent_id = null
  form.description = ''
  form.is_active = true
}

async function handleSubmit() {
  submitting.value = true
  feedback.value = null
  try {
    await accountsStore.create({
      code: form.code.trim(),
      name: form.name.trim(),
      account_type: form.account_type,
      sub_type: form.sub_type?.trim() || null,
      normal_balance: form.normal_balance,
      parent_id: form.parent_id ?? null,
      description: form.description?.trim() || null,
      is_active: form.is_active,
    })
    feedback.value = 'Akun berhasil ditambahkan.'
    resetForm()
    showCreate.value = false
  } catch (err) {
    feedback.value = (err as Error).message ?? 'Gagal menambahkan akun.'
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <div class="space-y-7">
    <header class="flex flex-wrap items-center justify-between gap-4">
      <div class="space-y-2">
        <h1 class="text-3xl font-semibold text-brand-900">Bagan Akun</h1>
        <p class="max-w-xl text-sm text-brand-600">
          Kelola struktur akun perusahaan dengan mudah. Setiap perubahan langsung tersimpan ke database lokal.
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <button type="button" class="btn-muted" @click="accountsStore.load(true)" :disabled="loading">
          Muat ulang
        </button>
        <button type="button" class="btn-primary" @click="showCreate = true">
          + Tambah Akun
        </button>
      </div>
    </header>

    <section class="surface-card space-y-6 p-6">
      <div v-if="showCreate" class="rounded-4xl border border-brand-200 bg-brand-50 p-5">
        <form class="grid gap-4" @submit.prevent="handleSubmit">
          <div class="grid gap-4 sm:grid-cols-2">
            <label class="space-y-1 text-sm text-brand-600">
              Kode Akun
              <input
                v-model.trim="form.code"
                type="text"
                required
                placeholder="Misal: 1101"
                class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
              />
            </label>
            <label class="space-y-1 text-sm text-brand-600">
              Nama Akun
              <input
                v-model.trim="form.name"
                type="text"
                required
                placeholder="Nama akun"
                class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
              />
            </label>
          </div>

          <div class="grid gap-4 sm:grid-cols-3">
            <label class="space-y-1 text-sm text-brand-600">
              Tipe Akun
              <select
                v-model="form.account_type"
                class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
              >
                <option v-for="option in accountTypeOptions" :key="option.value" :value="option.value">
                  {{ option.label }}
                </option>
              </select>
            </label>
            <label class="space-y-1 text-sm text-brand-600">
              Saldo Normal
              <select
                v-model="form.normal_balance"
                class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
              >
                <option v-for="option in normalBalanceOptions" :key="option.value" :value="option.value">
                  {{ option.label }}
                </option>
              </select>
            </label>
            <label class="space-y-1 text-sm text-brand-600">
              Parent Akun (opsional)
              <select
                v-model.number="form.parent_id"
                class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
              >
                <option :value="null">Tanpa parent</option>
                <option v-for="option in parentOptions" :key="option.value" :value="option.value">
                  {{ option.label }}
                </option>
              </select>
            </label>
          </div>

          <label class="space-y-1 text-sm text-brand-600">
            Deskripsi (opsional)
            <textarea
              v-model.trim="form.description"
              rows="3"
              placeholder="Ringkasan fungsi akun"
              class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
            ></textarea>
          </label>

          <div class="flex items-center gap-3">
            <label class="inline-flex items-center gap-2 text-sm text-brand-600">
              <input v-model="form.is_active" type="checkbox" class="h-4 w-4 rounded border-brand-300 text-brand-600 focus:ring-brand-400" />
              Aktif
            </label>
          </div>

          <div class="flex flex-wrap items-center gap-3">
            <button type="submit" class="btn-primary" :disabled="submitting">
              {{ submitting ? 'Menyimpan...' : 'Simpan Akun' }}
            </button>
            <button type="button" class="btn-muted" @click="() => { resetForm(); showCreate = false }" :disabled="submitting">
              Batal
            </button>
            <p v-if="feedback" class="text-sm text-brand-500">{{ feedback }}</p>
          </div>
        </form>
      </div>

      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="text-sm text-brand-500">
          {{ sortedAccounts.length }} akun aktif • Status: {{ loading ? 'Memuat data...' : 'Sinkron' }}
        </div>
        <p v-if="error" class="text-sm text-accent-500">{{ error }}</p>
      </div>

      <div class="overflow-hidden rounded-5xl border border-brand-200">
        <table class="min-w-full divide-y divide-brand-100 text-sm text-brand-700">
          <thead class="bg-brand-50 text-xs uppercase tracking-[0.25em] text-brand-400">
            <tr>
              <th class="px-6 py-4 text-left font-medium">Kode</th>
              <th class="px-6 py-4 text-left font-medium">Nama Akun</th>
              <th class="px-6 py-4 text-left font-medium">Tipe</th>
              <th class="px-6 py-4 text-left font-medium">Saldo Normal</th>
              <th class="px-6 py-4 text-left font-medium">Status</th>
            </tr>
          </thead>
          <tbody v-if="sortedAccounts.length" class="divide-y divide-brand-100 bg-white">
            <tr v-for="account in sortedAccounts" :key="account.id" class="transition hover:bg-brand-50">
              <td class="px-6 py-4 font-semibold text-brand-900">{{ account.code }}</td>
              <td class="px-6 py-4 text-brand-700">
                <div class="font-medium text-brand-900">{{ account.name }}</div>
                <p v-if="account.description" class="text-xs text-brand-500">{{ account.description }}</p>
              </td>
              <td class="px-6 py-4 text-brand-600">
                <span class="rounded-full bg-brand-100 px-3 py-1 text-xs font-medium text-brand-700">
                  {{ accountTypeLabel[account.account_type] ?? account.account_type }}
                </span>
              </td>
              <td class="px-6 py-4 text-brand-600">{{ account.normal_balance === 'DEBIT' ? 'Debit' : 'Kredit' }}</td>
              <td class="px-6 py-4">
                <span
                  :class="[
                    'rounded-full px-3 py-1 text-xs font-medium',
                    account.is_active ? 'bg-brand-500/30 text-brand-800' : 'bg-brand-100 text-brand-400',
                  ]"
                >
                  {{ account.is_active ? 'Aktif' : 'Nonaktif' }}
                </span>
              </td>
            </tr>
          </tbody>
          <tbody v-else class="bg-white">
            <tr>
              <td class="px-6 py-10 text-center text-sm text-brand-500" colspan="5">
                {{ loading ? 'Memuat daftar akun...' : 'Belum ada akun tersimpan.' }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <p class="text-xs text-brand-500">
        Catatan: Struktur hierarki akun akan dikembangkan pada sprint modul Akun. Saat ini tabel ini terhubung ke database SQLite dan siap menerima data nyata.
      </p>
    </section>
  </div>
</template>
