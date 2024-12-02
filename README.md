# Aplikasi Persewaan Alat Camping

Aplikasi ini memungkinkan admin dan penyewa untuk mengelola alat camping.

## Fitur
- **Admin:** Tambah, edit, hapus alat camping, lihat transaksi.
- **Penyewa:** Lihat daftar alat camping, sewa alat camping.

## Menjalankan Aplikasi

### Backend
1. Masuk ke direktori `backend/`.
2. Bangun dan jalankan backend:
    ```bash
    cargo build --release
    cargo run
    ```

### Frontend
1. Masuk ke direktori `frontend/`.
2. Bangun frontend dengan WebAssembly:
    ```bash
    wasm-pack build --target web
    ```
3. Jalankan server lokal untuk mengakses aplikasi:
    ```bash
    python3 -m http.server 8080
    ```

Akses aplikasi di `http://localhost:8080`.
