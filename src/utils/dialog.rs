const PREFIX: &str = "dfint-rust-cjk 错误";
pub fn show_error_dialog(message: &str) {
  eprintln!("{PREFIX}：{message}");
  log::error!("{PREFIX}：{message}");
  let _ = sdl2::messagebox::show_simple_message_box(sdl2::messagebox::MessageBoxFlag::ERROR, PREFIX, message, None);
}
