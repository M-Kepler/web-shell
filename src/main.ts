import { invoke } from "@tauri-apps/api/core";
import { register, unregisterAll } from "@tauri-apps/plugin-global-shortcut";
import { Window } from "@tauri-apps/api/window";
import { enable, disable } from "@tauri-apps/plugin-autostart";
import { exists, writeTextFile, readTextFile } from "@tauri-apps/plugin-fs";
import { appConfigDir, appDataDir } from "@tauri-apps/api/path";

// 配置接口
interface Config {
  app: {
    name: string;
    version: string;
  };
  window: {
    title: string;
    width: number;
    height: number;
    resizable: boolean;
    fullscreen: boolean;
    decorations: boolean;
  };
  web: {
    url: string;
    title: string;
    allow: string;
  };
  hotkey: {
    show: string;
    hide: string;
  };
  autostart: {
    enabled: boolean;
    promptOnFirstRun: boolean;
  };
  tray: {
    enabled: boolean;
    menu: {
      show: string;
      autostart: string;
      autostartDisable: string;
      quit: string;
    };
  };
}

// 默认配置
const defaultConfig: Config = {
  app: {
    name: "Web Shell",
    version: "1.0.0",
  },
  window: {
    title: "Web Shell",
    width: 400,
    height: 700,
    resizable: true,
    fullscreen: false,
    decorations: false,
  },
  web: {
    url: "https://www.kimi.com/",
    title: "Kimi Chat",
    allow: "clipboard-write; clipboard-read;",
  },
  hotkey: {
    show: "Alt+Space",
    hide: "Alt+Space",
  },
  autostart: {
    enabled: false,
    promptOnFirstRun: true,
  },
  tray: {
    enabled: true,
    menu: {
      show: "显示窗口",
      autostart: "开机自启",
      autostartDisable: "取消开机自启",
      quit: "退出",
    },
  },
};

// 全局配置变量
let config: Config = defaultConfig;

// 读取配置文件
async function loadConfig(): Promise<Config> {
  // 开发模式：从 public/config.json 读取
  if (window.location.protocol === "http:") {
    try {
      const response = await fetch("/config.json");
      if (response.ok) {
        const content = await response.text();
        const loadedConfig = JSON.parse(content) as Config;
        // 合并配置，确保所有字段都存在
        return {
          ...defaultConfig,
          ...loadedConfig,
          app: { ...defaultConfig.app, ...loadedConfig.app },
          window: { ...defaultConfig.window, ...loadedConfig.window },
          web: { ...defaultConfig.web, ...loadedConfig.web },
          hotkey: { ...defaultConfig.hotkey, ...loadedConfig.hotkey },
          autostart: { ...defaultConfig.autostart, ...loadedConfig.autostart },
          tray: {
            ...defaultConfig.tray,
            ...loadedConfig.tray,
            menu: { ...defaultConfig.tray.menu, ...loadedConfig.tray?.menu },
          },
        };
      }
    } catch (error) {
      console.log(
        "Failed to load config from public/config.json, using default:",
        error
      );
    }
  }

  // 生产模式：从应用配置目录读取
  try {
    const configDir = await appConfigDir();
    const configPath = configDir + "config.json";
    const content = await readTextFile(configPath);
    const loadedConfig = JSON.parse(content) as Config;

    // 合并配置，确保所有字段都存在
    return {
      ...defaultConfig,
      ...loadedConfig,
      app: { ...defaultConfig.app, ...loadedConfig.app },
      window: { ...defaultConfig.window, ...loadedConfig.window },
      web: { ...defaultConfig.web, ...loadedConfig.web },
      hotkey: { ...defaultConfig.hotkey, ...loadedConfig.hotkey },
      autostart: { ...defaultConfig.autostart, ...loadedConfig.autostart },
      tray: {
        ...defaultConfig.tray,
        ...loadedConfig.tray,
        menu: { ...defaultConfig.tray.menu, ...loadedConfig.tray?.menu },
      },
    };
  } catch (error) {
    console.log("Config file not found or error, using default config:", error);
    return defaultConfig;
  }
}

// 检查并设置开机自启
async function checkAndSetAutostart() {
  if (!config.autostart.enabled) {
    return;
  }

  try {
    const dir = await appDataDir();
    const flagPath = dir + "autostart.flag";
    const existsFlag = await exists(flagPath);
    if (existsFlag) {
      await enable();
      // 删除标志文件
      await writeTextFile(flagPath, "");
    }
  } catch (e) {
    // 忽略错误
  }
}

// 检查首次运行并询问用户
async function checkFirstRunAndPrompt() {
  if (!config.autostart.promptOnFirstRun) {
    return;
  }

  const configDir = await appConfigDir();
  const flagPath = configDir + "autostart_prompted.flag";
  const prompted = await exists(flagPath);

  if (!prompted) {
    // 你可以用更美观的 UI 组件替换 window.confirm
    const shouldAutostart = window.confirm("是否开机自启？（推荐，默认勾选）");
    if (shouldAutostart) {
      await enable();
    } else {
      await disable();
    }
    await writeTextFile(flagPath, "1");
  }
}

// 更新 iframe 配置
function updateIframeConfig() {
  const iframe = document.getElementById("web-frame") as HTMLIFrameElement;
  if (iframe) {
    iframe.src = config.web.url;
    iframe.setAttribute("allow", config.web.allow);
  }
}

// 注册全局快捷键
async function registerHotkey() {
  try {
    await unregisterAll();
    await register(config.hotkey.show, async (event: { state: string }) => {
      if (event.state === "Pressed") {
        const win = await Window.getByLabel("main");
        if (win) {
          await win.show();
          await win.setFocus();
        }
      }
    });
    console.log(`Hotkey registered: ${config.hotkey.show}`);
  } catch (error) {
    console.error("Failed to register hotkey:", error);
  }
}

// 尝试向 iframe 注入 CSS（受跨域限制）
function injectCSS() {
  const iframe = document.getElementById("web-frame") as HTMLIFrameElement;
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
}

// 处理问候表单
function handleGreetForm() {
  const greetInputEl = document.querySelector(
    "#greet-input"
  ) as HTMLInputElement;
  const greetMsgEl = document.querySelector("#greet-msg") as HTMLElement;
  const form = document.querySelector("#greet-form") as HTMLFormElement;

  if (form && greetInputEl && greetMsgEl) {
    form.addEventListener("submit", async (e) => {
      e.preventDefault();
      // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
      greetMsgEl.textContent = await invoke("greet", {
        name: greetInputEl.value,
      });
    });
  }
}

// 初始化
async function init() {
  // 加载配置
  config = await loadConfig();

  // 更新 iframe 配置
  updateIframeConfig();

  // 检查并设置开机自启
  await checkAndSetAutostart();

  // 检查首次运行
  await checkFirstRunAndPrompt();

  // 注册快捷键
  await registerHotkey();

  // 尝试注入 CSS
  injectCSS();

  // 处理表单
  handleGreetForm();
}

// 页面加载完成后初始化
window.addEventListener("DOMContentLoaded", () => {
  init();
});
