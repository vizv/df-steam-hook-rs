mod functions;
mod globals;
mod offset;

#[derive(Debug, Default)]
pub struct Offsets {
  pub globals: globals::Globals,
  pub functions: functions::Functions,
}

pub const OFFSETS: Offsets = Offsets {
  globals: globals::GLOBALS,
  functions: functions::FUNCTIONS,
};
