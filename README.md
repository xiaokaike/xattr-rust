```bash
xattr -wx com.apple.FinderInfo "0000000000000000040000000000000000000000000000000000000000000000" "/Users/donke/Test/ddd3"

xattr -p com.apple.FinderInfo ./ddd3 | xxd
00000000: 0a
```
