use anyhow::Result;
use retour::static_detour;

use crate::config::CONFIG;
// use crate::cxxstring::CxxString;
// use crate::dictionary::DICTIONARY;
use crate::utils;

use r#macro::hook;

#[cfg(target_os = "linux")]
#[static_init::dynamic]
static ENABLER: usize = unsafe {
  match CONFIG.symbol.is_some() {
    true => {
      utils::symbol_handle_self::<*const i64>(&CONFIG.symbol.as_ref().unwrap().enabler.as_ref().unwrap()[1]) as usize
    }
    false => 0 as usize,
  }
};

#[cfg(target_os = "linux")]
#[static_init::dynamic]
static GPS: usize = unsafe {
  match CONFIG.symbol.is_some() {
    true => utils::symbol_handle_self::<*const i64>(&CONFIG.symbol.as_ref().unwrap().gps.as_ref().unwrap()[1]) as usize,
    false => 0 as usize,
  }
};

pub unsafe fn attach_all() -> Result<()> {
  attach_delete_all_post_init_textures()?;
  Ok(())
}

pub unsafe fn enable_all() -> Result<()> {
  enable_delete_all_post_init_textures()?;
  Ok(())
}

pub unsafe fn disable_all() -> Result<()> {
  disable_delete_all_post_init_textures()?;
  Ok(())
}

#[cfg_attr(
  target_os = "linux",
  hook(module = "libg_src_lib.so", symbol = "_ZN8textures29delete_all_post_init_texturesEv")
)]
fn delete_all_post_init_textures(tex: usize) {
  log::debug!("delete_all_post_init_textures prevented!")
}
