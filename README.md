### Install upx
# Windows
winget install UPX.UPX

# Ubuntu / Debian
sudo apt update && sudo apt install upx

### Optimize exe file capacity
upx --best --lzma onliner-gr.exe
