#!/bin/bash

# === 参数 ===
FOLDER_PATH="$1"
ICON_FILE="$2"

# === 校验输入 ===
if [[ ! -d "$FOLDER_PATH" ]]; then
  echo "❌ 目标文件夹不存在: $FOLDER_PATH"
  exit 1
fi

if [[ ! -f "$ICON_FILE" ]]; then
  echo "❌ 图标文件不存在: $ICON_FILE"
  exit 1
fi


rsrc=/tmp/icon.rsrc
sips -i $ICON_FILE
DeRez -only icns $ICON_FILE > $rsrc

# === 拷贝 .icns 图标到 Icon^M 文件 ===
touch $FOLDER_PATH/$'Icon\r'
# cat "$ICON_FILE" > $FOLDER_PATH/Icon?
Rez -append $rsrc -o $FOLDER_PATH/Icon?

# === 设置隐藏属性，让它不在 Finder 显示 ===
SetFile -a V $FOLDER_PATH/Icon?

# === 给文件夹设置 com.apple.FinderInfo 属性，表示使用自定义图标 ===
xattr -x -w com.apple.FinderInfo "0000000000000000040000000000000000000000000000000000000000000000" "$FOLDER_PATH"

# # # === 强制刷新 Finder 显示 ===
# osascript -e "tell application \"Finder\" to update POSIX file \"${FOLDER_PATH}\""

# echo "✅ 文件夹图标设置完成：$FOLDER_PATH"
