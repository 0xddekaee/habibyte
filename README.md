# Habibyte Core Blockchain

[![Language](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-lightgrey.svg?style=flat-square)](https://docs.rs)
[![Architecture](https://img.shields.io/badge/Architecture-Modular-blue.svg?style=flat-square)](https://github.com/habibyte)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)
[![Status](https://img.shields.io/badge/Phase-1%20Complete-success.svg?style=flat-square)](roadmap.txt)

**Habibyte** adalah infrastruktur blockchain berdaulat yang dirancang untuk menjadi fondasi **Ledger Identitas Nasional** generasi baru. Dibangun dengan arsitektur modular berbasis **Rust**, sistem ini memprioritaskan keamanan data mutlak (*Zero Data Leaks*), validitas identitas tunggal (*Zero Duplicate*), dan performa tinggi untuk skala negara.

Sistem ini menerapkan pendekatan **Privacy-First Hybrid Architecture**: data sensitif warga (seperti rekam medis atau biometrik) dienkripsi menggunakan standar militer (AES-GCM) dan disimpan secara *off-chain* (IPFS/Private Storage), sementara *blockchain* hanya menyimpan bukti kriptografis (Proof) dan Hash yang tidak dapat dipalsukan.

---

## 🏛️ Fitur Utama

-   **Zero Duplicate Identity Engine**
    Mencegah pendaftaran ganda menggunakan algoritma hashing canggih pada NIK, memastikan satu individu hanya memiliki satu identitas digital yang valid di seluruh ekosistem layanan negara (BPJS, Dukcapil, Rumah Sakit).

-   **Modular & Scalable Architecture**
    Dibangun dengan prinsip *Clean Architecture*, memisahkan logika P2P, Konsensus, Ledger, dan API ke dalam modul terisolasi. Ini memungkinkan pembaharuan sistem tanpa mematikan jaringan (*seamless upgrade*).

-   **Secure Off-Chain Storage**
    Integrasi penyimpanan data hibrida. Data rahasia tetap di tangan pemilik (User-Centric) atau instansi berwenang, terenkripsi penuh, tidak pernah diekspos secara telanjang di ledger publik.

-   **Standardized Interoperability**
    Dilengkapi dengan API Gateway (REST/RPC) yang siap diintegrasikan dengan sistem *legacy* pemerintahan maupun aplikasi modern.

---

## 🏗️ Arsitektur Sistem

Proyek ini dikelola sebagai *Rust Workspace* dengan pembagian modul (Crates) yang tegas:

| Modul | Fungsi & Tanggung Jawab |
| :--- | :--- |
| **`habibyte-node`** | **Orchestrator**. Binary utama yang menjalankan node, mengelola *lifecycle* service, dan menangani *graceful shutdown*. |
| **`habibyte-p2p`** | **Networking Layer**. Menangani komunikasi *peer-to-peer* terdesentralisasi menggunakan stack `libp2p` (Gossipsub, mDNS, Noise). |
| **`habibyte-ledger`** | **Core Logic**. Menyimpan struktur data Blok, Transaksi, dan memvalidasi integritas rantai (*Immutable Ledger*). |
| **`habibyte-identity`** | **Privacy Engine**. Mengatur logika identitas, *hashing* data pribadi, dan verifikasi kepemilikan. |
| **`habibyte-storage`** | **Data Persistence**. Interface untuk enkripsi/dekripsi AES-GCM dan konektor ke penyimpanan eksternal (IPFS/Disk). |
| **`habibyte-consensus`** | **Agreement Protocol**. Logika validasi blok dan aturan konsensus (saat ini *Authority-based*). |
| **`habibyte-api`** | **Gateway**. Menyediakan akses HTTP untuk aplikasi eksternal berinteraksi dengan blockchain. |

---

## 🚀 Panduan Memulai

### Prasyarat Sistem
Pastikan lingkungan pengembangan Anda telah siap:
-   **Rust Toolchain** (Stable terbaru)
-   **Build Essentials** (GCC/G++ untuk kompilasi ketergantungan kriptografi)

```bash
# Update sistem dan install compiler
sudo apt update && sudo apt install build-essential -y
```

### Kompilasi & Eksekusi

Jalankan perintah berikut untuk membangun seluruh infrastruktur dari kode sumber:

```bash
# Masuk ke direktori core
cd core

# Jalankan Node Utama (Default Port)
cargo run -p habibyte-node
```

### Konfigurasi Lanjutan (CLI)

Node dapat dikonfigurasi secara dinamis saat *runtime* untuk kebutuhan validator atau *bootnode*:

```bash
# Menjalankan node pada Port P2P 7000 dan API 9090
cargo run -p habibyte-node -- --p2p-port 7000 --api-port 9090
```

---

## 🗺️ Peta Jalan (Roadmap)

Pengembangan Habibyte dibagi menjadi fase-fase strategis untuk menjamin stabilitas produksi. Saat ini kami telah menyelesaikan **Fase 1**.

Untuk detail pencapaian dan rencana teknis selanjutnya, silakan baca file [**roadmap.txt**](./roadmap.txt).

---

*Dikembangkan untuk kemandirian teknologi bangsa.*
