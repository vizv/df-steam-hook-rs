use bitflags::bitflags;
use cxx::let_cxx_string;
use std::collections::HashMap;

use crate::{
  cjk,
  font::{self, FONT},
  global::{get_key_display, ENABLER, GPS},
  raw,
  screen::{self, CANVAS_FONT_HEIGHT, CANVAS_FONT_WIDTH, SCREEN_TOP},
};

const CURSES_FONT_WIDTH: i32 = font::CURSES_FONT_WIDTH as i32;

#[static_init::dynamic]
pub static mut MARKUP: Markup = Default::default();

#[allow(dead_code)]
enum CursesColor {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Yellow = 6,
  White = 7,
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
enum LinkType {
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

// TODO: remove this? as it's parsed by original code already
#[allow(dead_code)]
#[derive(Debug)]
struct MarkupLink {
  typ: LinkType,
  id: i32,
  subid: i32,
}

impl MarkupLink {
  fn new(typ: LinkType, id: i32, subid: i32) -> Self {
    Self { typ, id, subid }
  }
}

bitflags! {
  #[derive(Debug, Default)]
  pub struct MarkupWordFlag: u32 {
    const NEWLINE    = 0b0001;
    const BLANK_LINE = 0b0010;
    const INDENT     = 0b0100;
  }
}

#[derive(Debug, Default)]
struct MarkupWord {
  str: String,
  red: u8,
  green: u8,
  blue: u8,
  link_index: i32,
  x: i32,
  py: i32,
  flags: MarkupWordFlag,
}

#[derive(Debug, Default)]
struct MarkupTextBox {
  word: Vec<MarkupWord>,
  link: Vec<MarkupLink>,
  current_width: i32,
  max_y: i32,
  environment: usize, // pointer, not implemented
}

impl MarkupTextBox {
  // See DFHack: library/modules/Gui.cpp - void Gui::MTB_parse(df::markup_text_boxst *mtb, string parse_text)
  pub fn parse(content: &String) -> Self {
    // log::warn!("??? MarkupText::parse({})", content);
    let mut text: MarkupTextBox = Default::default();

    let chars = content.chars().collect::<Vec<char>>();

    let mut str = String::new();
    let mut link_index: i32 = -1;
    let mut color = CursesColor::White as usize;
    let mut use_char;
    let mut no_split_space;

    let i_max = chars.len();
    let mut i = 0;
    while i < i_max {
      let mut char_token = '\0';
      use_char = true;
      no_split_space = false;

      if chars[i] == ']' {
        // Skip over ']'
        i += 1;
        if i >= i_max {
          break;
        }

        if chars[i] != ']' {
          // Check this char again from top
          i -= 1;
          continue;
        }
        // else "]]", append ']' to str since use_char == true
      } else if chars[i] == '[' {
        // Skip over '['
        i += 1;
        if i >= i_max {
          break;
        }

        if chars[i] == '.' || chars[i] == ':' || chars[i] == '?' || chars[i] == ' ' || chars[i] == '!' {
          // Immediately after '['

          // Completely pointless for everything but ' '?
          no_split_space = true;
        } else if chars[i] != '[' {
          use_char = false;
          let token_buffer = Self::grab_token_string_pos(&chars, i, ':');
          i += token_buffer.chars().count();

          match token_buffer.as_str() {
            "CHAR" => {
              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff = Self::grab_token_string_pos(&chars, i, ':');
              let buff_chars = buff.chars().collect::<Vec<char>>();
              i += buff_chars.len();

              char_token = if buff_chars.len() > 1 && buff_chars[0] == '~' {
                buff_chars[1]
              } else {
                char::from_u32(buff.parse::<u32>().unwrap_or(0)).unwrap_or('\0')
              };
              no_split_space = true;
              use_char = true;
            }
            "LPAGE" => {
              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff_type = Self::grab_token_string_pos(&chars, i, ':');
              i += buff_type.len();

              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff_id = Self::grab_token_string_pos(&chars, i, ':');
              i += buff_id.len();

              let link_type = match buff_type.as_str() {
                "HF" => LinkType::HIST_FIG,
                "SITE" => LinkType::SITE,
                "ARTIFACT" => LinkType::ARTIFACT,
                "BOOK" => LinkType::BOOK,
                "SR" => LinkType::SUBREGION,
                "LF" => LinkType::FEATURE_LAYER,
                "ENT" => LinkType::ENTITY,
                "AB" => LinkType::ABSTRACT_BUILDING,
                "EPOP" => LinkType::ENTITY_POPULATION,
                "ART_IMAGE" => LinkType::ART_IMAGE,
                "ERA" => LinkType::ERA,
                "HEC" => LinkType::HEC,
                _ => LinkType::NONE,
              };

              let id = buff_id.parse::<i32>().unwrap_or(0);
              let mut subid = -1;

              match link_type {
                LinkType::ABSTRACT_BUILDING | LinkType::ART_IMAGE => {
                  // Skip over ':'
                  i += 1;
                  if i >= i_max {
                    break;
                  }

                  let buff_subid = Self::grab_token_string_pos(&chars, i, ':');
                  i += buff_subid.len();

                  subid = buff_subid.parse::<i32>().unwrap_or(0);
                }
                _ => {}
              }

              match link_type {
                LinkType::NONE => {}
                _ => {
                  let link = MarkupLink::new(link_type, id, subid);
                  text.link.push(link);
                  link_index = text.link.len() as i32 - 1;
                }
              }
            }
            "/LPAGE" => {
              text.insert(&mut str, link_index, color);
              link_index = -1;
            }
            "C" => {
              text.insert(&mut str, link_index, color);

              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff1 = Self::grab_token_string_pos(&chars, i, ':');
              i += buff1.len();

              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff2 = Self::grab_token_string_pos(&chars, i, ':');
              i += buff2.len();

              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff3 = Self::grab_token_string_pos(&chars, i, ':');
              i += buff3.len();

              let mut local_screenf = 7;
              let mut local_screenbright = true;

              if buff1 == "VAR" {
                let environment = if text.environment != 0 { "Active" } else { "NULL" };
                log::debug!("MTB_parse received:\n[C:VAR:{}:{}]\nwhich is for dipscripts and is unimplemented.\nThe dipscript environment itself is: {}", buff2, buff3, environment);
                //MTB_set_color_on_var(mtb, buff2, buff3);
              } else {
                // skip setting colors in GPS, use local variables for colors
                local_screenf = buff1.parse::<i32>().unwrap_or(0);
                local_screenbright = buff3.parse::<i32>().unwrap_or(0) != 0;
              }

              color = (local_screenf + if local_screenbright { 8 } else { 0 }) as usize;
            }
            "KEY" => {
              text.insert(&mut str, link_index, color);

              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff = Self::grab_token_string_pos(&chars, i, ':');
              i += buff.len();

              let mut ptr: MarkupWord = Default::default();
              let binding = buff.parse::<i32>().unwrap_or(0);

              unsafe {
                let_cxx_string!(key = "");
                let key_ptr: usize = core::mem::transmute(key);
                get_key_display(key_ptr, ENABLER.to_owned(), binding);
                ptr.str = raw::deref_string(key_ptr);
              };

              let base = (GPS.to_owned() + 0x158) + 3 * (CursesColor::Green as usize + 8);
              ptr.red = raw::deref::<u8>(base + 0);
              ptr.green = raw::deref::<u8>(base + 1);
              ptr.blue = raw::deref::<u8>(base + 2);

              text.word.push(ptr);
            }
            "VAR" => {
              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff_format = Self::grab_token_string_pos(&chars, i, ':');
              i += buff_format.len();

              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff_var_type = Self::grab_token_string_pos(&chars, i, ':');
              i += buff_var_type.len();

              // Skip over ':'
              i += 1;
              if i >= i_max {
                break;
              }

              let buff_var_name = Self::grab_token_string_pos(&chars, i, ':');
              i += buff_var_name.len();

              let environment = if text.environment != 0 { "Active" } else { "NULL" };
              log::debug!("MTB_parse received:\n[VAR:{}:{}:{}]\nwhich is for dipscripts and is unimplemented.\nThe dipscript environment itself is: {}\n", buff_format, buff_var_type, buff_var_name, environment);
            }
            "R" | "B" | "P" => {
              text.insert(&mut str, link_index, color);

              let mut ptr: MarkupWord = Default::default();

              ptr.flags |= match token_buffer.as_str() {
                "R" => MarkupWordFlag::NEWLINE,
                "B" => MarkupWordFlag::BLANK_LINE,
                _ => MarkupWordFlag::INDENT,
              };

              text.word.push(ptr);
            }
            _ => {}
          }
        }
      }

      if use_char {
        let ch = if char_token == '\0' { chars[i] } else { char_token };

        // flush if the next character is CJK character
        if cjk::is_cjk(ch) && !cjk::is_cjk_punctuation(ch) {
          text.insert(&mut str, link_index, color);
        }

        if ch != ' ' || no_split_space {
          // flush the previous string if last character is CJK character
          if str.len() > 0 {
            let last_ch = str.chars().last().unwrap();
            if cjk::is_cjk(last_ch) && !cjk::is_cjk_punctuation(ch) {
              text.insert(&mut str, link_index, color);
            }
          }

          str.push(ch);
        } else {
          text.insert(&mut str, link_index, color);
        }
      }

      i += 1;
    }

    text.insert(&mut str, link_index, color);

    let mut i = text.word.len();
    while i > 1 {
      i -= 1;
      let (left, right) = text.word.split_at_mut(i);

      let cur_entry = &right[0];
      if cur_entry.link_index != -1 || cur_entry.str.is_empty() {
        continue;
      }

      let prev_entry = &mut left[i - 1];
      if prev_entry.link_index == -1 || prev_entry.str.is_empty() {
        continue;
      }

      match cur_entry.str.chars().next().unwrap() {
        '.' | ',' | '?' | '!' | '。' | '，' | '？' | '！' => {
          prev_entry.str.push_str(&cur_entry.str);
          text.word.remove(i);
        }
        _ => {}
      }
    }

    text
  }

  // See DFHack: library/modules/Gui.cpp - void Gui::MTB_set_width(df::markup_text_boxst *mtb, int32_t width)
  pub fn set_width(&mut self, width: i32) {
    if self.current_width == width {
      return;
    }
    log::info!("??? set_width to {:?}", width);

    self.max_y = 0;
    self.current_width = width;

    let width_in_pixels = width * CURSES_FONT_WIDTH;
    let mut remain_width = width_in_pixels;
    let mut x_val = 0;
    let mut py_val = 0;

    let mut iter = self.word.iter_mut().peekable();
    while let Some(cur_word) = iter.next() {
      if cur_word.flags.contains(MarkupWordFlag::NEWLINE) {
        remain_width = 0;
        continue;
      }

      if cur_word.flags.contains(MarkupWordFlag::BLANK_LINE) {
        remain_width = 0;
        x_val = 0;
        py_val += 1;
        continue;
      }

      if cur_word.flags.contains(MarkupWordFlag::INDENT) {
        remain_width = width_in_pixels;
        x_val = 4 * CURSES_FONT_WIDTH;
        py_val += 1;
        continue;
      }

      let word_width = cur_word.str.chars().map(|ch| FONT.write().get_width(ch) as i32).sum();
      if remain_width < word_width {
        remain_width = width_in_pixels;
        x_val = 0;
        py_val += 1;
      }

      if let Some(next_word) = iter.peek() {
        if next_word.str.chars().count() == 1 {
          let next_char = next_word.str.chars().next().unwrap();
          if x_val > 0 && remain_width <= (FONT.write().get_width(next_char) as i32 + CURSES_FONT_WIDTH) {
            match next_char {
              '.' | ',' | '?' | '!' => {
                remain_width = width_in_pixels;
                x_val = 0;
                py_val += 1;
              }
              _ => {}
            }
          }
        }
      }

      if cur_word.str.chars().count() == 1 && x_val > 0 {
        let cur_char = cur_word.str.chars().next().unwrap();
        match cur_char {
          '.' | ',' | '?' | '!' => {
            cur_word.x = x_val - CURSES_FONT_WIDTH;
            cur_word.py = py_val;

            if self.max_y < py_val {
              self.max_y = py_val;
            }

            remain_width -= CURSES_FONT_WIDTH;
            x_val += CURSES_FONT_WIDTH;
            continue;
          }
          _ => {}
        }
      }

      cur_word.x = x_val;
      cur_word.py = py_val;

      if self.max_y < py_val {
        self.max_y = py_val;
      }

      remain_width -= word_width + CURSES_FONT_WIDTH;
      x_val += word_width + CURSES_FONT_WIDTH;

      if let Some(next_word) = iter.peek() {
        if cur_word.str.chars().count() > 0 && next_word.str.chars().count() > 0 {
          let cur_last_char = cur_word.str.chars().last().unwrap();
          let next_first_char = next_word.str.chars().next().unwrap();
          if FONT.write().is_cjk(cur_last_char) && FONT.write().is_cjk(next_first_char) {
            remain_width += CURSES_FONT_WIDTH;
            x_val -= CURSES_FONT_WIDTH;
          }
        }
      }
    }
  }

  fn grab_token_string_pos(source: &Vec<char>, pos: usize, compc: char) -> String {
    let mut out = String::new();

    for i in pos..source.len() {
      if source[i] == compc || source[i] == ']' {
        break;
      }
      out.push(source[i]);
    }

    out
  }

  fn insert(&mut self, str: &mut String, link_index: i32, color: usize) -> bool {
    if str.is_empty() {
      return false;
    }

    let mut ptr: MarkupWord = Default::default();
    ptr.str = str.clone();
    ptr.link_index = link_index;

    let base = (GPS.to_owned() + 0x158) + 3 * color;
    ptr.red = raw::deref::<u8>(base + 0);
    ptr.green = raw::deref::<u8>(base + 1);
    ptr.blue = raw::deref::<u8>(base + 2);

    self.word.push(ptr);
    str.clear();

    return true;
  }
}

#[derive(Default)]
pub struct Markup {
  items: HashMap<usize, MarkupTextBox>,
  // pub x: i32,
  // pub y: i32,
  // pub rendering: bool,
}

impl Markup {
  pub fn add(&mut self, address: usize, content: &String) {
    let text = MarkupTextBox::parse(content);

    self.items.insert(address, text);
  }

  pub fn layout(&mut self, address: usize, current_width: i32) -> i32 {
    if let Some(text) = self.items.get_mut(&address) {
      text.set_width(current_width);

      return text.max_y;
    }

    -1
  }

  pub fn render(&self, address: usize, x: i32, y: i32) {
    if let Some(text) = self.items.get(&address) {
      let gps = GPS.to_owned();
      let color_base = gps + 0x8c; // TODO: check Windows offset
      let color = raw::deref_mut::<screen::ColorInfo>(color_base);

      for word in &text.word {
        let wx = word.x;
        let wy = word.py * CANVAS_FONT_HEIGHT;
        unsafe {
          (*color).use_old_16_colors = false;
          (*color).screen_color_r = word.red;
          (*color).screen_color_g = word.green;
          (*color).screen_color_b = word.blue;
        }
        SCREEN_TOP.write().add(
          gps,
          wx + CANVAS_FONT_WIDTH * x,
          wy + CANVAS_FONT_HEIGHT * y,
          word.str.clone(),
          0,
        );
      }
    }
  }
}
