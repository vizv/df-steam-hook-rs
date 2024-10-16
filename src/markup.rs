use bitflags::bitflags;
use raw::{delete_cxxstring, new_cxxstring};
use std::collections::HashMap;

use crate::{df, encodings, screen};

#[static_init::dynamic]
pub static mut MARKUP: Markup = Default::default();

// TODO: remove this? as it's parsed by original code already
#[allow(dead_code)]
#[derive(Debug)]
struct MarkupLink {
  typ: df::enums::LinkType,
  id: i32,
  subid: i32,
}

impl MarkupLink {
  fn new(typ: df::enums::LinkType, id: i32, subid: i32) -> Self {
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
  color: df::common::Color,
  link_index: i32,
  x: i32,
  y: i32, // TODO: change this to pixels
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
  pub fn parse(content: &str) -> Self {
    let mut text: MarkupTextBox = Default::default();

    let chars = content.chars().collect::<Vec<char>>();

    let mut str = String::new();
    let mut link_index: i32 = -1;
    let mut color = df::enums::CursesColor::White;
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
                "HF" => df::enums::LinkType::HIST_FIG,
                "SITE" => df::enums::LinkType::SITE,
                "ARTIFACT" => df::enums::LinkType::ARTIFACT,
                "BOOK" => df::enums::LinkType::BOOK,
                "SR" => df::enums::LinkType::SUBREGION,
                "LF" => df::enums::LinkType::FEATURE_LAYER,
                "ENT" => df::enums::LinkType::ENTITY,
                "AB" => df::enums::LinkType::ABSTRACT_BUILDING,
                "EPOP" => df::enums::LinkType::ENTITY_POPULATION,
                "ART_IMAGE" => df::enums::LinkType::ART_IMAGE,
                "ERA" => df::enums::LinkType::ERA,
                "HEC" => df::enums::LinkType::HEC,
                _ => df::enums::LinkType::NONE,
              };

              let id = buff_id.parse::<i32>().unwrap_or(0);
              let mut subid = -1;

              match link_type {
                df::enums::LinkType::ABSTRACT_BUILDING | df::enums::LinkType::ART_IMAGE => {
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
                df::enums::LinkType::NONE => {}
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
                local_screenf = buff1.parse::<i32>().unwrap_or(7);
                local_screenbright = buff3.parse::<i32>().unwrap_or(1) != 0;
              }

              color = local_screenf.into();
              color = color.with_bright(local_screenbright);
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

              let key_ptr = new_cxxstring();
              df::globals::get_key_display(key_ptr, *df::globals::ENABLER, binding);
              ptr.str = encodings::read_raw_string(key_ptr);
              delete_cxxstring(key_ptr);
              ptr.color = df::gps::get_uccolor(*df::globals::GPS, df::enums::CursesColor::LightGreen);

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
        if encodings::cjk::is_cjk(ch) && !encodings::is_cjk_punctuation(ch) {
          text.insert(&mut str, link_index, color);
        }

        if ch != ' ' || no_split_space {
          // flush the previous string if last character is CJK character
          if str.len() > 0 {
            let last_ch = str.chars().last().unwrap();
            if encodings::cjk::is_cjk(last_ch) && !encodings::is_cjk_punctuation(ch) {
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

    self.max_y = 0;
    self.current_width = width;

    let width_in_pixels = width * screen::CANVAS_FONT_WIDTH;
    let mut remain_width = width_in_pixels;
    let mut x_val = 0;
    let mut y_val = 0;

    let mut iter = self.word.iter_mut().peekable();
    while let Some(cur_word) = iter.next() {
      if cur_word.flags.contains(MarkupWordFlag::NEWLINE) {
        remain_width = 0;
        continue;
      }

      if cur_word.flags.contains(MarkupWordFlag::BLANK_LINE) {
        remain_width = 0;
        x_val = 0;
        y_val += screen::CANVAS_FONT_HEIGHT;
        continue;
      }

      if cur_word.flags.contains(MarkupWordFlag::INDENT) {
        remain_width = width_in_pixels;
        x_val = 4 * screen::CANVAS_FONT_WIDTH;
        y_val += screen::CANVAS_FONT_HEIGHT;
        continue;
      }

      let word_width = encodings::string_width_in_pixels(&cur_word.str) as i32;
      if remain_width < word_width {
        remain_width = width_in_pixels;
        x_val = 0;
        y_val += screen::CANVAS_FONT_HEIGHT;
      }

      if let Some(next_word) = iter.peek() {
        if next_word.str.chars().count() == 1 {
          let next_char = next_word.str.chars().next().unwrap();
          if x_val > 0
            && remain_width <= (encodings::char_width_in_pixels(next_char) as i32 + screen::CANVAS_FONT_WIDTH)
          {
            match next_char {
              '.' | ',' | '?' | '!' => {
                remain_width = width_in_pixels;
                x_val = 0;
                y_val += screen::CANVAS_FONT_HEIGHT;
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
            cur_word.x = x_val - screen::CANVAS_FONT_WIDTH;
            cur_word.y = y_val;

            if self.max_y < y_val {
              self.max_y = y_val;
            }

            remain_width -= screen::CANVAS_FONT_WIDTH;
            x_val += screen::CANVAS_FONT_WIDTH;
            continue;
          }
          _ => {}
        }
      }

      cur_word.x = x_val;
      cur_word.y = y_val;

      if self.max_y < y_val {
        self.max_y = y_val;
      }

      remain_width -= word_width + screen::CANVAS_FONT_WIDTH;
      x_val += word_width + screen::CANVAS_FONT_WIDTH;

      if let Some(next_word) = iter.peek() {
        if cur_word.str.chars().count() > 0 && next_word.str.chars().count() > 0 {
          let cur_last_char = cur_word.str.chars().last().unwrap();
          let next_first_char = next_word.str.chars().next().unwrap();
          if encodings::cjk::is_cjk(cur_last_char) && encodings::cjk::is_cjk(next_first_char) {
            remain_width += screen::CANVAS_FONT_WIDTH;
            x_val -= screen::CANVAS_FONT_WIDTH;
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

  fn insert(&mut self, str: &mut String, link_index: i32, color: df::enums::CursesColor) -> bool {
    if str.is_empty() {
      return false;
    }

    let mut ptr: MarkupWord = Default::default();
    ptr.str = str.clone();
    ptr.link_index = link_index;

    ptr.color = df::gps::get_uccolor(*df::globals::GPS, color);

    self.word.push(ptr);
    str.clear();

    return true;
  }
}

#[derive(Default)]
pub struct Markup {
  items: HashMap<usize, MarkupTextBox>,
}

impl Markup {
  pub fn add(&mut self, address: usize, content: &str) {
    let text = MarkupTextBox::parse(content);

    self.items.insert(address, text);
  }

  pub fn layout(&mut self, address: usize, current_width: i32) -> i32 {
    if let Some(text) = self.items.get_mut(&address) {
      text.set_width(current_width);

      return (text.max_y as f32 / screen::CANVAS_FONT_HEIGHT as f32).ceil() as i32;
    }

    -1
  }

  pub fn render(&self, gps: usize, address: usize) {
    if let Some(text) = self.items.get(&address) {
      for word in &text.word {
        let text = screen::Text::new((&word.str, 0)).by_gps(gps);
        screen::SCREEN_TOP.write().add_text(text.with_offset(word.x, word.y).with_fg_color(word.color.clone()));
      }
    }
  }
}
