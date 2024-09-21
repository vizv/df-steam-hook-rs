mod functions;
mod globals;
mod offset;

pub struct Offsets {
  pub globals: globals::Globals,
  pub functions: functions::Functions,
}

pub const OFFSETS: Offsets = Offsets {
  globals: globals::GLOBALS,
  functions: functions::FUNCTIONS,
};
