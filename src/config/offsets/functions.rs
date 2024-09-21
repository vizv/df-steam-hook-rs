use super::offset;

#[derive(Debug, Default)]
pub struct Functions {
  // addst hooks (Linux-only)
  #[cfg(target_os = "linux")]
  pub addst: offset::OffsetTuple,
  #[cfg(target_os = "linux")]
  pub addst_flag: offset::OffsetTuple,

  // addchar hooks (Linux-only)
  #[cfg(target_os = "windows")]
  pub addchar: offset::OffsetTuple,
  #[cfg(target_os = "windows")]
  pub addchar_flag: offset::OffsetTuple,

  // FIXME: top_addst hook
  pub addst_top: offset::OffsetTuple,

  // draw nineslice hooks, for dialogs covering
  pub draw_nineslice: offset::OffsetTuple,
  pub draw_horizontal_nineslice: offset::OffsetTuple,

  // gps_allocate hook, for resizing
  pub gps_allocate: offset::OffsetTuple,
  pub update_all: offset::OffsetTuple,
  pub update_tile: offset::OffsetTuple,

  // markup text hooks, for formatting markup texts
  pub mtb_process_string_to_lines: offset::OffsetTuple,
  pub mtb_set_width: offset::OffsetTuple,

  // render_help_dialog hook, for rendering the help dialog
  pub render_help_dialog: offset::OffsetTuple,

  // get_key_display function for getting labels for keys
  pub get_key_display: offset::OffsetTuple,
}

#[cfg(all(target_os = "linux", not(feature = "steam")))]
pub const FUNCTIONS: Functions = Functions {
  addst: ("libg_src_lib.so", 0x04f3e0),
  addst_flag: ("libg_src_lib.so", 0x04f110),

  addst_top: ("libg_src_lib.so", 0x04f970),

  draw_nineslice: ("libg_src_lib.so", 0x0b2cc0),
  draw_horizontal_nineslice: ("libg_src_lib.so", 0x0b31d0),

  gps_allocate: ("libg_src_lib.so", 0x05d140),

  update_all: ("libg_src_lib.so", 0x06f4d0),
  update_tile: ("libg_src_lib.so", 0x06e970),

  mtb_process_string_to_lines: ("self", 0x18b77c0),
  mtb_set_width: ("self", 0x18b7340),
  render_help_dialog: ("self", 0x1193fe0),

  get_key_display: ("libg_src_lib.so", 0x074710),
};

#[cfg(all(target_os = "windows", not(feature = "steam")))]
pub const FUNCTIONS: Functions = Functions {
  addchar: ("self", 0x058320),
  addchar_flag: ("self", 0x81dbd0),

  addst_top: ("self", 0x81e3b0),

  draw_horizontal_nineslice: ("self", 0x10459f0),
  draw_nineslice: ("self", 0x1045810),

  gps_allocate: ("self", 0x64da80),

  update_all: ("self", 0x64b170),
  update_tile: ("self", 0x647b40),

  mtb_process_string_to_lines: ("self", 0xa14470),
  mtb_set_width: ("self", 0xa14240),
  render_help_dialog: ("self", 0x142790),

  get_key_display: ("self", 0x655480),
};

#[cfg(all(target_os = "windows", feature = "steam"))]
pub const FUNCTIONS: Functions = Functions {
  addchar: ("self", 0x058390),
  addchar_flag: ("self", 0x820500),

  addst_top: ("self", 0x820ce0),

  draw_horizontal_nineslice: ("self", 0x1049eb0),
  draw_nineslice: ("self", 0x1049cd0),

  gps_allocate: ("self", 0x650310),

  update_all: ("self", 0x64da00),
  update_tile: ("self", 0x64a3d0),

  mtb_process_string_to_lines: ("self", 0xa16da0),
  mtb_set_width: ("self", 0xa16b70),
  render_help_dialog: ("self", 0x142800),

  get_key_display: ("self", 0x657d10),
};
