use crate::{offsets, utils};

pub fn get_current_viewscreen_name(addr: usize) -> Option<&'static str> {
  let mut curr = addr + offsets::FIELDS.get("gview.view").unwrap();
  let mut ret = None;

  loop {
    let vtable_addr = unsafe { *(curr as *const usize) };
    if let Some((_, vtable)) = utils::OFFSETS.resolve(vtable_addr) {
      if let Some(vs) = offsets::vtables::VIEWSCREENS.get(&vtable) {
        ret = Some(vs.as_str())
      }
    }

    let next = raw::read(curr + offsets::FIELDS.get("viewscreen.child").unwrap());
    if next == 0 {
      break;
    }

    curr = next;
  }

  ret
}
