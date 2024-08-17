use crate::font::FONT;

#[static_init::dynamic]
pub static mut SCREEN: Screen = Default::default();

#[static_init::dynamic]
pub static mut SCREEN_TOP: Screen = Default::default();

pub struct Text {
  x: i32,
  y: i32,
  content: String,
  flag: u32,
}

impl Text {
  pub fn debug(&self) {
    log::debug!("{},{}: {}", self.x, self.y, self.content);
  }
}

#[derive(Default)]
pub struct Screen {
  texts: Vec<Text>,
}

impl Screen {
  pub fn add(&mut self, x: i32, y: i32, content: String, flag: u32) {
    self.texts.push(Text { x, y, content, flag });
  }

  pub fn clear(&mut self) {
    self.texts.clear();
  }

  pub fn render(&self) {
    // log::debug!("size: {}", self.texts.len());
    for text in &self.texts {
      text.content.chars().for_each(|ch| {
        let unicode = ch as u16;
        if unicode < 256 {
          return;
        }

        FONT.write().render(unicode);
      });
      // text.debug();
    }
  }
}
