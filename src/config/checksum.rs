use anyhow::{anyhow, Result};

#[cfg(target_os = "windows")]
const PATH_EXE: &'static str = "./Dwarf Fortress.exe";
#[cfg(target_os = "linux")]
const PATH_EXE: &'static str = "./dwarfort";

pub struct Checksum(u32);

impl Checksum {
  #[cfg(target_os = "linux")]
  fn checksum(&self) -> Result<u32> {
    let mut crc = checksum::crc::Crc::new(&PATH_EXE);
    match crc.checksum() {
      Ok(checksum) => Ok(checksum.crc32),
      Err(e) => Err(anyhow!("校验和错误：{:?}", e).into()),
    }
  }

  #[cfg(target_os = "windows")]
  fn checksum(&self) -> Result<u32> {
    use std::path::Path;
    use exe::{VecPE, PE};
    let pefile = VecPE::from_disk_file(Path::new(&PATH_EXE))?;
    Ok(pefile.get_nt_headers_64()?.file_header.time_date_stamp)
  }

  pub fn verify(&self) -> Result<u32> {
    let checksum = self.checksum()?;
    if self.0 != checksum {
      return Err(anyhow!("这个版本的《矮人要塞》不受支持，校验和：0x{checksum:x}"));
    }

    Ok(checksum)
  }
}

#[cfg(all(target_os = "linux", not(feature = "steam")))]
pub const CHECKSUM: Checksum = Checksum(0x1349f14a);

#[cfg(all(target_os = "windows", not(feature = "steam")))]
pub const CHECKSUM: Checksum = Checksum(0x66256bbd);

#[cfg(all(target_os = "windows", feature = "steam"))]
pub const CHECKSUM: Checksum = Checksum(0x6625635e);
