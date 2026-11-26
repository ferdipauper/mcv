//! Minecraft Core Visuals (MCV) 0.4: ALAM, untuk Weft 26.11.3.
//! Nama blok dan nama file tekstur mengikuti Minecraft Java 1.21.5+.
//!
//! Isi:
//!  1. Mematikan penyebaran rumput acak bawaan base (GRASS_COVER = 0). Bunga, kaktus, tebu bawaan tetap jalan.
//!  2. Menanam ulang tumbuhan lewat SATU dekorasi, dalam bercak noise, dengan profil per biome
//!     yang meniru Minecraft 1.21.5:
//!       plains        : rumput pendek lebat, bercak rumput tinggi, bercak bunga, bush kecil
//!       forest        : rumput sedang, bush, sedikit fern, bunga jarang
//!       birch forest  : rumput lebih lebat, bush, sedikit rumput tinggi
//!       taiga         : rumput + fern bercampur, large fern, rumput tinggi, sweet berry bush (jarang)
//!       mountains     : rumput tipis, fern, bush
//!       desert        : short_dry_grass, tall_dry_grass, dead_bush
//!       snowy/ocean   : tidak ada
//!  3. Semua blok punya item di creative inventory (tab Natural).
//!
//! Mengatur: ubah angka di `profile()` (kepadatan) dan `PATCH_*` (ukuran bercak).
//! Menambah jenis tumbuhan: salin satu blok di `init`, lalu tambahkan di `choose` / `plant_world`.
//! Tekstur: ganti PNG di `assets/textures/block/` (lihat TEXTURES.md).
//! Catatan: Weft 26.11.3 belum punya state `half`, jadi tanaman 2 blok = dua blok `_bottom` dan `_top`.

use weft_sdk::noise::*;
use weft_sdk::*;

declare_mod!(deps: ["base"], init: init);

// ---------------------------------------------------------------- blok ---------------------------

pub static SHORT_GRASS: Block = Block::named("short_grass");
pub static TALL_GRASS_BOTTOM: Block = Block::named("tall_grass_bottom");
pub static TALL_GRASS_TOP: Block = Block::named("tall_grass_top");
pub static FERN: Block = Block::named("fern");
pub static LARGE_FERN_BOTTOM: Block = Block::named("large_fern_bottom");
pub static LARGE_FERN_TOP: Block = Block::named("large_fern_top");
pub static BUSH: Block = Block::named("bush");
pub static SWEET_BERRY_BUSH: Block = Block::named("sweet_berry_bush");
pub static DEAD_BUSH: Block = Block::named("dead_bush");
pub static SHORT_DRY_GRASS: Block = Block::named("short_dry_grass");
pub static TALL_DRY_GRASS: Block = Block::named("tall_dry_grass");

// Milik base.
static DANDELION: Block = Block::named("base:dandelion");
static ROSE: Block = Block::named("base:rose");
static NATURAL: CreativeTab = CreativeTab::named("base:natural");

static PLAINS: Biome = Biome::named("base:plains");
static FOREST_BIOME: Biome = Biome::named("base:forest");
static BIRCH_FOREST: Biome = Biome::named("base:birch_forest");
static TAIGA: Biome = Biome::named("base:taiga");
static MOUNTAINS: Biome = Biome::named("base:mountains");

// Tag di Weft bersifat global (tanpa "base:"), komponen pakai nama lengkap.
static PLANT_SOIL: Tag = Tag::named("plant_soil");
static FROZEN: Tag = Tag::named("frozen");
static DRY: Tag = Tag::named("dry");
static FOREST: Tag = Tag::named("forest");
static COLD: Tag = Tag::named("cold");
static MOUNTAIN: Tag = Tag::named("mountain");
/// Tag buatan mod ini: tanah untuk tumbuhan gurun (pasir + tanah biasa). Dibuat di `init`.
static DRY_PLANT_SOIL: Tag = Tag::named("dry_plant_soil");
static GRASS_COVER: Component<f32> = Component::named("base:grass");

const SEA: i32 = 63;

/// Rumput yang menjatuhkan biji gandum (seperti di Minecraft).
fn seeds(r: &mut Registry, b: &'static Block, chance: f32) {
    r.loot(b).pool(Pool::new().when(Cond::chance(chance)).item("base:wheat_seeds"));
}

fn init(r: &mut Registry) {
    // 1. Matikan rumput acak bawaan base.
    r.edit(&PLAINS).set(&GRASS_COVER, 0.0);
    r.edit(&FOREST_BIOME).set(&GRASS_COVER, 0.0);
    r.edit(&BIRCH_FOREST).set(&GRASS_COVER, 0.0);
    r.edit(&TAIGA).set(&GRASS_COVER, 0.0);
    r.edit(&MOUNTAINS).set(&GRASS_COVER, 0.0);

    // Tanah tumbuhan gurun: pasir (cactus_soil) dan tanah berumput biasa.
    r.tag("dry_plant_soil", &["#cactus_soil", "#plant_soil"]);

    // 2. Blok. `plant()` = model silang tanpa tabrakan; `replaceable()` = bisa ditimpa blok.
    //    Tanpa `.no_item()` berarti ada item di creative; `.tab(&NATURAL)` menaruhnya di tab Natural.
    r.block(&SHORT_GRASS)
        .sound_group("base:block.grass")
        .texture("short_grass")
        .plant()
        .replaceable()
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .tab(&NATURAL);
    seeds(r, &SHORT_GRASS, 0.125);

    // Rumput tinggi: item = bagian bawah; saat diletakkan, bagian atas ikut muncul.
    r.block(&TALL_GRASS_BOTTOM)
        .sound_group("base:block.grass")
        .texture("tall_grass_bottom")
        .plant()
        .replaceable()
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .on_place(|w, c| place_top(w, c, &TALL_GRASS_TOP))
        .tab(&NATURAL);
    r.block(&TALL_GRASS_TOP)
        .sound_group("base:block.grass")
        .texture("tall_grass_top")
        .plant()
        .replaceable()
        .needs_below("mcv:tall_grass_bottom")
        .tint(&tints::GRASS)
        .no_item();
    seeds(r, &TALL_GRASS_BOTTOM, 0.125);

    r.block(&FERN)
        .sound_group("base:block.grass")
        .texture("fern")
        .plant()
        .replaceable()
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .tab(&NATURAL);
    seeds(r, &FERN, 0.125);

    r.block(&LARGE_FERN_BOTTOM)
        .sound_group("base:block.grass")
        .texture("large_fern_bottom")
        .plant()
        .replaceable()
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .on_place(|w, c| place_top(w, c, &LARGE_FERN_TOP))
        .tab(&NATURAL);
    r.block(&LARGE_FERN_TOP)
        .sound_group("base:block.grass")
        .texture("large_fern_top")
        .plant()
        .replaceable()
        .needs_below("mcv:large_fern_bottom")
        .tint(&tints::GRASS)
        .no_item();
    seeds(r, &LARGE_FERN_BOTTOM, 0.125);

    // Bush (1.21.5): lebih besar dari rumput, warna daun (foliage). Menjatuhkan dirinya sendiri.
    r.block(&BUSH)
        .sound_group("base:block.grass")
        .texture("bush")
        .plant()
        .hitbox([2.0, 0.0, 2.0], [14.0, 13.0, 14.0])
        .needs_below("#plant_soil")
        .tint(&tints::FOLIAGE)
        .tab(&NATURAL);

    // Sweet berry bush: 4 tahap (age 0-3), tekstur sweet_berry_bush_stage0..3 seperti Minecraft.
    r.block(&SWEET_BERRY_BUSH)
        .sound_group("base:block.grass")
        .prop("age", 0..=3)
        .states(|p, s| s.texture(&format!("sweet_berry_bush_stage{}", p.get("age"))))
        .plant()
        .hitbox([1.0, 0.0, 1.0], [15.0, 14.0, 15.0])
        .needs_below("#plant_soil")
        .tab(&NATURAL);

    // Tumbuhan gurun / kering (1.21.5): dead_bush, short_dry_grass, tall_dry_grass.
    r.block(&DEAD_BUSH)
        .sound_group("base:block.grass")
        .texture("dead_bush")
        .plant()
        .replaceable()
        .needs_below("#dry_plant_soil")
        .tab(&NATURAL);
    r.loot(&DEAD_BUSH).pool(Pool::new().entry(Entry::item("base:stick").count(0, 2)));
    r.block(&SHORT_DRY_GRASS)
        .sound_group("base:block.grass")
        .texture("short_dry_grass")
        .plant()
        .replaceable()
        .needs_below("#dry_plant_soil")
        .tint(&tints::GRASS)
        .drops_nothing()
        .tab(&NATURAL);
    r.block(&TALL_DRY_GRASS)
        .sound_group("base:block.grass")
        .texture("tall_dry_grass")
        .plant()
        .replaceable()
        .hitbox([2.0, 0.0, 2.0], [14.0, 14.0, 14.0])
        .needs_below("#dry_plant_soil")
        .tint(&tints::GRASS)
        .drops_nothing()
        .tab(&NATURAL);

    // 3. Penanaman.
    r.decoration(plant_world);
}

/// Bagian atas tanaman 2 blok ikut diletakkan bila ruangnya kosong.
fn place_top(w: &World, c: &BlockChange, top: &Block) {
    let above = [c.pos.x, c.pos.y + 1, c.pos.z];
    if w.block(above) == 0 {
        w.set_block(above, top.id());
    }
}

// ---------------------------------------------------------- pengaturan alam ---------------------

/// Seberapa padat tiap jenis di sebuah biome. Semua angka 0.0 - 1.0:
/// peluang MAKSIMAL per petak di bagian terpadat sebuah bercak (di luar bercak makin jarang).
#[derive(Clone, Copy)]
struct Profile {
    grass: f64,      // short_grass
    tall: f64,       // tall_grass (inti bercak rumput)
    fern: f64,       // fern
    large_fern: f64, // large_fern (inti bercak fern)
    bush: f64,       // bush (jarang, bercak kecil)
    berry: f64,      // sweet_berry_bush (sangat jarang)
    flowers: f64,    // bercak bunga (dandelion / rose)
    dry_short: f64,  // short_dry_grass (gurun)
    dry_tall: f64,   // tall_dry_grass (inti bercak di gurun)
    dead_bush: f64,  // dead_bush (jarang, merata)
}

const NONE: Profile = Profile {
    grass: 0.0, tall: 0.0, fern: 0.0, large_fern: 0.0, bush: 0.0,
    berry: 0.0, flowers: 0.0, dry_short: 0.0, dry_tall: 0.0, dead_bush: 0.0,
};

/// Profil per biome, meniru Minecraft 1.21.5. Biome yang tidak dikenal memakai tag-nya.
fn profile(b: u32) -> Profile {
    if PLAINS.id() == b {
        // Plains: rumput lebat, bercak rumput tinggi, bercak bunga, bush kecil.
        return Profile { grass: 0.70, tall: 0.45, bush: 0.08, flowers: 0.55, ..NONE };
    }
    if BIRCH_FOREST.id() == b {
        // Birch forest: rumput lebih lebat dari forest, bush, sedikit rumput tinggi.
        return Profile { grass: 0.50, tall: 0.10, fern: 0.04, bush: 0.12, flowers: 0.10, ..NONE };
    }
    if FOREST_BIOME.id() == b {
        // Forest: rumput sedang, bush, sedikit fern, bunga jarang.
        return Profile { grass: 0.38, tall: 0.06, fern: 0.08, bush: 0.12, flowers: 0.12, ..NONE };
    }
    if TAIGA.id() == b {
        // Taiga: rumput dan fern bercampur, large fern, rumput tinggi, berry bush jarang.
        return Profile { grass: 0.30, tall: 0.10, fern: 0.50, large_fern: 0.25, berry: 0.30, ..NONE };
    }
    if MOUNTAINS.id() == b {
        // Windswept hills: rumput tipis, fern campur, bush.
        return Profile { grass: 0.22, fern: 0.10, bush: 0.06, ..NONE };
    }
    // Biome dari mod lain: tebak dari tag.
    if FROZEN.on_biome(b) {
        return NONE;
    }
    if DRY.on_biome(b) {
        // Gurun: rumput kering dan dead bush.
        return Profile { dry_short: 0.30, dry_tall: 0.10, dead_bush: 0.012, ..NONE };
    }
    if FOREST.on_biome(b) && COLD.on_biome(b) {
        return Profile { grass: 0.30, tall: 0.10, fern: 0.50, large_fern: 0.25, berry: 0.30, ..NONE };
    }
    if FOREST.on_biome(b) {
        return Profile { grass: 0.38, tall: 0.06, fern: 0.08, bush: 0.12, flowers: 0.12, ..NONE };
    }
    if MOUNTAIN.on_biome(b) {
        return Profile { grass: 0.22, fern: 0.10, bush: 0.06, ..NONE };
    }
    Profile { grass: 0.60, tall: 0.30, bush: 0.06, flowers: 0.30, ..NONE }
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
const PATCH_BUSH: Patch = Patch { scale: 24.0, from: 0.58, to: 0.78 };
const PATCH_BERRY: Patch = Patch { scale: 35.0, from: 0.55, to: 0.75 };
const PATCH_FLOWERS: Patch = Patch { scale: 14.0, from: 0.55, to: 0.75 };
const PATCH_DRY: Patch = Patch { scale: 24.0, from: 0.35, to: 0.65 };
const PATCH_DRY_TALL: Patch = Patch { scale: 14.0, from: 0.50, to: 0.70 };
/// Sedikit rumput di mana-mana supaya padang tidak gundul (bagian dari kepadatan maksimal).
const GRASS_BASELINE: f64 = 0.12;
/// Pengali kelangkaan: bush dan berry dibuat jauh lebih jarang daripada angka profilnya.
const BUSH_RARITY: f64 = 0.25;
const BERRY_RARITY: f64 = 0.01;

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
    Berry,
    Dandelion,
    Rose,
    DryShort,
    DryTall,
    DeadBush,
}

/// Memilih tumbuhan untuk satu kolom (atau tidak ada). Urutan = prioritas.
fn choose(seed: u64, wx: i32, wz: i32, prof: Profile) -> Option<Plant> {
    let roll = |salt: i64| unit(hash(seed ^ 0x3C5A_17E5, wx as i64, salt, wz as i64));

    // Gurun: rumput kering tinggi (inti bercak) > dead bush > rumput kering pendek.
    if prof.dry_short > 0.0 {
        let dry_s = strength(field(seed, 0xF6, wx, wz, &PATCH_DRY), &PATCH_DRY);
        let tall_s = strength(field(seed, 0xF7, wx, wz, &PATCH_DRY_TALL), &PATCH_DRY_TALL) * dry_s;
        if roll(6) < prof.dry_tall * tall_s {
            return Some(Plant::DryTall);
        }
        if roll(7) < prof.dead_bush {
            return Some(Plant::DeadBush);
        }
        if roll(8) < prof.dry_short * (GRASS_BASELINE + (1.0 - GRASS_BASELINE) * dry_s) {
            return Some(Plant::DryShort);
        }
        return None;
    }

    let grass_s = strength(field(seed, 0xA1, wx, wz, &PATCH_GRASS), &PATCH_GRASS);

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
    // Bush: bercak kecil dan jarang.
    if prof.bush > 0.0 {
        let s = strength(field(seed, 0xE5, wx, wz, &PATCH_BUSH), &PATCH_BUSH);
        if roll(3) < prof.bush * BUSH_RARITY * s {
            return Some(Plant::Bush);
        }
    }
    // Sweet berry bush: sangat jarang.
    if prof.berry > 0.0 {
        let s = strength(field(seed, 0xE6, wx, wz, &PATCH_BERRY), &PATCH_BERRY);
        if roll(9) < prof.berry * BERRY_RARITY * s {
            return Some(Plant::Berry);
        }
    }
    // Bercak bunga: satu jenis per bercak besar (dandelion atau rose), seperti padang bunga Minecraft.
    if prof.flowers > 0.0 {
        let s = strength(field(seed, 0x11, wx, wz, &PATCH_FLOWERS), &PATCH_FLOWERS);
        if roll(10) < prof.flowers * 0.35 * s {
            let kind = noise2(seed ^ 0x22, wx, wz, 70.0);
            return Some(if kind < 0.5 { Plant::Dandelion } else { Plant::Rose });
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
            if prof.grass + prof.fern + prof.bush + prof.flowers + prof.dry_short == 0.0 {
                continue;
            }
            let (wx, wz) = (ox + x as i32, oz + z as i32);
            let Some(plant) = choose(seed, wx, wz, prof) else { continue };
            if g.get(x, y, z) != 0 {
                continue; // sudah ada isinya (bunga bawaan, pohon, ...)
            }
            let below = if y == 0 { g.surface_block(wx, wz) } else { g.get(x, y - 1, z) };
            let dry_plant = matches!(plant, Plant::DryShort | Plant::DryTall | Plant::DeadBush);
            if !(if dry_plant { &DRY_PLANT_SOIL } else { &PLANT_SOIL }).on_block(below) {
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
                // Semak beri berbuah penuh (age 3) saat dunia dibuat.
                Plant::Berry => g.set(x, y, z, SWEET_BERRY_BUSH.with(&[("age", "3")])),
                Plant::Dandelion => g.set(x, y, z, DANDELION.id()),
                Plant::Rose => g.set(x, y, z, ROSE.id()),
                Plant::DryShort => g.set(x, y, z, SHORT_DRY_GRASS.id()),
                Plant::DryTall => g.set(x, y, z, TALL_DRY_GRASS.id()),
                Plant::DeadBush => g.set(x, y, z, DEAD_BUSH.id()),
            }
        }
    }
}
