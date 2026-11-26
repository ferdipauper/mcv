//! Prototipe `mcv:chest`: tutup chest diputar (animasi buka/tutup) saat inventory dibuka/ditutup.
//! Tubuh chest = block entity bermodel `models/chest.geo.json` + `animations/chest.animation.json`.
//! Buka/tutup: `on_tick` menu jalan tiap tick selama menu terbuka; berhenti = tertutup.
use std::collections::HashMap;
use std::sync::Mutex;
use weft_sdk::*;

pub static CHEST: Block = Block::named("chest");
pub static CHEST_MENU: Menu = Menu::named("chest");
static FUNCTIONAL: CreativeTab = CreativeTab::named("base:functional");

/// entity -> tick sejak menu terakhir terlihat terbuka. Ada di peta = sedang terbuka.
static OPEN: Mutex<Option<HashMap<u64, u32>>> = Mutex::new(None);

pub fn register(r: &mut Registry) {
    r.block(&CHEST)
        .sound_group("base:block.wood")
        // Blok hanya hitbox: gambarnya dari model entity. Teksturnya transparan.
        .states(|_, s| s.texture("chest_hitbox").model_box([1.0, 0.0, 1.0], [15.0, 14.0, 15.0]))
        .hardness(2.5)
        .mined_by("axe", tool_tier::HAND)
        .tab(&FUNCTIONAL)
        .entity(|e| {
            e.inventory(27).on_tick(|t| {
                let mut g = OPEN.lock().unwrap();
                let map = g.get_or_insert_with(HashMap::new);
                let w = t.world();
                map.retain(|id, idle| {
                    *idle += 1;
                    if *idle > 4 {
                        w.action_play(*id, "close", false);
                        false
                    } else {
                        true
                    }
                });
            })
        })
        .opens(&CHEST_MENU);
    r.menu(&CHEST_MENU)
        .group("chest", Slots::chest(3).holder())
        .on_tick(|c| {
            let Some(h) = c.owner() else { return };
            let w = c.world();
            let id = w.block_entity(h.pos);
            if id == 0 {
                return;
            }
            let mut g = OPEN.lock().unwrap();
            let map = g.get_or_insert_with(HashMap::new);
            if map.insert(id, 0).is_none() {
                w.action_play(id, "open", false);
            }
        })
        .default_screen();
}
