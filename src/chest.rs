//! `mcv:chest` untuk Weft 26.11.3: chest bertexture sendiri (models/block/chest.json), tanpa animasi.
//! Animasi tutup chest baru tersedia di Weft master (`animated_part`), dan chest bawaan master sudah memakainya.
use weft_sdk::*;

pub static CHEST: Block = Block::named("chest");
pub static CHEST_MENU: Menu = Menu::named("chest");
static FUNCTIONAL: CreativeTab = CreativeTab::named("base:functional");

pub fn register(r: &mut Registry) {
    r.block(&CHEST)
        .sound_group("base:block.wood")
        .texture("chest")
        .hitbox([1.0, 0.0, 1.0], [15.0, 15.0, 15.0])
        .hardness(2.5)
        .mined_by("axe", tool_tier::HAND)
        .tab(&FUNCTIONAL)
        .entity(|e| e.inventory(27))
        .opens(&CHEST_MENU);
    r.menu(&CHEST_MENU)
        .group("chest", Slots::chest(3).holder())
        .default_screen();
}
