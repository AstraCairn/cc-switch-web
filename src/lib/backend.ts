import { invoke as tauriInvoke, isTauri } from "@tauri-apps/api/core";
import type { UnlistenFn } from "@tauri-apps/api/event";

export type { UnlistenFn };

const TOKEN_STORAGE_KEY = "cc-switch-web-token";

type EventHandler = (event: { payload: unknown }) => void;

const listeners = new Map<string, Set<EventHandler>>();
let socket: WebSocket | null = null;
let socketPromise: Promise<void> | null = null;

export function isDesktopShell(): boolean {
  return typeof isTauri === "function" && isTauri();
}

function tokenStore(): Storage | null {
  try {
    return window.localStorage;
  } catch {
    return null;
  }
}

function readToken(): string {
  const store = tokenStore();
  const stored = store?.getItem(TOKEN_STORAGE_KEY) ?? "";
  if (stored) return stored;
  const hash = new URLSearchParams(window.location.hash.replace(/^#/, ""));
  const token = hash.get("token") ?? "";
  if (!token) return "";
  store?.setItem(TOKEN_STORAGE_KEY, token);
  hash.delete("token");
  const rest = hash.toString();
  window.history.replaceState(
    null,
    "",
    `${window.location.pathname}${window.location.search}${rest ? `#${rest}` : ""}`,
  );
  return token;
}

async function ensureEvents(): Promise<void> {
  if (socket?.readyState === WebSocket.OPEN) return;
  if (socketPromise) return socketPromise;
  socketPromise = new Promise((resolve, reject) => {
    const token = readToken();
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const next = new WebSocket(
      `${protocol}//${window.location.host}/api/v1/events?token=${encodeURIComponent(token)}`,
    );
    next.addEventListener("open", () => {
      socket = next;
      resolve();
    });
    next.addEventListener("error", () => {
      socketPromise = null;
      reject(new Error("无法连接事件通道"));
    });
    next.addEventListener("message", (message) => {
      if (typeof message.data !== "string") return;
      const parsed = JSON.parse(message.data) as {
        event?: string;
        payload?: unknown;
      };
      if (!parsed.event) return;
      listeners.get(parsed.event)?.forEach((handler) => {
        handler({ payload: parsed.payload });
      });
    });
    next.addEventListener("close", () => {
      if (socket === next) socket = null;
      socketPromise = null;
    });
  });
  return socketPromise;
}

export async function invoke<T>(
  command: string,
  args: Record<string, unknown> = {},
): Promise<T> {
  if (isDesktopShell()) {
    return tauriInvoke<T>(command, args);
  }

  if (command === "open_external") {
    const url = String(args.url ?? "");
    if (url) window.open(url, "_blank", "noopener,noreferrer");
    return true as T;
  }
  if (command === "copy_text_to_clipboard") {
    await navigator.clipboard.writeText(String(args.text ?? ""));
    return true as T;
  }
  if (command === "pick_directory") {
    const picked = window.prompt(
      "填写服务器上的目录路径",
      String(args.defaultPath ?? ""),
    );
    return (picked && picked.trim() ? picked.trim() : null) as T;
  }
  if (command === "save_file_dialog") {
    const picked = window.prompt(
      "保存到服务器上的路径",
      String(args.defaultName ?? "export.sql"),
    );
    return (picked && picked.trim() ? picked.trim() : null) as T;
  }
  if (
    command === "open_file_dialog" ||
    command === "open_zip_file_dialog"
  ) {
    const picked = window.prompt(
      command === "open_zip_file_dialog"
        ? "服务器上的 zip 或 skill 文件路径"
        : "服务器上的文件路径",
      "",
    );
    return (picked && picked.trim() ? picked.trim() : null) as T;
  }

  const token = readToken();
  const response = await fetch(`/api/v1/invoke/${command}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify(args ?? {}),
  });
  const text = await response.text();
  if (response.status === 401) {
    throw new Error("WEB_UNAUTHORIZED");
  }
  if (!response.ok) {
    const message = text || `调用 ${command} 失败`;
    if (message.startsWith("OPEN_URL:")) {
      window.open(message.slice("OPEN_URL:".length), "_blank", "noopener,noreferrer");
      return undefined as T;
    }
    throw new Error(message);
  }
  if (!text) return undefined as T;
  return JSON.parse(text) as T;
}

export async function listen<T>(
  event: string,
  handler: (event: { payload: T }) => void,
): Promise<UnlistenFn> {
  if (isDesktopShell()) {
    const { listen: tauriListen } = await import("@tauri-apps/api/event");
    return tauriListen<T>(event, handler);
  }

  await ensureEvents();
  const wrapped: EventHandler = (payload) => {
    handler(payload as { payload: T });
  };
  const set = listeners.get(event) ?? new Set<EventHandler>();
  set.add(wrapped);
  listeners.set(event, set);
  return () => {
    set.delete(wrapped);
  };
}
