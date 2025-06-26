use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use xattr;

/// 文件夹中隐藏图标文件的真实名称：Icon\r（Icon 加一个回车）
fn icon_file_path(folder: &Path) -> PathBuf {
    let mut p = folder.to_path_buf();
    p.push("Icon\r"); // 真实文件名包含回车
    p
}

/// 设置 FinderInfo 的 HasCustomIcon 标志位（第 9 字节为 0x10）
fn set_has_custom_icon_flag(folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = match xattr::get(folder, "com.apple.FinderInfo")? {
        Some(d) if d.len() == 32 => d,
        _ => vec![0u8; 32],
    };

    data[8] |= 0x10; // 设置 HasCustomIcon 位
    xattr::set(folder, "com.apple.FinderInfo", &data)?;
    Ok(())
}

/// 拷贝 icns 图标为 Icon\r，并设置隐藏属性（需要安装 Xcode Command Line Tools）
fn copy_icon_and_hide(icns_path: &Path, folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let icon_dest = icon_file_path(folder);

    // 拷贝 icns 内容
    fs::copy(icns_path, &icon_dest)?;

    // 使用 macOS 的 SetFile 命令设置为隐藏（必须有 Xcode CLT）
    let status = std::process::Command::new("SetFile")
        .arg("-a")
        .arg("V")
        .arg(&icon_dest)
        .status()?;

    if !status.success() {
        eprintln!("⚠️ SetFile 设置隐藏失败。你可能没有安装 Xcode Command Line Tools");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 参数：folder_path icon_file.icns
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("用法: {} <folder_path> <icon_file.icns>", args[0]);
        std::process::exit(1);
    }

    let folder_path = Path::new(&args[1]);
    let icns_path = Path::new(&args[2]);

    if !folder_path.is_dir() {
        return Err(format!("❌ 文件夹不存在: {}", folder_path.display()).into());
    }
    if !icns_path.exists() {
        return Err(format!("❌ 图标文件不存在: {}", icns_path.display()).into());
    }

    copy_icon_and_hide(icns_path, folder_path)?;
    set_has_custom_icon_flag(folder_path)?;

    println!("✅ 设置成功！");
    println!("📦 文件夹: {}", folder_path.display());
    println!("📎 图标文件: {}", icns_path.display());
    println!("\n📌 如果 Finder 图标未更新，请运行：");
    println!(
        "osascript -e 'tell application \"Finder\" to update POSIX file \"{}\"'",
        folder_path.display()
    );

    Ok(())
}
