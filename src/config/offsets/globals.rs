use super::offset;

pub struct Globals {
  pub enabler: offset::OffsetTuple,
  pub game: offset::OffsetTuple,
  pub gps: offset::OffsetTuple,
  pub gview: offset::OffsetTuple,
}

#[cfg(all(target_os = "linux", not(feature = "steam")))]
pub const GLOBALS: Globals = Globals {
  enabler: ("self", 0x1fbb9e0),
  game: ("self", 0x21d4380),
  gps: ("self", 0x1f80340),
  gview: ("self", 0x2a58820),
};

#[cfg(all(target_os = "windows", not(feature = "steam")))]
pub const GLOBALS: Globals = Globals {
  enabler: ("self", 0x1eed870),
  game: ("self", 0x14084f0),
  gps: ("self", 0x21a5e40),
  gview: ("self", 0x1408330),
};

#[cfg(all(target_os = "windows", feature = "steam"))]
pub const GLOBALS: Globals = Globals {
  enabler: ("self", 0x1ef2940),
  game: ("self", 0x140d520),
  gps: ("self", 0x21aaf10),
  gview: ("self", 0x140d360),
};
