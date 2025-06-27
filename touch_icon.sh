#!/bin/bash

# === 参数 ===
ICON_FILE="$1"

if [[ ! -f "$ICON_FILE" ]]; then
  echo "❌ 图标文件不存在: $ICON_FILE"
  exit 1
fi


rsrc=/tmp/icon.rsrc
icon_tmp=/tmp/Icon_tmp
sips -i $ICON_FILE
DeRez -only icns $ICON_FILE > $rsrc

# === 拷贝 .icns 图标到 Icon^M 文件 ===
touch $icon_tmp
Rez -append $rsrc -o $icon_tmp

# === 给文件夹设置 com.apple.FinderInfo 属性，表示使用自定义图标 ===
# xattr -x -w com.apple.FinderInfo "0000000000000000040000000000000000000000000000000000000000000000" "$FOLDER_PATH"


# echo "✅ 文件夹图标设置完成：$FOLDER_PATH"
