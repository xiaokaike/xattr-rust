use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use xattr;

/// 获取 Icon^M 的路径（Icon 后跟 \r）
fn icon_file_path(folder: &Path) -> PathBuf {
    let mut p = folder.to_path_buf();
    p.push("Icon\r");
    p
}

fn run(cmd: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new(cmd).args(args).status()?;
    if !status.success() {
        Err(format!("❌ 命令执行失败: {} {:?}", cmd, args).into())
    } else {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("用法: {} <FOLDER_PATH> <ICON_FILE>", args[0]);
        std::process::exit(1);
    }

    let folder = Path::new(&args[1]);
    let icon = Path::new(&args[2]);

    if !folder.is_dir() {
        return Err(format!("❌ 文件夹不存在: {}", folder.display()).into());
    }
    if !icon.exists() {
        return Err(format!("❌ 图标文件不存在: {}", icon.display()).into());
    }

    // === 步骤 1: sips -i 图标，写入资源分支
    run("sips", &["-i", icon.to_str().unwrap()])?;

    // === 步骤 2: DeRez 提取 icns 为 rsrc
    let rsrc_path = "/tmp/icon.rsrc";
    // run("DeRez", &["-only", "icns", icon.to_str().unwrap(), ">", rsrc_path])?;
    // ⚠️ 注意：上面这行 > 并不会生效，因为 `>` 是 shell 功能。我们需要使用 shell 包装：

    let rsrc_command = format!(
        "DeRez -only icns {} > {}",
        icon.to_str().unwrap(),
        rsrc_path
    );
    run("sh", &["-c", &rsrc_command])?;

    // === 步骤 3: 创建 Icon\r 文件
    let icon_file = icon_file_path(folder);
    fs::write(&icon_file, &[])?; // 相当于 touch

    // === 步骤 4: 注入 rsrc 到 Icon\r
    run("Rez", &["-append", rsrc_path, "-o", icon_file.to_str().unwrap()])?;

    // === 步骤 5: 设置 Icon\r 为隐藏
    run("SetFile", &["-a", "V", icon_file.to_str().unwrap()])?;

    // === 步骤 6: 设置 com.apple.FinderInfo，标志使用自定义图标
    let finder_info_hex = "0000000000000000040000000000000000000000000000000000000000000000";
    let finder_info_bytes = hex::decode(finder_info_hex)?;
    xattr::set(folder, "com.apple.FinderInfo", &finder_info_bytes)?;

    println!("✅ 文件夹图标设置完成：{}", folder.display());

    Ok(())
}
