import { invoke } from "@tauri-apps/api/core";
import { register } from '@tauri-apps/plugin-global-shortcut';
import { Window } from '@tauri-apps/api/window';
import { enable, disable } from '@tauri-apps/plugin-autostart';
import { invoke as tauriInvoke } from '@tauri-apps/api/tauri';
import { exists, writeTextFile } from '@tauri-apps/api/fs';
import { appConfigDir } from '@tauri-apps/api/path';

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

async function checkAndSetAutostart() {
  try {
    // 检查 autostart.flag 是否存在
    const exists = await tauriInvoke('plugin:fs|exists', { path: 'autostart.flag' });
    if (exists) {
      await enable();
      // 删除标志文件
      await tauriInvoke('plugin:fs|removeFile', { path: 'autostart.flag' });
    }
  } catch (e) {
    // 忽略错误
  }
}

async function checkFirstRunAndPrompt() {
  const configDir = await appConfigDir();
  const flagPath = configDir + 'autostart_prompted.flag';
  const prompted = await exists(flagPath);

  if (!prompted) {
    // 你可以用更美观的 UI 组件替换 window.confirm
    const shouldAutostart = window.confirm('是否开机自启？（推荐，默认勾选）');
    if (shouldAutostart) {
      await enable();
    } else {
      await disable();
    }
    await writeTextFile(flagPath, '1');
  }
}

checkAndSetAutostart();
checkFirstRunAndPrompt();

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  // 注入自定义 CSS 到 Deepseek Chat iframe
  const iframe = document.getElementById("deepseek-chat-frame") as HTMLIFrameElement;
  if (iframe) {
    iframe.addEventListener("load", () => {
      try {
        const style = document.createElement("style");
        style.innerHTML = `
          header, .header, .top-bar { display: none !important; }
          /* 这里可以添加更多自定义样式 */
        `;
        // 尝试注入 CSS
        iframe.contentWindow?.document.head.appendChild(style);
      } catch (e) {
        // 由于跨域限制，可能无法直接注入
        console.warn("无法注入自定义 CSS，可能由于跨域限制。", e);
      }
    });
  }

  // 注册 Alt+Space 全局快捷键，显示主窗口并聚焦
  register('Alt+Space', async (event: { state: string }) => {
    if (event.state === "Pressed") {
      const win = await Window.getByLabel('main');
      if (win) {
        await win.show();
        await win.setFocus();
      }
      // 由于安全策略，无法自动聚焦网页内输入框
    }
  });
});
