# DeepSeek Desktop (DeepSeek-Shell)

> 中文版说明请看 [README-CN.md](./README-CN.md)

## Project Overview

**DeepSeek Desktop (DeepSeek-Shell)** is a minimal, efficient, and lightweight (only ~8MB) desktop client for DeepSeek. Inspired by the convenience of Doubao Desktop, users can summon a compact chat window anytime with the `Alt+Space` shortcut to quickly interact with DeepSeek Chat.

This project is built with [Tauri](https://tauri.app/) + [Vite](https://vitejs.dev/), featuring:

- **Tiny Size**: Installer is only about 8MB, with extremely low resource usage for fast download and startup.
- **Minimal Desktop Client**: Instantly summon the app with a shortcut, no need to switch windows, ask questions anytime.
- **DeepSeek Power**: Directly leverages DeepSeek Chat's powerful AI via the web, supporting native account login.
- **UI Adaptation**: Designed for small window scenarios, with a clean interface and efficient interactions.
- **System Tray Support**: Minimize to tray, tray menu, and easy background access.
- **Cross-Platform**: Supports Windows, macOS, and Linux.
- **Secure & Reliable**: Local shell connects to official DeepSeek services, ensuring data security.

The goal is to provide DeepSeek users with a Doubao-like desktop experience, but with stronger AI, better UI adaptation, and a much smaller footprint.

![1751031319359](image/README/1751031319359.png)

---

## Download & Usage

- **Download**: [Click here for the latest installer](https://github.com/benhack20/deepseek-shell/releases/download/v1.0.0/deepseek-shell_0.1.0_x64-setup.exe)
- **Installation**:
  1. Download the installer for your platform (Windows provided; macOS/Linux can be built manually).
  2. Follow the prompts to complete installation.
- **Usage (ensure Alt+Space is not occupied)**:
  1. After installation, run DeepSeek-Shell.
  2. Use `Alt+Space` to **show or hide** the chat window at any time.
  3. Closing the window minimizes it to the system tray.
  4. Right-click the tray icon to set auto-start on boot.
  5. Log in to your DeepSeek account to start chatting with AI.

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
