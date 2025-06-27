# Deepseek Shell 桌面端开发说明

本项目基于 [Tauri](https://tauri.app/) + [Vite](https://vitejs.dev/) 构建，支持跨平台桌面端开发。

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
- **权限问题**：有时需要用管理员权限运行命令行。

---

## 7. 官方文档

- [Tauri 官方文档](https://tauri.app/v1/guides/getting-started/prerequisites/)
- [Vite 官方文档](https://vitejs.dev/guide/)
