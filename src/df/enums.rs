#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum CursesColor {
  Black = 0x0,
  Blue = 0x1,
  Green = 0x2,
  Aqua = 0x3,
  Red = 0x4,
  Purple = 0x5,
  Yellow = 0x6,
  White = 0x7,
  Gray = 0x8,
  LightBlue = 0x9,
  LightGreen = 0xa,
  LightAqua = 0xb,
  LightRed = 0xc,
  LightPurple = 0xd,
  LightYellow = 0xe,
  BrightWhite = 0xf,
}

impl From<i32> for CursesColor {
  fn from(value: i32) -> Self {
    unsafe { std::mem::transmute::<i32, CursesColor>(value & 0xf) }
  }
}

impl CursesColor {
  pub fn light(self) -> Self {
    ((self as i32) & 0x7 | 0x8).into()
  }
  pub fn dark(self) -> Self {
    ((self as i32) & 0x7).into()
  }
  pub fn with_bright(self, bright: bool) -> Self {
    if bright {
      self.light()
    } else {
      self.dark()
    }
  }
}

#[allow(dead_code, non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum LinkType {
  NONE = -1,
  HIST_FIG = 0,
  SITE = 1,
  ARTIFACT = 2,
  BOOK = 3,
  SUBREGION = 4,
  FEATURE_LAYER = 5,
  ENTITY = 6,
  ABSTRACT_BUILDING = 7,
  ENTITY_POPULATION = 8,
  ART_IMAGE = 9,
  ERA = 10,
  HEC = 11,
}
