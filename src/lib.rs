//! Minecraft Core Visuals (MCV) 0.2: ALAM, untuk Weft 26.11.3.
//!
//! Yang dilakukan mod ini:
//!  1. Mematikan penyebaran rumput acak bawaan base (GRASS_COVER = 0 di semua biome berumput).
//!     Bunga, kaktus, dan tebu bawaan tetap jalan.
//!  2. Menanam ulang SEMUA tumbuhan lewat satu dekorasi, dalam bercak alami (noise), bukan acak:
//!       - rumput pendek  = blok `base:tall_grass` bawaan Weft (tampilannya memang rumput pendek)
//!       - rumput tinggi  = `mcv:tall_grass_bottom` + `mcv:tall_grass_top` (2 blok)
//!       - fern           = `mcv:fern`
//!       - large fern     = `mcv:large_fern_bottom` + `mcv:large_fern_top` (2 blok)
//!       - bush           = `mcv:bush`
//!
//! Cara mengatur: ubah angka di `profile()` (kepadatan per biome) dan `PATCH` (ukuran bercak).
//! Cara menambah jenis tumbuhan: salin satu blok di `init`, tambahkan di `plant_column`.
//! Tekstur: ganti PNG di `assets/textures/block/` (sementara semuanya salinan `tallgrass`).

use weft_sdk::noise::*;
use weft_sdk::*;

declare_mod!(deps: ["base"], init: init);

// ---------------------------------------------------------------- blok ---------------------------

pub static TALL_GRASS_BOTTOM: Block = Block::named("tall_grass_bottom");
pub static TALL_GRASS_TOP: Block = Block::named("tall_grass_top");
pub static FERN: Block = Block::named("fern");
pub static LARGE_FERN_BOTTOM: Block = Block::named("large_fern_bottom");
pub static LARGE_FERN_TOP: Block = Block::named("large_fern_top");
pub static BUSH: Block = Block::named("bush");

/// Rumput pendek milik base (blok satu petak yang dulu disebar acak).
static SHORT_GRASS: Block = Block::named("base:tall_grass");

// Milik base. Tag di Weft bersifat global (tanpa "base:"), komponen pakai nama lengkap.
static PLANT_SOIL: Tag = Tag::named("plant_soil");
static FROZEN: Tag = Tag::named("frozen");
static DRY: Tag = Tag::named("dry");
static FOREST: Tag = Tag::named("forest");
static COLD: Tag = Tag::named("cold");
static MOUNTAIN: Tag = Tag::named("mountain");
static GRASS_COVER: Component<f32> = Component::named("base:grass");

const SEA: i32 = 63;

fn init(r: &mut Registry) {
    // 1. Matikan rumput acak bawaan base.
    for biome in ["base:plains", "base:forest", "base:birch_forest", "base:taiga", "base:mountains"] {
        let handle: &'static Biome = Box::leak(Box::new(Biome::named(biome)));
        r.edit(handle).set(&GRASS_COVER, 0.0);
    }

    // 2. Blok baru. `plant()` = model silang tanpa tabrakan; `replaceable()` = bisa ditimpa blok.
    r.block(&TALL_GRASS_BOTTOM)
        .sound_group("base:block.grass")
        .texture("tall_grass_bottom")
        .plant()
        .replaceable()
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .no_item();
    r.block(&TALL_GRASS_TOP)
        .sound_group("base:block.grass")
        .texture("tall_grass_top")
        .plant()
        .replaceable()
        .needs_below("mcv:tall_grass_bottom")
        .tint(&tints::GRASS)
        .no_item();

    r.block(&FERN)
        .sound_group("base:block.grass")
        .texture("fern")
        .plant()
        .replaceable()
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .no_item();
    r.block(&LARGE_FERN_BOTTOM)
        .sound_group("base:block.grass")
        .texture("large_fern_bottom")
        .plant()
        .replaceable()
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .no_item();
    r.block(&LARGE_FERN_TOP)
        .sound_group("base:block.grass")
        .texture("large_fern_top")
        .plant()
        .replaceable()
        .needs_below("mcv:large_fern_bottom")
        .tint(&tints::GRASS)
        .no_item();

    // Bush: lebih besar dari rumput, berwarna daun (foliage).
    r.block(&BUSH)
        .sound_group("base:block.grass")
        .texture("bush")
        .plant()
        .hitbox([2.0, 0.0, 2.0], [14.0, 13.0, 14.0])
        .needs_below("#plant_soil")
        .tint(&tints::FOLIAGE)
        .no_item();

    // Biji gandum dari rumput tinggi, seperti di base.
    for b in [&TALL_GRASS_BOTTOM, &TALL_GRASS_TOP] {
        r.loot(b).pool(Pool::new().when(Cond::chance(0.15)).item("base:wheat_seeds"));
    }

    // 3. Penanaman.
    r.decoration(plant_world);
}

// ---------------------------------------------------------- pengaturan alam ---------------------

/// Seberapa padat tiap jenis di sebuah biome. Semua angka 0.0 - 1.0:
/// peluang MAKSIMAL per petak di bagian terpadat sebuah bercak (di luar bercak makin jarang).
#[derive(Clone, Copy)]
struct Profile {
    grass: f64,      // rumput pendek
    tall: f64,       // rumput tinggi (hanya di inti bercak rumput)
    fern: f64,       // fern
    large_fern: f64, // large fern (hanya di inti bercak fern)
    bush: f64,       // bush (sangat jarang)
}

const NONE: Profile = Profile { grass: 0.0, tall: 0.0, fern: 0.0, large_fern: 0.0, bush: 0.0 };

/// Biome ditentukan dari TAG-nya, jadi biome dari mod lain ikut kebagian.
fn profile(b: u32) -> Profile {
    if FROZEN.on_biome(b) || DRY.on_biome(b) {
        return NONE; // salju dan gurun: tidak ada tumbuhan ini
    }
    if FOREST.on_biome(b) && COLD.on_biome(b) {
        // taiga: fern dominan
        return Profile { grass: 0.18, tall: 0.00, fern: 0.55, large_fern: 0.25, bush: 0.05 };
    }
    if FOREST.on_biome(b) {
        // hutan biasa dan birch: rumput sedang, fern, bush
        return Profile { grass: 0.35, tall: 0.12, fern: 0.22, large_fern: 0.06, bush: 0.12 };
    }
    if MOUNTAIN.on_biome(b) {
        return Profile { grass: 0.22, tall: 0.00, fern: 0.10, large_fern: 0.00, bush: 0.03 };
    }
    // dataran (plains) dan lainnya: padang rumput
    Profile { grass: 0.70, tall: 0.45, fern: 0.02, large_fern: 0.00, bush: 0.05 }
}

/// Ukuran bercak dalam blok (makin besar = bercak makin lebar) dan batas munculnya.
struct Patch {
    scale: f64,
    from: f64, // di bawah nilai ini: tidak ada sama sekali
    to: f64,   // di atas nilai ini: kepadatan penuh
}
const PATCH_GRASS: Patch = Patch { scale: 28.0, from: 0.30, to: 0.65 };
const PATCH_TALL: Patch = Patch { scale: 16.0, from: 0.50, to: 0.70 };
const PATCH_FERN: Patch = Patch { scale: 22.0, from: 0.35, to: 0.70 };
const PATCH_LARGE_FERN: Patch = Patch { scale: 14.0, from: 0.50, to: 0.72 };
const PATCH_BUSH: Patch = Patch { scale: 45.0, from: 0.55, to: 0.75 };
/// Sedikit rumput di mana-mana supaya padang tidak gundul (bagian dari kepadatan maksimal).
const GRASS_BASELINE: f64 = 0.12;
/// Bush dibuat jauh lebih jarang daripada angka profilnya.
const BUSH_RARITY: f64 = 0.15;

// ------------------------------------------------------------------ noise ----------------------

fn smooth(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

/// Noise nilai 2D halus, 0.0 - 1.0, bergantung pada koordinat dunia (tidak ada garis di batas cube).
fn noise2(seed: u64, wx: i32, wz: i32, scale: f64) -> f64 {
    let (fx, fz) = (wx as f64 / scale, wz as f64 / scale);
    let (ix, iz) = (fx.floor(), fz.floor());
    let (tx, tz) = (smooth(fx - ix), smooth(fz - iz));
    let (ix, iz) = (ix as i64, iz as i64);
    let at = |dx: i64, dz: i64| unit(hash(seed, ix + dx, 7, iz + dz));
    let top = at(0, 0) + (at(1, 0) - at(0, 0)) * tx;
    let bot = at(0, 1) + (at(1, 1) - at(0, 1)) * tx;
    top + (bot - top) * tz
}

/// Dua lapis noise: bentuk bercak lebih alami daripada satu lapis.
fn field(seed: u64, salt: u64, wx: i32, wz: i32, p: &Patch) -> f64 {
    let s = seed ^ salt;
    0.65 * noise2(s, wx, wz, p.scale) + 0.35 * noise2(s ^ 0x9E37, wx, wz, p.scale / 2.3)
}

/// 0.0 di bawah `from`, 1.0 di atas `to`, halus di antaranya.
fn strength(v: f64, p: &Patch) -> f64 {
    smooth(((v - p.from) / (p.to - p.from)).clamp(0.0, 1.0))
}

// ------------------------------------------------------------ penanaman --------------------------

enum Plant {
    Short,
    Tall,
    Fern,
    LargeFern,
    Bush,
}

/// Memilih tumbuhan untuk satu kolom (atau tidak ada). Urutan = prioritas.
fn choose(seed: u64, wx: i32, wz: i32, prof: Profile) -> Option<Plant> {
    let roll = |salt: i64| unit(hash(seed ^ 0x3C5A_17E5, wx as i64, salt, wz as i64));
    let grass_f = field(seed, 0xA1, wx, wz, &PATCH_GRASS);
    let grass_s = strength(grass_f, &PATCH_GRASS);

    // Rumput tinggi: inti bercak rumput.
    if prof.tall > 0.0 && grass_s > 0.3 {
        let s = strength(field(seed, 0xB2, wx, wz, &PATCH_TALL), &PATCH_TALL);
        if roll(1) < prof.tall * s {
            return Some(Plant::Tall);
        }
    }
    // Large fern: inti bercak fern.
    let fern_s = strength(field(seed, 0xC3, wx, wz, &PATCH_FERN), &PATCH_FERN);
    if prof.large_fern > 0.0 {
        let s = strength(field(seed, 0xD4, wx, wz, &PATCH_LARGE_FERN), &PATCH_LARGE_FERN) * fern_s;
        if roll(2) < prof.large_fern * s {
            return Some(Plant::LargeFern);
        }
    }
    // Bush: bercak sangat luas dan jarang.
    if prof.bush > 0.0 {
        let s = strength(field(seed, 0xE5, wx, wz, &PATCH_BUSH), &PATCH_BUSH);
        if roll(3) < prof.bush * BUSH_RARITY * s {
            return Some(Plant::Bush);
        }
    }
    if prof.fern > 0.0 && roll(4) < prof.fern * fern_s {
        return Some(Plant::Fern);
    }
    if prof.grass > 0.0 && roll(5) < prof.grass * (GRASS_BASELINE + (1.0 - GRASS_BASELINE) * grass_s) {
        return Some(Plant::Short);
    }
    None
}

/// Dekorasi: jalan pada tiap cube yang dibuat, murni dari seed + koordinat (syarat worldgen Weft).
fn plant_world(g: &mut Gen) {
    let Some(&cols) = g.columns() else { return };
    let seed = g.seed();
    let BlockPos { x: ox, y: oy, z: oz } = g.origin();
    let max_h = cols.iter().map(|c| c.height).max().unwrap_or(0);
    let min_h = cols.iter().map(|c| c.height).min().unwrap_or(0);
    if oy > max_h + 16 || oy + 15 < min_h - 1 {
        return;
    }
    for z in 0..16usize {
        for x in 0..16usize {
            let col = &cols[z * 16 + x];
            let y = col.height + 1 - oy;
            if !(0..16).contains(&y) || col.height < SEA {
                continue;
            }
            let y = y as usize;
            let prof = profile(col.biome);
            if prof.grass + prof.fern + prof.bush == 0.0 {
                continue;
            }
            let (wx, wz) = (ox + x as i32, oz + z as i32);
            let Some(plant) = choose(seed, wx, wz, prof) else { continue };
            if g.get(x, y, z) != 0 {
                continue; // sudah ada isinya (bunga, pohon, ...)
            }
            let below = if y == 0 { g.surface_block(wx, wz) } else { g.get(x, y - 1, z) };
            if !PLANT_SOIL.on_block(below) {
                continue;
            }
            // Dua blok butuh ruang di atas, di dalam cube yang sama.
            let room = y + 1 < 16 && g.get(x, y + 1, z) == 0;
            match plant {
                Plant::Tall if room => {
                    g.set(x, y, z, TALL_GRASS_BOTTOM.id());
                    g.set(x, y + 1, z, TALL_GRASS_TOP.id());
                }
                Plant::LargeFern if room => {
                    g.set(x, y, z, LARGE_FERN_BOTTOM.id());
                    g.set(x, y + 1, z, LARGE_FERN_TOP.id());
                }
                Plant::Tall | Plant::Short => g.set(x, y, z, SHORT_GRASS.id()),
                Plant::LargeFern | Plant::Fern => g.set(x, y, z, FERN.id()),
                Plant::Bush => g.set(x, y, z, BUSH.id()),
            }
        }
    }
}
