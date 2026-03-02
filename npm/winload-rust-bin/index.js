/**
 * winload-rust-bin — binary path resolver
 *
 * npm 安装时会根据 optionalDependencies 中各平台包的 os/cpu 字段，
 * 仅下载与当前平台匹配的那一个。此模块负责定位该平台包中的二进制。
 *
 * 原理同 esbuild / @biomejs/biome / turbo 等项目。
 */

"use strict";

const path = require("path");

/**
 * 平台映射表
 * key:   `${process.platform}-${process.arch}`
 * value: npm 平台包名
 */
const PLATFORMS = {
  "win32-x64":    "winload-rust-bin-win32-x64",
  "win32-arm64":  "winload-rust-bin-win32-arm64",
  "linux-x64":    "winload-rust-bin-linux-x64",
  "linux-arm64":  "winload-rust-bin-linux-arm64",
  "darwin-x64":   "winload-rust-bin-darwin-x64",
  "darwin-arm64":  "winload-rust-bin-darwin-arm64",
};

/**
 * 获取当前平台对应的 winload 二进制绝对路径
 * @returns {string} 二进制路径
 * @throws {Error} 不支持的平台 / 平台包未安装
 */
function getBinaryPath() {
  const key = `${process.platform}-${process.arch}`;
  const pkg = PLATFORMS[key];

  if (!pkg) {
    const supported = Object.keys(PLATFORMS).join(", ");
    throw new Error(
      `winload: unsupported platform "${key}"\n` +
      `Supported: ${supported}\n` +
      `Download manually: https://github.com/VincentZyuApps/winload/releases`
    );
  }

  try {
    const pkgDir = path.dirname(require.resolve(`${pkg}/package.json`));
    const ext = process.platform === "win32" ? ".exe" : "";
    return path.join(pkgDir, "bin", `winload${ext}`);
  } catch {
    throw new Error(
      `winload: platform package "${pkg}" not found\n` +
      `Try reinstalling: npm install winload-rust-bin\n` +
      `Or download manually: https://github.com/VincentZyuApps/winload/releases`
    );
  }
}

module.exports = { getBinaryPath, PLATFORMS };
