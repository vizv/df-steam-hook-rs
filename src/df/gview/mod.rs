use crate::offsets;

pub fn get_current_viewscreen_name(addr: usize) -> Option<&'static str> {
  let mut curr = addr + offsets::FIELDS.get("gview.view").unwrap();
  let mut ret = None;

  loop {
    if let Some(vs) = offsets::vtables::VIEWSCREENS.get(&unsafe { *(curr as *const usize) }) {
      ret = Some(vs.as_str())
    }

    let next = raw::read(curr + offsets::FIELDS.get("viewscreen.child").unwrap());
    if next == 0 {
      break;
    }

    curr = next;
  }

  ret
}
