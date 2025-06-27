use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use xattr;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::Error;
use icns::{IconFamily, Image};

// A reverse-engineered implementation of the macos
// Icon\r resource fork binary format
pub fn encode(icns: &Vec<u8>) -> Result<Vec<u8>, Error> {
  let mut rsrc = Vec::new();
  let icon_size = icns.len() as u32;

  let mut header = [0; 65];
  header[0] = 0x100;
  header[1] = icon_size + 0x104;
  header[2] = icon_size + 0x4;
  header[3] = 0x32;
  header[64] = icon_size;

  for &n in &header {
    rsrc.write_u32::<BigEndian>(n)?;
  }
  for &n in icns {
    rsrc.write_u8(n)?;
  }
  for &n in &vec![
    0x00000100,
    icon_size + 0x104,
    icon_size + 0x4,
    0x00000032,
    0x00000000,
    0x00000000,
    0x001C0032,
    0x00006963,
    0x6E730000,
    0x000ABFB9,
    0xFFFF0000,
    0x00000000,
    0x00000000,
  ] {
    rsrc.write_u32::<BigEndian>(n)?;
  }

  Ok(rsrc)
}


// 写入 Icon\r 并隐藏
fn write_icon_file(folder: &Path, icns_data: &[u8]) -> std::io::Result<()> {
    let icon_path = folder.join("Icon\r");
    fs::write(&icon_path, icns_data)?;
    std::process::Command::new("chflags")
        .arg("hidden")
        .arg(&icon_path)
        .status()
        .ok();
    Ok(())
}

/// 设置 FinderInfo 的 kHasCustomIcon 标志（用 xattr crate）
fn set_finder_info_flag(folder: &Path) -> std::io::Result<()> {
    let attr_name = "com.apple.FinderInfo";

    let finder_info_hex = "0000000000000000040000000000000000000000000000000000000000000000";
    let finder_info_bytes = hex::decode(finder_info_hex)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    xattr::set(folder, attr_name, &finder_info_bytes)?;

    Ok(())
}

/// 纯 Rust：设置文件夹图标
pub fn set_custom_folder_icon(folder: &Path, icns_data: &[u8]) -> std::io::Result<()> {
    write_icon_file(folder, icns_data)?;
    set_finder_info_flag(folder)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let folder = Path::new("/Users/donke/Test/ddd1");
    // let png_bytes = include_bytes!("/Users/donke/Test/sync-folder-icon.png");
    // let mut icon_family = IconFamily::new();

    // icon_family.add_icon(&image.try_into()?)?;

    // let mut icns: Vec<u8> = Vec::new();
    // icon_family.write(&mut icns)?;


    let icns_data_bytes = include_bytes!("/Users/donke/Test/sync-folder-icon.icns");
    // let icns_data = std::fs::read("/Users/donke/Test/sync-folder-icon.png")?;
    let icns_data = icns_data_bytes.to_vec();
    let rsrc = encode(&icns_data).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    set_custom_folder_icon(folder, &rsrc)?;
    println!("✅ 设置成功！");
    Ok(())
}
