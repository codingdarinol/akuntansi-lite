<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { storeToRefs } from 'pinia'

import { useAccountsStore } from '@/stores/accounts'
import { useJournalsStore } from '@/stores/journals'
import type { JournalLineInput } from '@/types/journal'

const accountsStore = useAccountsStore()
const journalsStore = useJournalsStore()
const { accounts } = storeToRefs(accountsStore)
const { journals, loading, error } = storeToRefs(journalsStore)

const showCreate = ref(false)
const submitting = ref(false)
const formError = ref('')

const journalForm = reactive({
  journal_date: new Date().toISOString().slice(0, 10),
  memo: '',
  source: '',
  lines: [
    { account_id: null, debit: 0, credit: 0 },
    { account_id: null, debit: 0, credit: 0 },
  ] as JournalLineInput[],
})

const accountOptions = computed(() =>
  accounts.value.map((account) => ({ value: account.id, label: `${account.code} - ${account.name}` })),
)

const totalDebit = computed(() =>
  journalForm.lines.reduce((sum, line) => sum + Number(line.debit ?? 0), 0),
)
const totalCredit = computed(() =>
  journalForm.lines.reduce((sum, line) => sum + Number(line.credit ?? 0), 0),
)

const currency = new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR' })

onMounted(() => {
  accountsStore.load()
  journalsStore.load()
})

function resetForm() {
  journalForm.journal_date = new Date().toISOString().slice(0, 10)
  journalForm.memo = ''
  journalForm.source = ''
  journalForm.lines = [
    { account_id: null, debit: 0, credit: 0 },
    { account_id: null, debit: 0, credit: 0 },
  ]
  formError.value = ''
}

function addLine() {
  journalForm.lines.push({ account_id: null, debit: 0, credit: 0 })
}

function removeLine(index: number) {
  if (journalForm.lines.length <= 2) return
  journalForm.lines.splice(index, 1)
}

async function submitJournal() {
  formError.value = ''

  if (journalForm.lines.some((line) => !line.account_id)) {
    formError.value = 'Pilih akun untuk setiap baris jurnal.'
    return
  }

  if (journalForm.lines.every((line) => Number(line.debit) === 0 && Number(line.credit) === 0)) {
    formError.value = 'Isi minimal satu nilai debit atau kredit.'
    return
  }

  const roundedDebit = Number(totalDebit.value.toFixed(2))
  const roundedCredit = Number(totalCredit.value.toFixed(2))
  if (Math.abs(roundedDebit - roundedCredit) > 0.01) {
    formError.value = 'Total debit dan kredit harus seimbang.'
    return
  }

  submitting.value = true
  try {
    await journalsStore.create({
      journal_number: null,
      journal_date: journalForm.journal_date,
      memo: journalForm.memo || null,
      source: journalForm.source || null,
      lines: journalForm.lines.map((line) => ({
        account_id: line.account_id!,
        debit: Number(line.debit) || 0,
        credit: Number(line.credit) || 0,
        description: line.debit > 0 ? 'Debit' : line.credit > 0 ? 'Kredit' : null,
      })),
    })
    resetForm()
    showCreate.value = false
  } catch (err) {
    formError.value = (err as Error).message ?? 'Gagal menyimpan jurnal.'
  } finally {
    submitting.value = false
  }
}

function handleReload() {
  journalsStore.load({}, true)
}
</script>

<template>
  <div class="space-y-7">
    <header class="flex flex-wrap items-center justify-between gap-4">
      <div>
        <h1 class="text-3xl font-semibold text-brand-900">Jurnal Umum</h1>
        <p class="text-sm text-brand-600">
          Lihat dan catat transaksi double-entry di seluruh modul. Setiap entri terhubung langsung dengan buku besar.
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <button type="button" class="btn-muted" @click="handleReload" :disabled="loading">
          Muat ulang
        </button>
        <button type="button" class="btn-primary" @click="showCreate = true">
          + Transaksi Manual
        </button>
      </div>
    </header>

    <section class="surface-card space-y-6 p-6">
      <div v-if="showCreate" class="rounded-4xl border border-brand-200 bg-brand-50 p-5">
        <form class="space-y-4" @submit.prevent="submitJournal">
          <div class="grid gap-4 md:grid-cols-2">
            <label class="space-y-1 text-sm text-brand-600">
              Tanggal Jurnal
              <input
                v-model="journalForm.journal_date"
                type="date"
                required
                class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
              />
            </label>
            <label class="space-y-1 text-sm text-brand-600">
              Referensi (opsional)
              <input
                v-model.trim="journalForm.source"
                type="text"
                placeholder="Sumber / nomor dokumen"
                class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
              />
            </label>
          </div>

          <label class="space-y-1 text-sm text-brand-600">
            Memo (opsional)
            <textarea
              v-model.trim="journalForm.memo"
              rows="2"
              placeholder="Catatan singkat untuk jurnal ini"
              class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
            ></textarea>
          </label>

          <div class="space-y-3">
            <div
              v-for="(line, index) in journalForm.lines"
              :key="index"
              class="grid gap-3 rounded-3xl border border-brand-200 bg-white/80 p-4 sm:grid-cols-[2fr,1fr,1fr,auto]"
            >
              <label class="space-y-1 text-sm text-brand-600">
                Akun
                <select
                  v-model.number="line.account_id"
                  required
                  class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
                >
                  <option :value="null">Pilih akun</option>
                  <option v-for="option in accountOptions" :key="option.value" :value="option.value">
                    {{ option.label }}
                  </option>
                </select>
              </label>
              <label class="space-y-1 text-sm text-brand-600">
                Debit
                <input
                  v-model.number="line.debit"
                  type="number"
                  min="0"
                  step="0.01"
                  class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
                />
              </label>
              <label class="space-y-1 text-sm text-brand-600">
                Kredit
                <input
                  v-model.number="line.credit"
                  type="number"
                  min="0"
                  step="0.01"
                  class="w-full rounded-3xl border border-brand-200 bg-white px-4 py-2.5 text-sm text-brand-900 focus:border-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300/60"
                />
              </label>
              <button
                v-if="journalForm.lines.length > 2"
                type="button"
                class="btn-muted self-end"
                @click="removeLine(index)"
              >
                Hapus
              </button>
            </div>
            <button type="button" class="btn-muted" @click="addLine">+ Tambah baris</button>
          </div>

          <div class="flex flex-wrap items-center gap-3 text-sm text-brand-600">
            <span>Total Debit: <strong>{{ currency.format(totalDebit) }}</strong></span>
            <span>Total Kredit: <strong>{{ currency.format(totalCredit) }}</strong></span>
          </div>

          <div class="flex flex-wrap items-center gap-3">
            <button type="submit" class="btn-primary" :disabled="submitting">
              {{ submitting ? 'Menyimpan...' : 'Simpan Jurnal' }}
            </button>
            <button type="button" class="btn-muted" @click="() => { resetForm(); showCreate = false }" :disabled="submitting">
              Batal
            </button>
            <p v-if="formError" class="text-sm text-accent-500">{{ formError }}</p>
          </div>
        </form>
      </div>

      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="text-sm text-brand-500">
          {{ journals.length }} jurnal • Status: {{ loading ? 'Memuat data...' : 'Sinkron' }}
        </div>
        <p v-if="error" class="text-sm text-accent-500">{{ error }}</p>
      </div>

      <div class="overflow-hidden rounded-5xl border border-brand-200">
        <div class="hidden grid-cols-[120px,1.2fr,1fr,1fr,1fr] gap-4 bg-brand-50 px-6 py-3 text-xs uppercase tracking-[0.25em] text-brand-400 md:grid">
          <span>Tanggal</span>
          <span>No. Jurnal</span>
          <span>Deskripsi</span>
          <span>Debit</span>
          <span>Kredit</span>
        </div>

        <div v-if="journals.length" class="divide-y divide-brand-100 bg-white text-sm text-brand-700">
          <div
            v-for="journal in journals"
            :key="journal.id"
            class="grid gap-4 px-6 py-4 md:grid-cols-[120px,1.2fr,1fr,1fr,1fr]"
          >
            <span class="font-medium text-brand-900">{{ journal.journal_date }}</span>
            <span>
              <div class="font-semibold text-brand-900">{{ journal.journal_number ?? 'Manual' }}</div>
              <p v-if="journal.memo" class="text-xs text-brand-500">{{ journal.memo }}</p>
            </span>
            <span class="text-brand-600">{{ journal.source ?? '-' }}</span>
            <span class="font-semibold text-brand-600">{{ currency.format(journal.total_debit) }}</span>
            <span class="font-semibold text-brand-600">{{ currency.format(journal.total_credit) }}</span>
          </div>
        </div>

        <div v-else class="px-6 py-10 text-center text-sm text-brand-500">
          {{ loading ? 'Memuat jurnal...' : 'Belum ada jurnal yang tercatat.' }}
        </div>
      </div>
    </section>
  </div>
</template>
