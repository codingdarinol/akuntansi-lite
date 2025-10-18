<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'

import { useBalanceSheetStore } from '@/stores/balanceSheet'
import type { BalanceSheetSection, ReportAccountBalance } from '@/types/reports'

const balanceSheetStore = useBalanceSheetStore()
const { report, loading, error } = storeToRefs(balanceSheetStore)

const today = new Date()
const asOf = ref(formatDateInput(today))

interface AccountRow extends ReportAccountBalance {
  level: number
}

interface BalanceSheetSectionView extends BalanceSheetSection {
  accounts: AccountRow[]
}

const currencyFormatter = computed(
  () =>
    new Intl.NumberFormat('id-ID', {
      style: 'currency',
      currency: report.value?.currency ?? 'IDR',
      maximumFractionDigits: 0,
    }),
)

const displayedSections = computed<BalanceSheetSectionView[]>(() => {
  if (!report.value) return []
  return report.value.sections.map((section) => transformSection(section))
})

const totalsCardData = computed(() => report.value?.sections ?? [])

const liabilitiesAndEquity = computed(() => report.value?.total_liabilities_and_equity ?? 0)

const generatedAtLabel = computed(() => (report.value ? formatDateTime(report.value.generated_at) : ''))

function formatDateInput(date: Date) {
  const tzOffset = date.getTimezoneOffset() * 60000
  return new Date(date.getTime() - tzOffset).toISOString().slice(0, 10)
}

function formatDate(value: string) {
  return formatDateLabel(value || asOf.value)
}

function formatCurrency(value: number) {
  return currencyFormatter.value.format(value)
}

function formatDateLabel(dateString: string) {
  const [year, month, day] = dateString.split('-').map(Number)
  if (!year || !month || !day) return dateString
  const date = new Date(Date.UTC(year, month - 1, day))
  return new Intl.DateTimeFormat('id-ID', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  }).format(date)
}

function formatDateTime(timestamp: string) {
  const date = new Date(timestamp)
  if (Number.isNaN(date.getTime())) return timestamp
  return new Intl.DateTimeFormat('id-ID', {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(date)
}

function transformSection(section: BalanceSheetSection): BalanceSheetSectionView {
  const relevant = filterAccounts(section.accounts)
  const levelMap = computeLevels(section.accounts)
  return {
    ...section,
    accounts: relevant.map((account) => ({
      ...account,
      level: levelMap.get(account.account_id) ?? 0,
    })),
  }
}

function filterAccounts(accounts: ReportAccountBalance[]) {
  const byId = new Map(accounts.map((account) => [account.account_id, account]))
  const included = new Set<number>()

  const includeWithParents = (account: ReportAccountBalance) => {
    if (included.has(account.account_id)) return
    included.add(account.account_id)
    if (account.parent_id && byId.has(account.parent_id)) {
      includeWithParents(byId.get(account.parent_id)!)
    }
  }

  accounts.forEach((account) => {
    if (Math.abs(account.balance) > 0.004) {
      includeWithParents(account)
    }
  })

  return accounts.filter((account) => included.has(account.account_id))
}

function computeLevels(accounts: ReportAccountBalance[]) {
  const byId = new Map(accounts.map((account) => [account.account_id, account]))
  const levels = new Map<number, number>()

  const resolve = (account: ReportAccountBalance): number => {
    if (levels.has(account.account_id)) return levels.get(account.account_id)!
    let level = 0
    if (account.parent_id && byId.has(account.parent_id)) {
      level = resolve(byId.get(account.parent_id)!) + 1
    }
    levels.set(account.account_id, level)
    return level
  }

  accounts.forEach(resolve)
  return levels
}

async function fetchReport(force = false) {
  try {
    await balanceSheetStore.load({ as_of: asOf.value }, force)
    if (report.value?.as_of) {
      asOf.value = report.value.as_of
    }
  } catch (err) {
    console.error('Gagal memuat neraca', err)
  }
}

onMounted(() => {
  if (report.value?.as_of) {
    asOf.value = report.value.as_of
  } else {
    fetchReport()
  }
})
</script>

<template>
  <div class="space-y-7">
    <header class="space-y-2">
      <h1 class="text-3xl font-semibold text-brand-900">Neraca</h1>
      <p class="text-sm text-brand-600">
        Komposisi aset, kewajiban, dan ekuitas per
        {{ report ? formatDate(report.as_of) : formatDate(asOf) }}.
      </p>
    </header>

    <section class="surface-card space-y-6 p-6">
      <form class="flex flex-col gap-4 md:flex-row md:items-end md:justify-between" @submit.prevent="fetchReport(true)">
        <div class="flex flex-wrap items-end gap-4">
          <label class="flex flex-col text-sm text-brand-700">
            <span class="text-xs font-semibold uppercase tracking-[0.28em] text-brand-400">Per Tanggal</span>
            <input
              v-model="asOf"
              type="date"
              name="as_of"
              class="form-input mt-1 w-52 rounded-2xl border-brand-200 bg-white text-brand-900 focus:border-brand-500 focus:ring-brand-500"
            />
          </label>
          <div class="flex gap-3">
            <button
              type="submit"
              class="rounded-2xl bg-brand-900 px-4 py-2 text-sm font-medium text-white transition hover:bg-brand-800 disabled:cursor-not-allowed disabled:opacity-60"
              :disabled="loading"
            >
              Terapkan
            </button>
            <button
              type="button"
              class="rounded-2xl border border-brand-200 px-4 py-2 text-sm font-medium text-brand-700 transition hover:border-brand-300 hover:text-brand-900 disabled:cursor-not-allowed disabled:opacity-60"
              :disabled="loading"
              @click="fetchReport(true)"
            >
              Segarkan
            </button>
          </div>
        </div>
        <p v-if="generatedAtLabel" class="text-xs text-brand-500">Diperbarui {{ generatedAtLabel }}</p>
      </form>

      <div
        v-if="error"
        class="rounded-3xl border border-red-200 bg-red-50 p-5 text-sm text-red-700"
      >
        <div class="flex flex-wrap items-center justify-between gap-3">
          <span>{{ error }}</span>
          <button
            type="button"
            class="rounded-2xl border border-red-200 px-3 py-1.5 text-xs font-semibold text-red-700 transition hover:border-red-300 hover:text-red-800"
            :disabled="loading"
            @click="fetchReport(true)"
          >
            Coba lagi
          </button>
        </div>
      </div>

      <div
        v-else-if="loading && !report"
        class="rounded-3xl border border-brand-100 bg-brand-50/70 p-6 text-sm text-brand-500"
      >
        Memuat data neraca...
      </div>

      <template v-else-if="report">
        <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
          <article
            v-for="section in totalsCardData"
            :key="section.key"
            class="glass-panel border-brand-200 bg-brand-50/80 p-5 text-brand-900"
          >
            <p class="text-xs uppercase tracking-[0.28em] text-brand-400">{{ section.label }}</p>
            <p class="mt-2 text-xl font-semibold">{{ formatCurrency(section.total) }}</p>
          </article>
          <article class="glass-panel border-brand-200 bg-brand-50/80 p-5 text-brand-900">
            <p class="text-xs uppercase tracking-[0.28em] text-brand-400">Liabilitas + Ekuitas</p>
            <p class="mt-2 text-xl font-semibold">{{ formatCurrency(liabilitiesAndEquity) }}</p>
          </article>
        </div>

        <div class="grid gap-6 lg:grid-cols-2">
          <section
            v-for="section in displayedSections"
            :key="section.key"
            class="rounded-3xl border border-brand-100 bg-white/90 p-5 shadow-card"
          >
            <header class="flex items-center justify-between">
              <div>
                <h3 class="text-lg font-semibold text-brand-900">{{ section.label }}</h3>
                <p class="text-xs text-brand-500">
                  Total {{ section.label.toLowerCase() }}: {{ formatCurrency(section.total) }}
                </p>
              </div>
            </header>

            <div class="mt-4 divide-y divide-brand-100">
              <p
                v-if="section.accounts.length === 0"
                class="p-4 text-sm text-brand-500"
              >
                Belum ada saldo tercatat untuk kategori ini.
              </p>
              <div
                v-for="account in section.accounts"
                v-else
                :key="account.account_id"
                class="flex items-center justify-between py-2 text-sm text-brand-700"
              >
                <div class="flex flex-col">
                  <span
                    class="font-medium text-brand-900"
                    :style="{ paddingLeft: `${account.level * 18}px` }"
                  >
                    {{ account.code }} · {{ account.name }}
                  </span>
                </div>
                <span class="font-semibold text-brand-900">{{ formatCurrency(account.balance) }}</span>
              </div>
            </div>
          </section>
        </div>
      </template>
      <div v-else class="rounded-3xl border border-brand-100 bg-brand-50/70 p-6 text-sm text-brand-500">
        Data neraca belum tersedia. Tambahkan transaksi untuk melihat ringkasan posisi keuangan.
      </div>
    </section>
  </div>
</template>
