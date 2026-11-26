# Tekstur MCV 0.4 (semua sementara salinan `tallgrass.png`)
Folder: assets/textures/block/  (format PNG 16x16; nama file sama dengan Minecraft Java 1.21.5+)

| File | Blok | Tint biome |
|---|---|---|
| short_grass.png | short_grass (rumput pendek) | rumput |
| tall_grass_bottom.png / tall_grass_top.png | rumput tinggi (2 blok) | rumput |
| fern.png | fern | rumput |
| large_fern_bottom.png / large_fern_top.png | large fern (2 blok) | rumput |
| bush.png | bush | daun (foliage) |
| sweet_berry_bush_stage0.png ... stage3.png | sweet_berry_bush, age 0-3 | tidak ada |
| dead_bush.png | dead_bush | tidak ada |
| short_dry_grass.png | short_dry_grass | rumput |
| tall_dry_grass.png | tall_dry_grass | rumput |

Dandelion dan rose adalah blok bawaan Weft, jadi teksturnya lewat resource pack-mu.
Tekstur dengan tint harus abu-abu/putih (warna datang dari biome). Tanpa tint: warna asli.

## Mengubah TGA ke PNG sekaligus
Weft hanya membaca PNG. Di folder `tools/`:
1. Install Python dan Pillow: `pip install pillow`
2. Salin semua .tga ke satu folder kerja, lalu jalankan:
   `python tools/convert_tga.py "C:\folder\tga" --mcv`
   (`--mcv` mengganti nama Bedrock: double_plant_grass_bottom -> tall_grass_bottom, double_plant_fern_top -> large_fern_top, tallgrass -> short_grass, deadbush -> dead_bush)
3. Salin hasil .png yang namanya ada di tabel ke `assets/textures/block/`.
Atau seret folder ke `tools/convert_tga.bat`.

## 0.5.0
- `short_grass.png` dipakai untuk blok rumput bawaan Weft (`base:tall_grass`), bukan blok baru. Tint: grass.
- `textures/item/short_grass.png` = ikon item creative rumput pendek (boleh sama dengan block).
- Tint: bush, fern, large_fern, tall/short grass = GRASS. sweet_berry_bush, dead_bush, short/tall_dry_grass = tanpa tint (warna dari texturmu).

## Chest
- `textures/block/chest.png` (64x64, tata letak chest Bedrock) + `models/block/chest.json`.
- Animasi tutup: belum bisa di 26.11.3; ada di Weft master (`animated_part`).
