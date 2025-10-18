1. 🎯 Pendahuluan
1.1. Tujuan Dokumen
Dokumen ini berfungsi sebagai panduan teknis utama (single source of truth) untuk perencanaan, pengembangan, dan pengujian aplikasi akuntansi desktop LokalBuku. Tujuannya adalah untuk mengeliminasi ambiguitas dan menyelaraskan visi teknis di antara semua pemangku kepentingan. Dokumen ini merinci persyaratan fungsional dan non-fungsional, arsitektur sistem, desain database yang terperinci, alur kerja fitur, serta rencana pengembangan bertahap yang dapat diukur.
1.2. Visi Produk
Menciptakan aplikasi akuntansi desktop yang cepat, aman, dan offline-first untuk usaha kecil dan menengah (UKM) atau freelancer di Indonesia. Di tengah meningkatnya kekhawatiran akan privasi data dan ketergantungan pada konektivitas internet, LokalBuku hadir sebagai solusi yang menempatkan pengguna sebagai pemilik data seutuhnya. Dengan menyimpan 100% data di perangkat lokal, kami menjamin kerahasiaan, kedaulatan data, dan fungsionalitas penuh tanpa memerlukan koneksi internet, mengatasi tantangan infrastruktur digital yang belum merata.
1.3. Lingkup Proyek
Proyek ini mencakup pengembangan aplikasi end-to-end, mulai dari desain awal hingga rilis versi 1.0. Lingkup fungsional mencakup, namun tidak terbatas pada:
•	Manajemen Bagan Akun (CoA): Pembuatan dan pengelolaan struktur akun hierarkis yang fleksibel.
•	Pencatatan Transaksi: Entri jurnal umum berbasis double-entry yang akurat dan dapat dilacak.
•	Modul Operasional: Modul Penjualan (Invoice) dan Pembelian (Faktur) yang terintegrasi penuh dengan jurnal dan inventori.
•	Manajemen Data Master: Pusat pengelolaan data Pelanggan, Pemasok, Produk & Jasa, serta Aset Tetap.
•	Pelaporan Keuangan Standar: Generasi otomatis Laporan Posisi Keuangan (Neraca), Laporan Laba Rugi, Laporan Arus Kas, dan Laporan Perubahan Ekuitas.
•	Pelaporan Analitis: Modul untuk Analisis Beban mendalam dan kalkulasi Rasio Kinerja Keuangan untuk pengambilan keputusan.
•	Dashboard Visual: Ringkasan bisnis interaktif melalui grafik dan metrik kunci.
•	Pengaturan Aplikasi: Konfigurasi fleksibel untuk data perusahaan, format penomoran, dan preferensi pengguna.
1.4. Target Pengguna
•	Pemilik Usaha Kecil & Menengah (UKM): Pengguna utama yang membutuhkan alat pembukuan yang mudah digunakan tanpa biaya langganan yang mahal.
•	Akuntan & Staf Keuangan: Profesional yang memerlukan fungsionalitas akuntansi lengkap, akurasi data, dan kemampuan pelaporan yang andal.
•	Freelancer dan Profesional Mandiri: Individu yang perlu melacak pendapatan, pengeluaran, dan membuat faktur secara efisien.
•	Mahasiswa Akuntansi: Sebagai sarana belajar praktik akuntansi dengan menggunakan perangkat lunak yang mensimulasikan lingkungan bisnis nyata.
2. 🏛️ Arsitektur & Tumpukan Teknologi (Tech Stack)
2.1. Arsitektur Umum
Aplikasi mengadopsi arsitektur Tauri, yang secara fundamental lebih aman dan lebih ringan dibandingkan alternatif berbasis Electron. Arsitektur ini memisahkan secara tegas antara proses antarmuka (Frontend) dan proses inti (Backend).
•	Frontend (Tauri WebView): Bertanggung jawab penuh atas render UI dan interaksi pengguna (UX). Dibangun sebagai Single Page Application (SPA) menggunakan Vue 3. Lapisan ini tidak memiliki akses langsung ke sistem file atau database, sehingga mengurangi permukaan serangan. Setiap permintaan data atau aksi harus melalui API internal yang diekspos oleh backend Rust.
•	Backend (Rust Core): Proses Rust yang berjalan secara native, memberikan performa mendekati aplikasi asli. Bertugas sebagai jembatan ke sistem operasi, menjalankan logika bisnis kritis (kalkulasi laporan, validasi), dan menjadi satu-satunya gerbang untuk semua operasi database SQLite. Komunikasi antara Frontend dan Backend terjadi secara aman dan efisien melalui Tauri Commands, yang pada dasarnya adalah pemanggilan fungsi Rust dari JavaScript dengan serialisasi data otomatis.
2.2. Tumpukan Teknologi
Pilihan teknologi didasarkan pada prinsip kinerja, keamanan, dan produktivitas pengembang.
- Core Framework: Tauri v2.x (diadopsi karena kompatibilitas plugin-sql terbaru) - Dipilih karena jejak memori yang kecil, ukuran bundle aplikasi yang minimal, dan keamanan bawaan dari Rust.
- Frontend Framework: Vue 3 (Composition API) + Vite - Composition API memungkinkan logika yang kompleks diorganisir ke dalam unit-unit yang dapat digunakan kembali (composables), sementara Vite memberikan pengalaman pengembangan yang sangat cepat.
- Routing: vue-router - Solusi routing resmi untuk Vue.js.
- Manajemen State: Pinia - State manager yang ringan, modular, dan memiliki integrasi TypeScript yang sangat baik, membuatnya lebih mudah untuk dikelola dibandingkan Vuex pada proyek skala besar.
- Styling: TailwindCSS v3 + Headless UI - TailwindCSS mempercepat pengembangan UI secara konsisten. Headless UI menyediakan komponen aksesibel (seperti Modal, Dropdown) tanpa gaya, yang kemudian kita gayakan dengan Tailwind.
- Desain Sistem UI: Palet utama navy (#3D5A89), aksen kuning (#F6B713), dan putih. Kartu memakai radius besar (rounded-4xl/5xl), shadow-card, serta tipografi kontras (teks putih pada permukaan gelap, brand-900 pada permukaan terang). Badge status menggunakan konvensi warna konsisten: brand untuk Paid, accent untuk Due Soon, abu-abu untuk Draft/Cancelled.
- Library Ikon: Lucide Icons (lucide-vue-next) - Ringan, konsisten, dan komprehensif.
- Library Grafik: ApexCharts.js (vue3-apexcharts) - Menyediakan grafik yang interaktif, responsif, dan mudah diintegrasikan dengan Vue.
- Backend Language: Rust - Dipilih karena jaminan keamanannya (tanpa garbage collector, manajemen memori yang ketat) dan performa tinggi, sangat ideal untuk logika bisnis inti dan operasi database.
- Database: SQLite (via tauri-plugin-sql) - Database berbasis file yang andal, portabel, dan tidak memerlukan server terpisah, sempurna untuk aplikasi desktop offline.
- Migrasi Database: rusqlite_migration crate - Kritis untuk mengelola evolusi skema database di versi aplikasi yang akan datang tanpa menghapus data pengguna.
3. 🗃️ Desain Skema Database (Revisi Final & Lengkap)
3.1. Filosofi Desain
Skema database ini dirancang berdasarkan prinsip-prinsip berikut:
•	Integritas Data: Menggunakan constraints seperti FOREIGN KEY, UNIQUE, dan CHECK secara ketat untuk memastikan data yang tersimpan valid dan konsisten. Relasi antar tabel didefinisikan dengan jelas untuk mencegah data yatim piatu (orphaned data).
•	Normalisasi: Didesain hingga Bentuk Normal Ketiga (3NF) untuk mengurangi redundansi data dan meningkatkan efisiensi penyimpanan.
•	Keterlacakan (Traceability): Setiap transaksi finansial dapat dilacak kembali ke dokumen sumbernya (misalnya, Jurnal -> Faktur Penjualan -> Pelanggan), menciptakan jejak audit yang jelas.
•	Skalabilitas: Didesain untuk menangani pertumbuhan data di masa depan dengan menambahkan indeks pada kolom yang sering di-query untuk menjaga performa.
3.2. Skema SQL Lengkap
Berikut adalah skema lengkap untuk database SQLite aplikasi LokalBuku.
-- ================================================================================= --
-- MODUL INTI & AKUNTANSI
-- ================================================================================= --

-- Tabel Pengaturan Aplikasi: Menyimpan konfigurasi global.
CREATE TABLE app_settings (
    key TEXT PRIMARY KEY NOT NULL, -- e.g., 'company_name', 'company_address', 'next_invoice_number'
    value TEXT
);

-- Tabel Akun (Chart of Accounts) dengan Struktur Hierarki
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY, -- Menggunakan kode akun 6 digit (e.g., 110101)
    parent_id INTEGER,      -- FK ke id sendiri, NULL jika level 1.
    name TEXT NOT NULL,
    level INTEGER NOT NULL CHECK(level IN (1, 2, 3, 4)),
    is_postable BOOLEAN NOT NULL DEFAULT 0, -- Hanya TRUE untuk akun level 4
    type TEXT NOT NULL CHECK(type IN ('ASET', 'LIABILITAS', 'EKUITAS', 'PENGHASILAN', 'BEBAN')),
    normal_balance TEXT NOT NULL CHECK(normal_balance IN ('DEBIT', 'CREDIT')),
    icon_name TEXT,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES accounts(id)
);

-- Tabel Jurnal Umum (Transactions): Header untuk setiap entri jurnal.
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_date DATE NOT NULL,
    description TEXT,
    reference_number TEXT,
    source_document TEXT, -- 'INVOICE', 'BILL', 'PAYMENT_IN', 'PAYMENT_OUT', 'MANUAL_JOURNAL', 'DEPRECIATION'
    source_document_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_transactions_date ON transactions(transaction_date);

-- Tabel Detail Jurnal (Double-Entry): Baris-baris dalam setiap jurnal.
CREATE TABLE journal_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    debit REAL DEFAULT 0,
    credit REAL DEFAULT 0,
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE,
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE RESTRICT
);

-- ================================================================================= --
-- MODUL DATA MASTER
-- ================================================================================= --

-- Tabel Kontak (Pelanggan & Pemasok)
CREATE TABLE contacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL CHECK(type IN ('PELANGGAN', 'PEMASOK')),
    name TEXT NOT NULL,
    company_name TEXT,
    email TEXT,
    phone TEXT,
    address TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Tabel Produk & Jasa
CREATE TABLE items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    sku TEXT UNIQUE,
    description TEXT,
    type TEXT NOT NULL CHECK(type IN ('PRODUK', 'JASA')),
    purchase_price REAL DEFAULT 0,
    sale_price REAL DEFAULT 0,
    is_inventory BOOLEAN DEFAULT 0,
    quantity_on_hand REAL DEFAULT 0,
    inventory_account_id INTEGER, -- Akun persediaan (e.g., 110603)
    cogs_account_id INTEGER,      -- Akun HPP (e.g., 510000)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (inventory_account_id) REFERENCES accounts(id) ON DELETE RESTRICT,
    FOREIGN KEY (cogs_account_id) REFERENCES accounts(id) ON DELETE RESTRICT
);

-- ================================================================================= --
-- MODUL PENJUALAN
-- ================================================================================= --

-- Tabel Penjualan (Invoices)
CREATE TABLE invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_id INTEGER NOT NULL,
    invoice_number TEXT UNIQUE NOT NULL,
    issue_date DATE NOT NULL,
    due_date DATE,
    total_amount REAL NOT NULL,
    amount_paid REAL DEFAULT 0,
    status TEXT DEFAULT 'BELUM DIBAYAR' CHECK(status IN ('BELUM DIBAYAR', 'DIBAYAR SEBAGIAN', 'LUNAS', 'DRAFT')),
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE RESTRICT
);
CREATE INDEX idx_invoices_status ON invoices(status);
CREATE INDEX idx_invoices_issue_date ON invoices(issue_date);

-- Tabel Detail Item Penjualan
CREATE TABLE invoice_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    description TEXT,
    quantity REAL NOT NULL,
    unit_price REAL NOT NULL,
    total REAL NOT NULL,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE RESTRICT
);

-- Tabel Pembayaran Masuk (Penerimaan dari Pelanggan)
CREATE TABLE invoice_payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_id INTEGER NOT NULL,
    payment_date DATE NOT NULL,
    amount REAL NOT NULL,
    deposit_to_account_id INTEGER NOT NULL, -- Akun Kas/Bank tempat uang masuk
    transaction_id INTEGER, -- FK ke jurnal yang dibuat untuk pembayaran ini
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE,
    FOREIGN KEY (deposit_to_account_id) REFERENCES accounts(id) ON DELETE RESTRICT,
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE SET NULL
);

-- ================================================================================= --
-- MODUL PEMBELIAN
-- ================================================================================= --

-- Tabel Pembelian (Bills)
CREATE TABLE bills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_id INTEGER NOT NULL,
    bill_number TEXT UNIQUE NOT NULL,
    issue_date DATE NOT NULL,
    due_date DATE,
    total_amount REAL NOT NULL,
    amount_paid REAL DEFAULT 0,
    status TEXT DEFAULT 'BELUM DIBAYAR' CHECK(status IN ('BELUM DIBAYAR', 'DIBAYAR SEBAGIAN', 'LUNAS', 'DRAFT')),
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE RESTRICT
);

-- Tabel Detail Item Pembelian
CREATE TABLE bill_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bill_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    description TEXT,
    quantity REAL NOT NULL,
    unit_price REAL NOT NULL,
    total REAL NOT NULL,
    FOREIGN KEY (bill_id) REFERENCES bills(id) ON DELETE CASCADE,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE RESTRICT
);

-- Tabel Pembayaran Keluar (Pembayaran ke Pemasok)
CREATE TABLE bill_payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bill_id INTEGER NOT NULL,
    payment_date DATE NOT NULL,
    amount REAL NOT NULL,
    paid_from_account_id INTEGER NOT NULL, -- Akun Kas/Bank tempat uang keluar
    transaction_id INTEGER, -- FK ke jurnal yang dibuat untuk pembayaran ini
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (bill_id) REFERENCES bills(id) ON DELETE CASCADE,
    FOREIGN KEY (paid_from_account_id) REFERENCES accounts(id) ON DELETE RESTRICT,
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE SET NULL
);

-- ================================================================================= --
-- MODUL ASET TETAP
-- ================================================================================= --

CREATE TABLE fixed_assets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    acquisition_date DATE NOT NULL,
    acquisition_cost REAL NOT NULL,
    depreciation_method TEXT DEFAULT 'STRAIGHT_LINE' CHECK(depreciation_method IN ('STRAIGHT_LINE')),
    useful_life_years INTEGER NOT NULL,
    fiscal_useful_life_group INTEGER, -- Golongan aset fiskal (1, 2, 3, 4)
    salvage_value REAL DEFAULT 0,
    asset_account_id INTEGER NOT NULL,
    accumulated_depreciation_account_id INTEGER NOT NULL,
    depreciation_expense_account_id INTEGER NOT NULL,
    FOREIGN KEY (asset_account_id) REFERENCES accounts(id) ON DELETE RESTRICT,
    FOREIGN KEY (accumulated_depreciation_account_id) REFERENCES accounts(id) ON DELETE RESTRICT,
    FOREIGN KEY (depreciation_expense_account_id) REFERENCES accounts(id) ON DELETE RESTRICT
);

-- Tabel Log Penyusutan Aset: Menyimpan riwayat penyusutan per aset.
CREATE TABLE depreciation_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fixed_asset_id INTEGER NOT NULL,
    period_date DATE NOT NULL, -- Tanggal periode penyusutan (e.g., akhir bulan)
    amount REAL NOT NULL,
    journal_transaction_id INTEGER UNIQUE, -- FK ke jurnal umum yang mencatat beban penyusutan ini
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (fixed_asset_id) REFERENCES fixed_assets(id) ON DELETE CASCADE,
    FOREIGN KEY (journal_transaction_id) REFERENCES transactions(id) ON DELETE SET NULL
);


4. ⚙️ Rincian Fitur & Alur Kerja (Feature Breakdown)
4.1. Kerangka Utama Aplikasi (Shell)
•	Tujuan: Menyediakan navigasi utama yang konsisten di seluruh aplikasi.
•	Desain UI/UX:
- Sidebar kiri vertikal menggunakan latar brand-900, teks brand-50, dan ikon Lucide. Mode collapsible mempertahankan ikon saat sidebar dipersempit.
- Saat diciutkan, hanya ikon menu dan tombol expand yang terlihat.
- Pojok kiri atas menampilkan inisial aplikasi ("LB") dan nama perusahaan dari `app_settings`.
- Daftar menu utama (ikon + teks): Dashboard, Penjualan, Pembelian, Master Data, Bagan Akun, Jurnal Umum, Buku Besar, Laporan, Pengaturan.
- Quick links di bagian bawah: Pusat Bantuan, Shortcut Keyboard, Ganti Database, serta informasi versi aplikasi.
2.	Akun
3.	Data
4.	Penjualan
5.	Pembelian
6.	Transaksi
7.	Laporan
8.	Pengaturan
o	Pojok kiri bawah berisi tombol Ubah Database dan Keluar.

4.2. Modul: Dashboard
•	Tujuan: Memberikan gambaran visual kondisi keuangan perusahaan secara real-time.
•	Desain UI/UX:
o	Layout grid 2x3.
o	Card Utama (2x1): Arus Kas
	Grafik Garis (Line Chart) menampilkan data Kas Masuk vs Kas Keluar.
	Filter: Tahun Ini (menampilkan data per bulan), Bulan Ini (menampilkan data per hari).
o	Card 2: Faktur Penjualan
	Grafik Batang Horizontal.
	Menampilkan Total Nominal Penjualan dan Jumlah Faktur.
	Filter: Tahun Ini, Bulan Ini.
o	Card 3: Faktur Pembelian
	Mirip dengan Card 2, tetapi untuk data pembelian.
o	Card 4: Laba & Rugi
	Grafik Batang Vertikal.
	Filter Tahun Ini: Menampilkan 12 batang (Jan-Des).
	Filter Bulan Ini: Menampilkan ~30 batang (per hari).
o	Card 5: Pengeluaran Teratas
	Pie Chart yang menampilkan 5-6 kategori beban terbesar.
	Daftar teks di samping chart yang merinci nama akun beban dan nominalnya.
•	Logika Backend (Rust Commands):
o	get_cash_flow_summary(period)
o	get_sales_summary(period)
o	get_purchases_summary(period)
o	get_profit_loss_summary(period)
o	get_top_expenses(period, count)
4.3. Modul: Akun (Bagan Akun)
•	Tujuan: Mengelola daftar akun perusahaan dengan struktur hierarkis yang jelas.
•	Desain UI/UX:
o	Tampilan utama adalah tabel hierarkis (struktur pohon/tree).
o	Setiap baris induk (Level 1-3) memiliki tombol > untuk membuka/menutup (expand/collapse) anak-anaknya.
o	Tombol di kanan atas: Perluas Semua / Ciutkan Semua.
o	Tombol di kanan atas: + Akun Baru.
o	Form Akun Baru (Modal/Halaman):
	Nomor Akun: (Input, 6 digit, validasi unik).
	Nama Akun: (Input Teks).
	Akun Induk: (Dropdown, menampilkan akun non-postable/header).
	Saldo Awal: (Input Angka, opsional).
	Keterangan: (Textarea).
	Field level, type, normal_balance, is_postable akan ditentukan secara otomatis oleh logika backend berdasarkan Nomor Akun dan Akun Induk.
•	Logika Backend (Rust Commands):
o	get_accounts_tree(): Mengambil semua akun dan mengembalikannya dalam struktur bersarang (nested JSON) untuk mudah dirender oleh frontend.
o	create_account(data): Melakukan validasi dan menyimpan akun baru.
o	update_account(id, data)
o	delete_account(id): Hanya bisa dilakukan jika akun tidak memiliki transaksi terkait.
Sementara masukkan akun sbb sebagai template awal, ke depannya chart of account dapat diedit / ditambah (atau kembangkan jika kamu rasa bisa lebih baik):
100000 - ASET
•	110000 - Aset Lancar
o	110100 - Kas dan Setara Kas
	110101 - Kas Kecil (Petty Cash)
	110102 - Kas di Bank (Rupiah & Valas)
	110103 - Deposito Jangka Pendek (< 3 bulan)
	110104 - Penempatan pada Bank Indonesia & Bank Lain
o	110200 - Investasi Lancar & Surat Berharga
	110201 - Surat Berharga (Diperdagangkan)
	110202 - Reksadana Pasar Uang
	110203 - Tagihan Repo & Reverse Repo
	110204 - Tagihan Spot & Derivatif
o	110300 - Piutang Usaha & Berelasi
	110301 - Piutang Usaha - Pihak Ketiga
	110302 - Piutang Usaha - Pihak Berelasi
	110303 - Piutang Nasabah (Pihak Ketiga & Berelasi)
	110399 - Cadangan Kerugian Penurunan Nilai Piutang
o	110400 - Piutang Lain-lain
	110401 - Piutang Karyawan
	110402 - Piutang Bunga & Dividen
	110403 - Piutang Retensi
o	110500 - Piutang Spesifik Industri
	110501 - Piutang Pembiayaan Neto (Investasi, Modal Kerja, Multiguna)
	110502 - Piutang Sewa Beli & Ijarah
	110503 - Piutang Murabahah, Istishna, Qardh
	110504 - Tagihan Premi, Klaim Koasuransi & Reasuransi
o	110600 - Persediaan
	110601 - Persediaan Bahan Baku & Penolong
	110602 - Persediaan Barang Dalam Proses
	110603 - Persediaan Barang Jadi / Dagang
o	110700 - Biaya & Pajak Dibayar di Muka
	110701 - Sewa & Asuransi Dibayar di Muka
	110702 - Uang Muka Pembelian & Proyek
	110703 - Pajak Dibayar di Muka (PPh 22, 23, 25, PPN Masukan)
o	110800 - Aset Lancar Lainnya
	110801 - Aset Kontrak
	110802 - Aset yang Dimiliki untuk Dijual
•	120000 - Aset Tidak Lancar
o	120100 - Investasi Jangka Panjang
	120101 - Investasi pada Perusahaan Asosiasi, Ventura & Anak
	120102 - Properti Investasi
	120103 - Penyertaan Modal & Saham
	120104 - Investasi Jangka Panjang Lainnya
o	120200 - Aset Tetap - Tanah & Bangunan
	120201 - Tanah
	120202 - Bangunan
	120299 - Akumulasi Penyusutan - Bangunan
o	120300 - Aset Tetap - Mesin & Kendaraan
	120301 - Mesin & Peralatan Produksi
	120302 - Kendaraan
	120399 - Akumulasi Penyusutan - Mesin & Kendaraan
o	120400 - Aset Tetap - Lainnya
	120401 - Inventaris & Peralatan Kantor
	120402 - Aset Biologis
	120499 - Akumulasi Penyusutan - Aset Tetap Lainnya
o	120500 - Aset Tak Berwujud & Konsesi
	120501 - Goodwill, Merek Dagang, Hak Paten
	120502 - Lisensi & Perangkat Lunak
	120503 - Aset Hak Guna & Hak Konsesi
	120599 - Akumulasi Amortisasi & Penyusutan
o	120600 - Aset Tidak Lancar Lainnya
	120601 - Aset Pajak Tangguhan
	120602 - Klaim atas Pengembalian Pajak
	120603 - Beban Tangguhan
________________________________________
200000 - LIABILITAS
•	210000 - Liabilitas Jangka Pendek
o	210100 - Utang Usaha (Pihak Ketiga & Berelasi)
o	210200 - Beban Akrual (Gaji, Bunga, dll)
o	210300 - Pendapatan Diterima di Muka
o	210400 - Utang Pajak (PPh 21, 23, 29, PPN Keluaran)
o	210500 - Pinjaman Jangka Pendek (Bank & Lainnya)
o	210600 - Utang Dividen
o	210700 - Liabilitas Jangka Pendek Spesifik Industri
	210701 - Giro, Tabungan, Deposito Nasabah
	210702 - Utang Akseptasi, Repo, Kliring
	210703 - Utang Klaim, Komisi, Koasuransi & Reasuransi
o	210800 - Liabilitas Jangka Pendek Lainnya
	210801 - Liabilitas Kontrak
	210802 - Bagian Liabilitas Jangka Panjang Jatuh Tempo < 1 Tahun
•	220000 - Liabilitas Jangka Panjang
o	220100 - Pinjaman Jangka Panjang (Bank & Pihak Berelasi)
o	220200 - Utang Obligasi & Surat Berharga Diterbitkan
o	220300 - Pinjaman Subordinasi
o	220400 - Liabilitas Imbalan Kerja & Manfaat Pensiun
o	220500 - Liabilitas Sewa Jangka Panjang
o	220600 - Liabilitas Pajak Tangguhan
o	220700 - Cadangan Teknis (Asuransi)

300000 - EKUITAS
•	310100 - Modal Saham
•	310200 - Tambahan Modal Disetor (Agio Saham)
•	320100 - Laba Ditahan
•	320200 - Laba/Rugi Tahun Berjalan
•	330100 - Dividen
•	340100 - Pendapatan Komprehensif Lainnya
•	350100 - Ekuitas Lainnya

400000 - PENDAPATAN
•	410000 - Pendapatan Usaha/Operasional
o	410100 - Penjualan Produk & Jasa (Domestik & Ekspor)
o	410200 - Pendapatan Premi (Asuransi)
o	410300 - Pendapatan Bunga & Bagi Hasil (Finansial)
o	410400 - Pendapatan Sewa & Fee Manajemen
o	410500 - Pendapatan Jasa Perantara & Penjaminan Emisi (Sekuritas)
o	410900 - Retur & Potongan Penjualan (-)
•	420000 - Pendapatan di Luar Usaha/Non-Operasional
o	420100 - Pendapatan Bunga (Non-Finansial)
o	420200 - Pendapatan Dividen
o	420300 - Keuntungan Penjualan Aset
o	420400 - Keuntungan Selisih Kurs
o	429900 - Pendapatan Lain-lain

500000 - BEBAN POKOK PENJUALAN & BEBAN POKOK PENDAPATAN
•	510000 - Harga Pokok Penjualan (Dagang & Manufaktur)
o	510100 - Pembelian Bersih
o	510200 - Biaya Tenaga Kerja Langsung
o	510300 - Biaya Overhead Pabrik (BOP)
•	520000 - Beban Pokok Pendapatan (Jasa & Finansial)
o	520100 - Beban Bunga (Finansial)
o	520200 - Beban Klaim & Underwriting (Asuransi)
o	520300 - Bagi Hasil untuk Pemilik Dana (Syariah)

600000 - BEBAN OPERASIONAL
•	610000 - Beban Gaji & Kesejahteraan Karyawan
o	610100 - Gaji, Tunjangan, Bonus, THR, dsb.
o	610200 - Beban Imbalan Kerja & Pelatihan
•	620000 - Beban Pemasaran & Penjualan
o	620100 - Beban Iklan & Promosi
o	620200 - Beban Komisi Penjualan
o	620300 - Beban Entertainment
•	630000 - Beban Umum & Administrasi
o	630100 - Beban Sewa
o	630200 - Beban Utilitas (Listrik, Air, Telekomunikasi)
o	630300 - Beban Transportasi & Perjalanan Dinas
o	630400 - Beban Jasa Profesional
o	630500 - Beban Perbaikan & Pemeliharaan
o	630600 - Beban Royalti
o	630700 - Beban Interkoneksi & Kustodian
o	630800 - Beban Penyusutan & Amortisasi
o	639900 - Beban Administrasi Lainnya

700000 - PENDAPATAN & BEBAN LAIN-LAIN (NON-OPERASIONAL)
•	710100 - Beban Bunga (Non-Finansial)
•	710200 - Beban Administrasi Bank
•	710300 - Kerugian Penjualan Aset
•	710400 - Kerugian Selisih Kurs
•	710500 - Sumbangan
•	719900 - Beban Lain-lain

800000 - PAJAK PENGHASILAN
•	810100 - Beban Pajak Penghasilan - Kini
•	810200 - Manfaat/Beban Pajak Tangguhan

4.4. Modul: Data (Pusat Data Master)
•	Tujuan: Menjadi hub untuk melihat dan mengelola data master.
•	Desain UI/UX:
o	Halaman utama berisi kumpulan kartu navigasi (misal, grid 4x3) yang masing-masing mengarah ke halaman detail.
o	Kartu: Informasi Umum, Data Kas, Data Bank, Data Piutang, Data Utang, Data Aset Tetap, Data Inventori, Data Pelanggan, Data Pemasok.
o	Halaman Data Aset Tetap:
	Tabel daftar aset tetap dengan kolom: Nama Aset, Tanggal Perolehan, Harga Perolehan, Akumulasi Penyusutan, Nilai Buku.
	Tombol + Aset Baru.
	Form Aset Baru: Mencakup input untuk masa manfaat (akuntansi & fiskal) dan metode penyusutan.
4.5. Modul: Penjualan & Pembelian
- Desain UI/UX:
  o Kontainer tabel berupa surface-card bernuansa navy gelap dengan sudut rounded-4xl dan shadow-card.
  o Filter di atas tabel: pencarian (nomor faktur/nama relasi), filter rentang tanggal, serta dropdown status (Semua, Lunas, Dibayar Sebagian, Belum Dibayar, Jatuh Tempo).
  o Kolom tabel standar: Tanggal, Nomor Invoice, Relasi (Pelanggan/Pemasok), Tanggal Jatuh Tempo, Total, Sisa Tagihan, Status.
  o Status badge menggunakan warna konsisten: brand (Paid/Lunas), accent (Partial/Dibayar Sebagian), abu-abu (Unpaid/Draft), merah lembut (Overdue).
  o Kolom aksi (kanan): menu konteks (lihat detail, edit, cetak, catat pembayaran, hapus).
  o Implementasi saat ini: placeholder UI telah tersedia di halaman Penjualan & Pembelian; data nyata dan command backend akan dihubungkan pada sprint modul operasional.
o	Kolom Tabel (Contoh Penjualan): Tanggal, No. Invoice, Nama Pelanggan, Tanggal Jatuh Tempo, Total, Sisa Tagihan, Status (dengan badge warna).
o	Kolom Aksi (paling kanan): Tombol ... yang membuka dropdown: Lihat Detail, Edit, Cetak, Catat Pembayaran, Hapus.
o	Tombol + Buat Penjualan Baru di pojok kanan atas.
•	Logika Backend (Rust Commands):
o	create_invoice(invoice_data, items): Menyimpan ke database DAN otomatis membuat jurnal akuntansi (Debit Piutang, Kredit Pendapatan).
o	record_invoice_payment(invoice_id, payment_data): Mengupdate status invoice DAN otomatis membuat jurnal (Debit Kas/Bank, Kredit Piutang).
o	Logika serupa berlaku untuk modul Pembelian.
4.6. Modul: Transaksi (Jurnal Umum)
•	Tujuan: Menampilkan semua entri jurnal dari seluruh modul dan memungkinkan entri manual.
•	Desain UI/UX:
o	Tampilan tabel General Ledger. Kolom: Tanggal, No. Jurnal, Keterangan, Akun, Debit, Kredit.
o	Tombol + Transaksi Baru untuk membuka form jurnal manual.
o	Aksi per baris: Lihat Detail, Edit, Hapus.
4.7. Modul: Laporan
•	Tujuan: Menyajikan data keuangan dalam format laporan standar yang dapat dianalisis dan diekspor.
•	Desain UI/UX:
o	Halaman ini memiliki submenu/tab di bagian atas untuk memilih jenis laporan.
o	Filter Laporan (di atas setiap laporan):
	Rentang Tanggal / Per Tanggal.
	Opsi pembanding periode (misal, bandingkan dengan 3 bulan sebelumnya, seperti pada gambar referensi).
	Tombol Ekspor (pilihan: PDF, Excel) dan Cetak.
o	Submenu:
1.	Laporan Posisi Keuangan
2.	Laporan Laba Rugi
3.	Laporan Arus Kas (Metode Tidak Langsung)
4.	Laporan Perubahan Ekuitas
5.	Laporan Kinerja Keuangan: Halaman berisi kartu-kartu yang menampilkan rasio keuangan penting (misal, Current Ratio, Debt-to-Equity Ratio, ROA, ROE).
6.	Laporan Analisis Beban: Halaman berisi visualisasi (Pie/Bar Chart) untuk menganalisis komposisi beban operasional.
•	Logika Backend (Rust Commands):
o	Setiap laporan memiliki command Rust-nya sendiri (generate_balance_sheet(params), generate_income_statement(params), dll.).
o	Logika kalkulasi 100% dilakukan di Rust untuk menjamin akurasi dan performa.
o	Untuk ekspor, gunakan crate Rust seperti genpdf dan rust_xlsxwriter.
________________________________________
5. 📋 Aspek Non-Fungsional
•	Kinerja: Aplikasi harus responsif. Operasi database pada data besar harus menggunakan paginasi. Query harus dioptimalkan dengan INDEX.
•	Keamanan: Seluruh data tersimpan secara lokal di file database.sqlite. Tidak ada transmisi data keluar. Enkripsi database dapat dipertimbangkan sebagai fitur di masa depan.
•	Manajemen Data:
o	Migrasi Database: Wajib diimplementasikan sejak awal menggunakan rusqlite_migration.
o	Backup & Restore: Sediakan fitur di menu Pengaturan untuk membuat cadangan file database dan memulihkannya.
•	Pengujian:
o	Backend (Rust): Unit test untuk fungsi logika murni. Integration test untuk commands yang berinteraksi dengan database (menggunakan SQLite in-memory).
o	Frontend (Vue): Unit test untuk komponen dan stores Pinia menggunakan Vitest.
________________________________________
6. 🚀 Rencana Proyek & Milestones
Pengembangan akan dibagi menjadi beberapa fase yang jelas.
•	Fase 0: Pondasi & Setup (Sprint 1)
o	[x] Inisialisasi proyek Tauri + Vue.
o	[x] Setup struktur direktori, TailwindCSS, Pinia, Vue Router.
o	[x] Implementasi skema database awal dan sistem migrasi.
o	[x] Buat kerangka aplikasi utama (Sidebar, Layout).
•	Fase 1: Inti Akuntansi (MVP) (Sprint 2-3)
o	[ ] Modul Akun: CRUD dan tampilan hierarkis.
o	[ ] Seed bagan akun standar ke dalam database.
o	[ ] Modul Transaksi: Form entri jurnal manual dan tabel General Ledger.
o	[ ] Backend command untuk Laporan Posisi Keuangan dan Laba Rugi dasar.
•	Fase 2: Modul Operasional (Sprint 4-5)
o	[ ] Modul Data: CRUD untuk Pelanggan & Pemasok.
o	[ ] Modul Penjualan: Pembuatan invoice, pencatatan pembayaran, dan pembuatan jurnal otomatis.
o	[ ] Modul Pembelian: Fungsionalitas serupa untuk faktur pembelian.
•	Fase 3: Pelaporan & Analisis (Sprint 6-7)
o	[ ] Modul Dashboard: Implementasi semua kartu dan grafik.
o	[ ] Modul Laporan: Sempurnakan semua laporan keuangan (Arus Kas, Perubahan Ekuitas).
o	[ ] Implementasi fitur Ekspor (PDF/Excel) dan Cetak.
o	[ ] Buat halaman Laporan Kinerja Keuangan (Rasio) dan Analisis Beban.
•	Fase 4: Fitur Lanjutan & Penyempurnaan (Sprint 8)
o	[ ] Modul Data: Fungsionalitas Aset Tetap, termasuk perhitungan penyusutan.
o	[ ] Modul Pengaturan: Halaman untuk mengubah nama perusahaan, dll.
o	[ ] Implementasi fitur Backup & Restore database.
o	[ ] Tinjauan UX/UI secara keseluruhan, penambahan animasi transisi.
•	Fase 5: Pengujian & Rilis (Sprint 9)
o	[ ] Pengujian E2E (End-to-End).
o	[ ] Perbaikan bug intensif.
o	[ ] Optimasi performa.
o	[ ] Persiapan build aplikasi untuk platform target (Windows, macOS, Linux) dan rilis v1.0.0.
--------------------------------------------------------------------------------
Lampiran A.1 - Setup Proyek LokalBuku (Oktober 2025)
- Prasyarat: Node.js 18+, npm 10+, Rust 1.77+, cargo-tauri 2.8.4.
- Inisialisasi frontend: jalankan perintah `npm create vite@latest frontend -- --template vue-ts`, lalu pindahkan hasil scaffold ke direktori akar repositori.
- Integrasi Tauri: jalankan `cargo tauri init --ci --app-name LokalBuku --window-title LokalBuku --before-dev-command "npm run dev" --before-build-command "npm run build" --force`.
- Instalasi dependensi: `npm install` kemudian tambahkan paket pendukung (vue-router, pinia, tailwindcss, @tauri-apps/api, @tauri-apps/plugin-sql, lucide-vue-next, vue3-apexcharts, @headlessui/vue).
- Konfigurasi Tailwind: `npx tailwindcss init -p`, perbarui alias Vite, dan buat `src/assets/main.css`.
- Pengaturan plugin SQL: tambahkan `tauri-plugin-sql` v2.3 beserta konfigurasi permissions `sql:default` dan `sql:allow-execute`.
- Cara menjalankan pengembangan: `npm run tauri:dev` (menggabungkan Vite dev server dan proses Tauri).
- Cara build release: `npm run tauri:build`.
- Lokasi database: tersimpan otomatis di direktori AppConfig OS (contoh Windows: %APPDATA%/com.tauri.dev/data/lokalbuku.db).
- Catatan kompatibilitas: Tauri v2 dipilih karena versi plugin-sql terbaru tidak lagi mendukung Tauri v1.x.





