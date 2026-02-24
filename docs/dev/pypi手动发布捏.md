# PyPI 手动发布流程指南

> 使用 uv 构建并发布 Python 包到 PyPI
>
> ⚠️ 本文档中 `$VERSION` 代表当前要发布的版本号（如 `0.1.6-beta.3`）。

---

## 📋 发布前准备

### 1. 确认版本号
```bash
# 从 pyproject.toml 读取版本
wsl
grep '^version' py/pyproject.toml
# 例: version = "0.1.6-beta.3"
```
> 🔔 以下所有操作都用这个版本号替换 `${VERSION}`。

### 2. 安装 uv（如果还没安装）
```bash
# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -c "irm https://gitee.com/wangnov/uv-custom/releases/download/0.10.5/uv-installer-custom.ps1 | iex"

# macOS / Linux
curl -LsSf https://gitee.com/wangnov/uv-custom/releases/download/0.10.5/uv-installer-custom.sh | sh

```

### 3. 安装构建依赖
```bash
cd py
uv sync
```

---

## 🔨 构建包

### 使用 uv 构建（推荐）
```bash
cd py

# 构建 wheel 和源码包
uv build

# 查看产物
ls dist/
```

---

## 📤 发布到 PyPI

### 方式 A: 使用 uv publish（推荐）

```bash
cd py

# 发布到 TestPyPI（测试）
# uv publish --index https://test.pypi.org/simple/
uv publish --publish-url https://test.pypi.org/legacy/

# 测试安装
cd D:\aaaStuffsaaa\from_git\test\20260224_test_winload
uv venv
uv pip install --index-url https://test.pypi.org/simple/ winload
uv run py winload --help

# 发布到正式 PyPI
uv publish
```

### 方式 B: 使用 twine

```bash
cd py

# 安装 twine
pip install twine

# 发布到 TestPyPI（测试）
twine upload --repository testpypi dist/*

# 测试安装
pip install --index-url https://test.pypi.org/simple/ winload

# 发布到正式 PyPI
twine upload dist/*
```

---

## 🧪 测试安装

### 从 TestPyPI 安装
```bash
pip install --index-url https://test.pypi.org/simple/ winload
winload --help
```

### 从正式 PyPI 安装
```bash
pip install winload
winload --help
```

---

## 📝 发布检查清单

每次发布新版本时：
- [ ] 更新 `py/pyproject.toml` 中的版本号
- [ ] 构建包：`cd py && uv build`
- [ ] 发布到 TestPyPI 并测试安装
- [ ] 发布到正式 PyPI
- [ ] 验证安装：`pip install winload && winload --version`

---

## 🤖 GitHub Actions 自动化（后续）

后续可以添加 CI 自动发布到 PyPI，触发方式和 AUR 类似：

| Commit message | 构建 | Release | PyPI |
|----------------|------|---------|------|
| `build action` | ✅ | ❌ | ❌ |
| `build release` | ✅ | ✅ | ❌ |
| `build publish` | ✅ | ✅ | ✅ |
| `publish from release` | ❌ | ❌ | ✅ |

### 自动化流程示例

```yaml
# .github/workflows/publish-pypi.yml
name: Publish to PyPI

on:
  push:
    branches: [main]
  pull_request:

permissions:
  contents: read

jobs:
  check:
    runs-on: ubuntu-latest
    outputs:
      should_publish: ${{ steps.flags.outputs.should_publish }}
      version: ${{ steps.flags.outputs.version }}
    steps:
      - uses: actions/checkout@v4
      - id: flags
        run: |
          MSG="${{ github.event.head_commit.message }}"
          VERSION="v$(grep '^version' py/pyproject.toml | sed 's/.*"\(.*\)".*/\1/')"
          echo "version=$VERSION" >> "$GITHUB_OUTPUT"
          
          if echo "$MSG" | grep -qi "build publish"; then
            echo "should_publish=true" >> "$GITHUB_OUTPUT"
          elif echo "$MSG" | grep -qi "publish from release"; then
            echo "should_publish=true" >> "$GITHUB_OUTPUT"
          fi

  build:
    needs: check
    if: needs.check.outputs.should_publish == 'true'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install uv
        uses: astral-sh/setup-uv@v4
      
      - name: Build package
        working-directory: py
        run: uv build
      
      - name: Publish to PyPI
        working-directory: py
        env:
          UV_PUBLISH_TOKEN: ${{ secrets.PYPI_API_TOKEN }}
        run: uv publish

  # 或者使用 twine + PYPI_TOKEN
  publish:
    needs: [check, build]
    if: needs.check.outputs.should_publish == 'true'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Download build artifacts
        uses: actions/download-artifact@v4
        with:
          name: dist
          path: dist/
      
      - name: Publish to PyPI
        uses: pypa/gh-action-pypi-publish@release/v1
        with:
          password: ${{ secrets.PYPI_TOKEN }}
```

### 配置 PyPI API Token

1. 访问 https://pypi.org/manage/account/token/
2. 创建一个 Token，复制
3. 到 GitHub 仓库 → Settings → Secrets and variables → Actions
4. 添加 secret:
   - **Name**: `PYPI_API_TOKEN` 或 `PYPI_TOKEN`
   - **Value**: 粘贴 token

---

## 📌 用户安装方式

```bash
# 方式 1: pip
pip install winload

# 方式 2: uv
uv add winload
```

---

## ⚠️ 常见问题

| 问题 | 解决 |
|------|------|
| `ModuleNotFoundError: No module named 'hatchling'` | 确保使用 `uv build`，它会自动安装构建依赖 |
| `error: invalid command 'bdist_wheel'` | 使用 `uv build` 而不是 `python setup.py` |
| 发布失败，提示权限错误 | 检查 PyPI API Token 是否正确 |
| 包名已存在 | 更改 `pyproject.toml` 中的 `name` 字段 |
