import { invoke } from "@tauri-apps/api/core";

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
});
