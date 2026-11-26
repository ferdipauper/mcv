//! Minecraft Core Visuals (MCV) untuk Weft 26.11.3.
//!
//! - `short_grass`: rumput pendek (1 blok) yang sangat banyak.
//! - `tall_grass_bottom` + `tall_grass_top`: rumput tinggi 2 blok seperti Minecraft.
//! - Dekorasi worldgen yang menambah kepadatan rumput di semua biome berumput.
//! Tekstur bergaya blocky 16x16, semuanya digambar orisinal.

use weft_sdk::noise::*;
use weft_sdk::*;

declare_mod!(deps: ["base"], init: init);

pub static SHORT_GRASS: Block = Block::named("short_grass");
pub static TALL_GRASS_BOTTOM: Block = Block::named("tall_grass_bottom");
pub static TALL_GRASS_TOP: Block = Block::named("tall_grass_top");

// Milik base, dipanggil dengan nama lengkap.
static PLANT_SOIL: Tag = Tag::named("base:plant_soil");
static FROZEN: Tag = Tag::named("base:frozen");
static GRASS_COVER: Component<f32> = Component::named("base:grass");
static FLOWERS_COVER_ROLL_SALT: u64 = 0xF10; // sama dengan dekorasi base
static MCV_SALT: u64 = 0x3C5A_17E5;

const SEA: i32 = 63;

fn init(r: &mut Registry) {
    // Rumput pendek: tanaman silang rendah, diwarnai biome, mudah diinjak/diganti.
    r.block(&SHORT_GRASS)
        .sound_group("grass")
        .texture("short_grass")
        .plant()
        .replaceable()
        .hitbox([4.0, 0.0, 4.0], [12.0, 8.0, 12.0])
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .no_item();
    r.loot(&SHORT_GRASS)
        .pool(Pool::new().when(Cond::chance(0.1)).item("wheat_seeds"));

    // Rumput tinggi = dua blok bertumpuk. Bawah butuh tanah, atas butuh bagian bawah.
    r.block(&TALL_GRASS_BOTTOM)
        .sound_group("grass")
        .texture("tall_grass_bottom")
        .plant()
        .replaceable()
        .needs_below("#plant_soil")
        .tint(&tints::GRASS)
        .no_item();
    r.block(&TALL_GRASS_TOP)
        .sound_group("grass")
        .texture("tall_grass_top")
        .plant()
        .replaceable()
        .needs_below("mcv:tall_grass_bottom")
        .tint(&tints::GRASS)
        .no_item();
    r.loot(&TALL_GRASS_BOTTOM)
        .pool(Pool::new().when(Cond::chance(0.15)).item("wheat_seeds"));
    r.loot(&TALL_GRASS_TOP)
        .pool(Pool::new().when(Cond::chance(0.15)).item("wheat_seeds"));

    r.decoration(more_grass);
}

/// Mengisi kolom yang dilewati dekorasi base dengan rumput pendek dan rumput tinggi.
/// Fungsi murni: seed + posisi selalu memberi hasil sama (syarat worldgen Weft).
fn more_grass(g: &mut Gen) {
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
            let b = col.biome;
            if FROZEN.on_biome(b) {
                continue;
            }
            let (wx, wz) = (ox + x as i32, oz + z as i32);
            let cover = GRASS_COVER.on_biome(b).unwrap_or(0.05) as f64;

            // Kolom yang sudah dipakai base (rumput/bunga): jangan disentuh.
            let base_roll = unit(hash(seed ^ FLOWERS_COVER_ROLL_SALT, wx as i64, 0, wz as i64));
            if base_roll < cover.max(0.012) {
                continue;
            }
            if g.get(x, y, z) != 0 {
                continue;
            }
            let below = if y == 0 { g.surface_block(wx, wz) } else { g.get(x, y - 1, z) };
            if !PLANT_SOIL.on_block(below) {
                continue;
            }

            let roll = unit(hash(seed ^ MCV_SALT, wx as i64, 1, wz as i64));
            let p_tall = cover * 0.6;
            let p_short = 0.20 + cover * 1.2;
            if roll < p_tall && y + 1 < 16 && g.get(x, y + 1, z) == 0 {
                g.set(x, y, z, TALL_GRASS_BOTTOM.id());
                g.set(x, y + 1, z, TALL_GRASS_TOP.id());
            } else if roll < p_tall + p_short {
                g.set(x, y, z, SHORT_GRASS.id());
            }
        }
    }
}
