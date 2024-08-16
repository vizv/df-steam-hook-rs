#[static_init::dynamic]
pub static mut SCREEN: Screen = Default::default();

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Default::default();

pub struct Text {
  x: i32,
  y: i32,
  text: String,
}

impl Text {
  pub fn debug(&self) {
    log::debug!("{},{}: {}", self.x, self.y, self.text);
  }
}

#[derive(Default)]
pub struct Screen {
  texts: Vec<Text>,
}

impl Screen {
  pub fn add(&mut self, x: i32, y: i32, text: String) {
    self.texts.push(Text { x, y, text });
  }

  pub fn clear(&mut self) {
    self.texts.clear();
  }

  pub fn render(&self) {
    // log::debug!("size: {}", self.texts.len());
    // for text in &self.texts {
    //   text.debug();
    // }
  }
}
