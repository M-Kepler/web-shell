# Web Shell (通用桌面壳应用)

> 中文版说明请看 [README-CN.md](./README-CN.md)

## Project Overview

**Web Shell** is a minimal, efficient, and lightweight (only ~8MB) desktop shell application that can open any website. Inspired by the convenience of Doubao Desktop, users can summon a compact window anytime with the `Alt+Space` shortcut to quickly access any web service.

This project is built with [Tauri](https://tauri.app/) + [Vite](https://vitejs.dev/), featuring:

- **Tiny Size**: Installer is only about 8MB, with extremely low resource usage for fast download and startup.
- **Universal Web Access**: Open any website by configuring the URL, not limited to a specific service.
- **Customizable Hotkey**: Configure your preferred shortcut key to show/hide the window.
- **Minimal Desktop Client**: Instantly summon the app with a shortcut, no need to switch windows, access web services anytime.
- **UI Adaptation**: Designed for small window scenarios, with a clean interface and efficient interactions.
- **System Tray Support**: Minimize to tray, tray menu, and easy background access.
- **Cross-Platform**: Supports Windows, macOS, and Linux.
- **Secure & Reliable**: Local shell connects to official web services, ensuring data security.

The goal is to provide a flexible desktop experience for any web service, with customizable configuration and a much smaller footprint.

![1751031319359](image/README/1751031319359.png)

---

## Download & Usage

- **Download**: [Click here for the latest installer](https://github.com/benhack20/web-shell/releases/download/v1.0.0/web-shell_0.1.0_x64-setup.exe)
- **Installation**:
  1. Download the installer for your platform (Windows provided; macOS/Linux can be built manually).
  2. Follow the prompts to complete installation.
- **Usage (ensure Alt+Space is not occupied)**:
  1. After installation, run Web Shell.
  2. Use `Alt+Space` to **show or hide** the window at any time.
  3. Closing the window minimizes it to the system tray.
  4. Right-click the tray icon to set auto-start on boot.
  5. The window will open the configured website (default: Kimi Chat).

---

## Configuration

Web Shell uses a JSON configuration file located at:
- **Windows**: `%APPDATA%\com.lenovo.web-shell\config.json`
- **macOS**: `~/Library/Application Support/com.lenovo.web-shell/config.json`
- **Linux**: `~/.config/com.lenovo.web-shell/config.json`

### Configuration Options

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

### Configuration Examples

**Example 1: Open Kimi Chat**
```json
{
  "web": {
    "url": "https://www.kimi.com/",
    "title": "Kimi Chat"
  }
}
```

**Example 2: Open ChatGPT**
```json
{
  "web": {
    "url": "https://chat.openai.com/",
    "title": "ChatGPT"
  }
}
```

**Example 3: Open GitHub**
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

**Example 4: Custom Hotkey**
```json
{
  "hotkey": {
    "show": "Alt+Shift+S",
    "hide": "Alt+Shift+S"
  }
}
```

**Note**: The configuration file is automatically created on first run with default values. You can modify it and restart the application to apply changes.

---

## 1. Install Dependencies

In the project root directory, run (once):

```bash
npm install
```

---

## 2. Start Development Mode

During development, Tauri will auto-launch both frontend and desktop, with hot reload support.

```bash
npm run tauri dev
```

- This opens a desktop window for real-time preview.
- Develop frontend in `src/`, backend (Rust) in `src-tauri/`.

---

## 3. Run Frontend Only (Web Preview)

To preview only the frontend (without desktop shell):

```bash
npm run dev
```

- This starts a Vite dev server, usually at http://localhost:5173.

---

## 4. Build for Release

Package as a distributable desktop app (`.exe`, `.dmg`, `.AppImage`, etc.):

```bash
npm run tauri build
```

- Output is in `src-tauri/target/release/bundle/`.
- On Windows, you'll get `.msi` or `.exe` installers.

---

## 5. Other Useful Commands

- **Clean build cache**
  ```bash
  npm run tauri clean
  ```
- **Update Tauri dependencies**
  ```bash
  npm update
  ```

---

## 6. FAQ

- **First run error**: Make sure Rust is installed ([Rust official site](https://www.rust-lang.org/tools/install)), required by Tauri.
- **Windows dependencies**: It's recommended to install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

---

## 7. Official Docs

- [Tauri Documentation](https://tauri.app/v1/guides/getting-started/prerequisites/)
- [Vite Documentation](https://vitejs.dev/guide/)
