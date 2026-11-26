# MCV untuk Weft 26.11.3
Mod rumput: short_grass, rumput tinggi 2 blok, dan kepadatan rumput lebih banyak.

Cara cepat tanpa install Rust: upload isi folder ini ke repo GitHub barumu, lalu buat tag `v0.1.0` (Releases > Draft new release > tag v0.1.0).
GitHub Actions akan membuat `mcv.zip`. Taruh `mcv.zip` itu (jangan di-extract) di `%APPDATA%\Weft\mods\`, restart game.

Cara lokal: install Rust (rustup.rs), jalankan `cargo build --release`, buat folder `%APPDATA%\Weft\mods\mcv\`, isi dengan `target\release\mcv.dll` dan folder `assets`.
Nama folder atau zip HARUS `mcv`.
