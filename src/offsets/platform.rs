use crate::utils;

#[static_init::dynamic]
pub static PLATFORM: String = {
  let os = std::env::consts::OS;
  let mut err_details = vec![format!("系统：{os}")];

  let mut checksum_opt: Option<u32> = None;
  if let Some(checksums) = super::checksum::CHECKSUMS.get(os) {
    #[cfg(target_os = "linux")]
    {
      const PATH_EXE: &'static str = "./dwarfort";
      let mut crc = checksum::crc::Crc::new(&PATH_EXE);
      match crc.checksum() {
        Ok(checksum) => checksum_opt = Some(checksum.crc32),
        Err(e) => {
          err_details.push(format!("校验和错误：{:?}", e));
        }
      }
    }

    #[cfg(target_os = "windows")]
    {
      use exe::{VecPE, PE};
      use std::path::Path;
      const PATH_EXE: &'static str = "./Dwarf Fortress.exe";
      match VecPE::from_disk_file(Path::new(&PATH_EXE)) {
        Ok(pefile) => match pefile.get_nt_headers_64() {
          Ok(nt_headers) => {
            checksum_opt = Some(nt_headers.file_header.time_date_stamp);
          }
          Err(e) => {
            err_details.push(format!("读取可执行文件头部信息出错：{:?}", e));
          }
        },
        Err(e) => {
          err_details.push(format!("读取可执行文件出错：{:?}", e));
        }
      }
    }

    if let Some(checksum) = checksum_opt {
      err_details.push(format!("校验和：0x{checksum:x}"));

      if let Some(platform) = checksums.get(&checksum) {
        return format!("{os}-{platform}");
      }
    }
  }

  let message = format!("不支持的版本！{}", err_details.join("，"));
  utils::show_error_dialog(&message);
  panic!("{}", message);
};
