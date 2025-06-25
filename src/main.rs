use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use xattr;

fn set_folder_icon(folder_path: &str, icon_path: &str) -> std::io::Result<()> {
    let folder = Path::new(folder_path);

    // Step 1: 创建 Icon\r 文件，写入图标数据
    let icon_file_path: PathBuf = folder.join("Icon\r");
    let icon_data = fs::read(icon_path)?;
    let mut icon_file = File::create(&icon_file_path)?;
    icon_file.write_all(&icon_data)?;

    // Step 2: 获取并修改 com.apple.FinderInfo 属性（32字节）
    let mut finder_info = match xattr::get(folder, "com.apple.FinderInfo") {
        Ok(Some(data)) if data.len() == 32 => {
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&data);
            buf
        }
        _ => [0u8; 32], // 新建空 FinderInfo
    };

    finder_info[8] |= 0x04; // 第10字节，第3位为 HasCustomIcon
    println!("{:?}", finder_info);
    xattr::set(folder, "com.apple.FinderInfo", &finder_info)?;

    // Step 3: 隐藏 Icon\r 文件
    Command::new("chflags")
        .arg("hidden")
        .arg(icon_file_path.to_str().unwrap())
        .status()?;

    // Step 4: 通知 Finder 更新显示
    Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "tell application \"Finder\" to update POSIX file \"{}\"",
            folder.display()
        ))
        .status()?;

    Ok(())
}
fn main() {
    let data = xattr::get("/Users/donke/Test/bbbb", "com.apple.FinderInfo");
    match data {
        Ok(Some(data)) => {
            println!("{:?}", data);
        }
        _ => {
            println!("没有找到 FinderInfo 属性");
        }
    }

    let folder = "/Users/donke/Test/ddd3";
    let icon = "/Users/donke/Test/icon.icns";
    match set_folder_icon(folder, icon) {
        Ok(_) => println!("图标设置成功"),
        Err(e) => eprintln!("设置失败: {}", e),
    }


}
