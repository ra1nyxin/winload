warning: in the working copy of 'docs/dev/pypi手动发布捏.md', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'py/pyproject.toml', LF will be replaced by CRLF the next time Git touches it
[1mdiff --git "a/docs/dev/pypi\346\211\213\345\212\250\345\217\221\345\270\203\346\215\217.md" "b/docs/dev/pypi\346\211\213\345\212\250\345\217\221\345\270\203\346\215\217.md"[m
[1mindex 2fe21f6..a3c1d5e 100644[m
[1m--- "a/docs/dev/pypi\346\211\213\345\212\250\345\217\221\345\270\203\346\215\217.md"[m
[1m+++ "b/docs/dev/pypi\346\211\213\345\212\250\345\217\221\345\270\203\346\215\217.md"[m
[36m@@ -18,11 +18,12 @@[m [mgrep '^version' py/pyproject.toml[m
 [m
 ### 2. 安装 uv（如果还没安装）[m
 ```bash[m
[31m-# Windows[m
[31m-winget install uv[m
[32m+[m[32m# Windows (PowerShell)[m
[32m+[m[32mpowershell -ExecutionPolicy Bypass -c "irm https://gitee.com/wangnov/uv-custom/releases/download/0.10.5/uv-installer-custom.ps1 | iex"[m
 [m
 # macOS / Linux[m
[31m-curl -LsSf https://astral.sh/uv/install.sh | sh[m
[32m+[m[32mcurl -LsSf https://gitee.com/wangnov/uv-custom/releases/download/0.10.5/uv-installer-custom.sh | sh[m
[32m+[m
 ```[m
 [m
 ### 3. 安装构建依赖[m
[1mdiff --git a/py/pyproject.toml b/py/pyproject.toml[m
[1mindex e3cabc6..8131a27 100644[m
[1m--- a/py/pyproject.toml[m
[1m+++ b/py/pyproject.toml[m
[36m@@ -2,7 +2,7 @@[m
 name = "winload"[m
 version = "0.1.6-beta.3"[m
 description = "Network Load Monitor - nload-like TUI tool for Windows"[m
[31m-readme = "../README.md"[m
[32m+[m[32mreadme = "README.md"[m
 license = { text = "MIT" }[m
 authors = [[m
     { name = "VincentZyu", email = "1830540513zyu@gmail.com" }[m
[36m@@ -25,5 +25,8 @@[m [mIssues = "https://github.com/VincentZyuApps/winload/issues"[m
 requires = ["hatchling"][m
 build-backend = "hatchling.build"[m
 [m
[32m+[m[32m[tool.hatch.build.targets.wheel][m
[32m+[m[32mpackages = ["."][m
[32m+[m
 [tool.uv][m
 dev-dependencies = [][m
[1mdiff --git a/readme.jp.md b/readme.jp.md[m
[1mindex 01944f0..1386fad 100644[m
[1m--- a/readme.jp.md[m
[1m+++ b/readme.jp.md[m
[36m@@ -42,7 +42,7 @@[m [mscoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket[m
 scoop install winload[m
 ```[m
 [m
[31m-**Arch Linux (AUR):**[m
[32m+[m[32m### Arch Linux (AUR):[m[41m[m
 ```bash[m
 paru -S winload-bin[m
 ```[m
[36m@@ -57,7 +57,7 @@[m [mcurl -fsSL https://raw.githubusercontent.com/VincentZyuApps/winload/main/docs/in[m
 > 📄 [インストールスクリプトのソースを表示](https://github.com/VincentZyuApps/winload/blob/main/docs/install_scripts/install.sh)[m
 [m
 <details>[m
[31m-<summary>手動インストール / その他のプラットフォーム</summary>[m
[32m+[m[32m<summary>手動インストール</summary>[m[41m[m
 [m
 **DEB (Debian/Ubuntu):**[m
 ```bash[m
[1mdiff --git a/readme.ko.md b/readme.ko.md[m
[1mindex 6582701..f4547d6 100644[m
[1m--- a/readme.ko.md[m
[1m+++ b/readme.ko.md[m
[36m@@ -41,7 +41,7 @@[m [mscoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket[m
 scoop install winload[m
 ```[m
 [m
[31m-**Arch Linux (AUR):**[m
[32m+[m[32m### Arch Linux (AUR):[m[41m[m
 ```bash[m
 paru -S winload-bin[m
 ```[m
[36m@@ -56,7 +56,7 @@[m [mcurl -fsSL https://raw.githubusercontent.com/VincentZyuApps/winload/main/docs/in[m
 > 📄 [설치 스크립트 소스 보기](https://github.com/VincentZyuApps/winload/blob/main/docs/install_scripts/install.sh)[m
 [m
 <details>[m
[31m-<summary>수동 설치 / 기타 플랫폼</summary>[m
[32m+[m[32m<summary>수동 설치</summary>[m[41m[m
 [m
 **DEB (Debian/Ubuntu):**[m
 ```bash[m
[1mdiff --git a/readme.md b/readme.md[m
[1mindex 750517e..dc8eaef 100644[m
[1m--- a/readme.md[m
[1m+++ b/readme.md[m
[36m@@ -43,7 +43,7 @@[m [mscoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket[m
 scoop install winload[m
 ```[m
 [m
[31m-**Arch Linux (AUR):**[m
[32m+[m[32m### Arch Linux (AUR):[m
 ```bash[m
 paru -S winload-rust-bin[m
 ```[m
[36m@@ -58,7 +58,7 @@[m [mcurl -fsSL https://raw.githubusercontent.com/VincentZyuApps/winload/main/docs/in[m
 > 📄 [View install script source](https://github.com/VincentZyuApps/winload/blob/main/docs/install_scripts/install.sh)[m
 [m
 <details>[m
[31m-<summary>Manual install / Other platforms</summary>[m
[32m+[m[32m<summary>Manual install</summary>[m
 [m
 **DEB (Debian/Ubuntu):**[m
 ```bash[m
[1mdiff --git a/readme.zh-cn.md b/readme.zh-cn.md[m
[1mindex 2aefbc1..642eec2 100644[m
[1m--- a/readme.zh-cn.md[m
[1m+++ b/readme.zh-cn.md[m
[36m@@ -42,7 +42,7 @@[m [mscoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket[m
 scoop install winload[m
 ```[m
 [m
[31m-**Arch Linux (AUR):**[m
[32m+[m[32m### Arch Linux (AUR):[m
 ```bash[m
 paru -S winload-bin[m
 ```[m
[36m@@ -57,7 +57,7 @@[m [mcurl -fsSL https://raw.githubusercontent.com/VincentZyuApps/winload/main/docs/in[m
 > 📄 [查看安装脚本源码](https://github.com/VincentZyuApps/winload/blob/main/docs/install_scripts/install.sh)[m
 [m
 <details>[m
[31m-<summary>手动安装 / 其他平台</summary>[m
[32m+[m[32m<summary>手动安装</summary>[m
 [m
 **DEB (Debian/Ubuntu):**[m
 ```bash[m
[1mdiff --git a/readme.zh-tw.md b/readme.zh-tw.md[m
[1mindex a58bb2a..0aeb9ee 100644[m
[1m--- a/readme.zh-tw.md[m
[1m+++ b/readme.zh-tw.md[m
[36m@@ -42,7 +42,7 @@[m [mscoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket[m
 scoop install winload[m
 ```[m
 [m
[31m-**Arch Linux (AUR):**[m
[32m+[m[32m### Arch Linux (AUR):[m
 ```bash[m
 paru -S winload-bin[m
 ```[m
[36m@@ -55,7 +55,7 @@[m [mcurl -fsSL https://raw.githubusercontent.com/VincentZyuApps/winload/main/docs/in[m
 ```[m
 [m
 <details>[m
[31m-<summary>手動安裝 / 其他平台</summary>[m
[32m+[m[32m<summary>手動安裝</summary>[m
 [m
 **DEB (Debian/Ubuntu):**[m
 ```bash[m
