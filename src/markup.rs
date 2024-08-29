use bitflags::bitflags;
use std::collections::HashMap;

use crate::{cjk, global::GPS, raw};

#[static_init::dynamic]
pub static mut MARKUP: Markup = Default::default();

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
  px: u32,
  py: u32,
  flags: MarkupWordFlag,
}

#[derive(Debug, Default)]
struct MarkupTextBox {
  word: Vec<MarkupWord>,
  link: Vec<MarkupLink>,
  current_width: u32,
  max_y: u32,
  environment: usize, // pointer, not implemented
}

impl MarkupTextBox {
  // See DFHack: library/modules/Gui.cpp - void Gui::MTB_parse(df::markup_text_boxst *mtb, string parse_text)
  fn parse(content: &String) -> Self {
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
                // skip: gps->screenf = (df::curses_color)atoi(buff1.c_str());
                // skip: gps->screenb = (df::curses_color)atoi(buff2.c_str());
                // skip: gps->screenbright = (bool)atoi(buff3.c_str());
                // skip: gps->use_old_16_colors = true;
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
              // TODO: ptr->str = df::global::enabler->GetKeyDisplay((df::interface_key)atoi(buff.c_str()));

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
}

impl Markup {
  pub fn add(&mut self, address: usize, content: &String) {
    let text = MarkupTextBox::parse(content);
    // log::info!("??? {:?}", text);

    for word in &text.word {
      log::info!("??? {}", word.str);
    }

    self.items.insert(address, text);
  }
}
