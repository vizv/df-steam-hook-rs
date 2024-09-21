use std::collections::HashMap;

use super::{offsets, utils};

#[static_init::dynamic]
static VTABLES: HashMap<usize, &'static str> = {
  let mut ret = HashMap::new();
  ret.insert(0x1f79198, "adopt_region");
  ret.insert(0x1f754f0, "assign_display_item");
  ret.insert(0x1f75558, "building");
  ret.insert(0x1f7bb60, "choose_game_type");
  ret.insert(0x1f7bbc8, "choose_start_site");
  ret.insert(0x1f756c0, "dwarfmode");
  ret.insert(0x1f753b8, "export_region");
  ret.insert(0x1f75488, "game_cleaner");
  ret.insert(0x1f75de0, "initial_prep");
  ret.insert(0x1f755c0, "job");
  ret.insert(0x1f79268, "layer");
  ret.insert(0x1f79840, "legends");
  ret.insert(0x1f79200, "loadgame");
  ret.insert(0x1f7a058, "new_arena");
  ret.insert(0x1f7a0c0, "new_region");
  ret.insert(0x1f79130, "savegame");
  ret.insert(0x1f7bc30, "setupdwarfgame");
  ret.insert(0x1f75d78, "title");
  ret.insert(0x1f75420, "update_region");
  ret.insert(0x1f75970, "world");

  ret
};

pub fn deref_current_viewscreen(addr: usize) -> Option<&'static str> {
  let mut curr = addr + offsets::GVIEW_VIEW;
  let mut ret = None;

  loop {
    if let Some(&vs) = VTABLES.get(&unsafe { *(curr as *const usize) }) {
      ret = Some(vs)
    }

    let next = utils::deref(curr + offsets::VIEWSCREEN_CHILD);
    if next == 0 {
      break;
    }

    curr = next;
  }

  ret
}
