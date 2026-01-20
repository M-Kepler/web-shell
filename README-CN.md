# Web Shell (通用桌面壳应用)


## 安装包与使用方式

- **下载地址**：[点击下载最新版安装包](https://gitee.com/xxdxxdxxd/web-shell/releases/tag/v1.0.0)
- **安装方式**：

  1. 下载对应平台的安装包（Windows安装包已提供，macOS、Linux未提供，可自行编译）。
  2. 按照提示完成安装。
- **使用方式（请确保alt+space快捷键未被占用）**：

  1. 安装完成后，运行 Web Shell 桌面端。
  2. 可通过 `Alt+Space` 快捷键随时**唤出或隐藏**窗口。
  3. 关闭窗口后自动收至系统托盘处。
  4. 托盘右键点击可设置开机启动。
  5. 窗口将打开配置的网站（默认：Kimi Chat）。

---

## 配置说明

Web Shell 使用 JSON 配置文件，位置如下：
- **Windows**: `%APPDATA%\com.lenovo.web-shell\config.json`
- **macOS**: `~/Library/Application Support/com.lenovo.web-shell/config.json`
- **Linux**: `~/.config/com.lenovo.web-shell/config.json`

### 配置选项

```json
{
  "app": {
    "name": "Web Shell",
    "version": "1.0.0"
  },
  "window": {
    "title": "Web Shell",
    "width": 400,
    "height": 700,
    "resizable": true,
    "fullscreen": false,
    "decorations": false
  },
  "web": {
    "url": "https://www.kimi.com/",
    "title": "Kimi Chat",
    "allow": "clipboard-write; clipboard-read;"
  },
  "hotkey": {
    "show": "Alt+Space",
    "hide": "Alt+Space"
  },
  "autostart": {
    "enabled": false,
    "promptOnFirstRun": true
  },
  "tray": {
    "enabled": true,
    "menu": {
      "show": "显示窗口",
      "autostart": "开机自启",
      "autostartDisable": "取消开机自启",
      "quit": "退出"
    }
  }
}
```

### 配置示例

**示例 1：打开 Kimi Chat**
```json
{
  "web": {
    "url": "https://www.kimi.com/",
    "title": "Kimi Chat"
  }
}
```

**示例 2：打开 ChatGPT**
```json
{
  "web": {
    "url": "https://chat.openai.com/",
    "title": "ChatGPT"
  }
}
```

**示例 3：打开 GitHub**
```json
{
  "web": {
    "url": "https://github.com/",
    "title": "GitHub"
  },
  "window": {
    "title": "GitHub",
    "width": 800,
    "height": 600
  }
}
```

**示例 4：自定义快捷键**
```json
{
  "hotkey": {
    "show": "Alt+Shift+S",
    "hide": "Alt+Shift+S"
  }
}
```

**注意**：配置文件会在首次运行时自动创建并使用默认值。修改配置文件后需要重启应用才能生效。

---

## 项目简介

**Web Shell（通用桌面壳应用）** 是一个极简、高效、极小体积（仅约 8MB）的通用桌面壳应用，可以打开任何网站。灵感来源于豆包桌面端的便捷体验，用户可通过 `Alt+Space` 快捷键随时唤出一个小巧的窗口，快速访问任何网络服务。

本项目基于 [Tauri](https://tauri.app/) + [Vite](https://vitejs.dev/) 构建，具备以下特点：

- **极小体积**：安装包仅约 8MB，资源占用极低，下载和启动都非常迅速。
- **通用网页访问**：通过配置 URL 可以打开任何网站，不限于特定服务。
- **可自定义快捷键**：配置你喜欢的快捷键来显示/隐藏窗口。
- **极简桌面端**：安装后可通过快捷键呼出，无需切换窗口，随时随地访问网络服务。
- **UI 适配**：专为小窗口场景设计，界面简洁，交互高效。
- **系统托盘支持**：支持最小化到系统托盘、托盘菜单等，便于后台常驻和快速访问。
- **跨平台支持**：支持 Windows、macOS、Linux 多平台。
- **安全可靠**：本地壳与官方网络服务对接，无需担心数据安全。

本项目目标是为任何网络服务提供灵活的桌面体验，拥有可自定义的配置和极小的体积。

![1751031319359](image/README/1751031319359.png)

---


## 1. 安装依赖

在项目根目录下运行（只需一次）：

```bash
npm install
```

---

## 2. 启动开发模式

开发时，Tauri 会自动启动前端和桌面端，支持热重载。

```bash
npm run tauri dev
```

- 这会打开一个桌面窗口，实时预览前端页面。
- 可以在 `src/` 里开发前端页面，`src-tauri/` 里开发 Rust 后端。

---

## 3. 运行前端（仅网页预览）

如果只想预览前端页面（不启动桌面端）：

```bash
npm run dev
```

- 这会用 Vite 启动本地开发服务器，通常访问 http://localhost:5173。

---

## 4. 打包发布

打包成可分发的桌面应用（.exe、.dmg、.AppImage等）：

```bash
npm run tauri build
```

- 生成的安装包在 `src-tauri/target/release/bundle/` 目录下。
- Windows 下会有 `.msi` 或 `.exe` 安装包。

---

## 5. 其他常用命令

- **清理构建缓存**
  ```bash
  npm run tauri clean
  ```
- **升级 Tauri 依赖**
  ```bash
  npm update
  ```

---

## 6. 常见问题

- **首次运行报错**：确保已安装 Rust 环境（Tauri 需要），可用 [Rust 官网](https://www.rust-lang.org/tools/install) 安装。
- **Windows 下依赖**：建议安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)。

---

## 7. 官方文档

- [Tauri 官方文档](https://tauri.app/v1/guides/getting-started/prerequisites/)
- [Vite 官方文档](https://vitejs.dev/guide/)
