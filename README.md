# CC Switch Web

<p align="center">
  <strong>Headless Web Management Console for CC Switch — Tailored for Linux Servers, Cloud VPS, and WSL2</strong>
</p>

<p align="center">
  English | <a href="README_ZH.md">简体中文</a> | <a href="README_JA.md">日本語</a> | <a href="README_DE.md">Deutsch</a>
</p>

<p align="center">
  <a href="#-why-cc-switch-web">Why Web?</a> •
  <a href="#-architecture--how-it-works">Architecture</a> •
  <a href="#-features">Features</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#%EF%B8%8F-cli-options">CLI Options</a> •
  <a href="#%EF%B8%8F-production-deployment">Production</a> •
  <a href="#-faq">FAQ</a> •
  <a href="#-acknowledgements--license">License</a>
</p>

---

## 💡 Why CC Switch Web?

[CC Switch](https://github.com/farion1231/cc-switch) is a powerful provider and configuration manager for AI coding assistants (Claude Code, Codex, Gemini CLI, Grok Build, etc.).

However, the official desktop application is built with Tauri 2 and depends on system graphical libraries (such as WebKitGTK on Linux):
* **Cannot run on headless servers**: On remote cloud Linux servers, VPS instances, Docker containers, or WSL2 without a GUI, the desktop app fails to start due to missing display and graphical library dependencies;
* **Terminal CLI limitations**: While community terminal-based TUI tools are lightweight, they cannot deliver the visual richness of a web console for tasks like multi-provider side-by-side comparison, real-time request log monitoring, token usage visual analytics, and skill package management.

**CC Switch Web** bridges this gap: **it decouples the application from desktop windowing and WebKitGTK dependencies, while preserving the complete official Rust backend logic and React frontend console**. By running a lightweight web daemon on your server, you can enjoy the exact same visual management experience right inside your local web browser!

---

## 🏗️ Architecture & How It Works

CC Switch Web adopts a security-first design that **completely separates the control plane from the data plane**:

```text
┌─────────────────────────────────────────────────────────────┐
│                 Remote Server (Headless Linux / VPS)        │
│                                                             │
│   [ Claude Code / Codex / Gemini CLI / Grok Build ... ]     │
│             │                                               │
│             │ (Model requests)                              │
│             ▼                                               │
│   [ Local Model Router ] 127.0.0.1:15721                    │
│             │                                               │
│             │ (Auto failover / request rectifier / rewrite) │
│             ▼                                               │
│     Upstream Providers (Claude / OpenAI / Gemini ...)       │
│                                                             │
│ ─────────────────────────────────────────────────────────── │
│                                                             │
│   [ CC Switch Web Daemon ]                                  │
│      ├── SQLite DB: ~/.cc-switch/cc-switch.db               │
│      ├── CLI tool configs: ~/.claude.json, ~/.codex, etc.   │
│      ├── Persistent auth token: ~/.cc-switch/web-token      │
│      └── Web management port: 127.0.0.1:3927 (Loopback)     │
└──────────────────────────────▲──────────────────────────────┘
                               │
                               │ SSH Local Port Forwarding (Recommended)
                               │ ssh -N -L 3927:127.0.0.1:3927 user@server
                               │
┌──────────────────────────────┴──────────────────────────────┐
│                    Your Local PC                            │
│                                                             │
│   [ Modern Browser ] ───▶ http://127.0.0.1:3927/#token=... │
│   (Pure control plane: providers, MCP, skills, analytics)   │
└─────────────────────────────────────────────────────────────┘
```

### 🔒 Core Security Principles
1. **Credentials & Traffic Stay on the Server**: API keys, OAuth session tokens, SQLite databases, and high-concurrency model traffic never leave the server. The browser only sends lightweight management commands.
2. **Loopback Isolation by Default**: Both the web management port (default `3927`) and the local proxy router port (`15721`) strictly listen on `127.0.0.1` by default, eliminating accidental exposure to the public internet.
3. **Persistent Token Authentication**: On first startup, a secure random token is generated and stored in `~/.cc-switch/web-token` (with `0600` permissions). Once authenticated via `#token=...`, the browser persists the session for seamless future visits.

---

## ✨ Features

- 🖥️ **Full Desktop UI Experience**: Identical React frontend to the desktop app — manage providers, MCP servers, Skills market, Prompts library, session history, and token usage analytics.
- ⚡ **Multi-Tool Support**: Hot-switch configurations seamlessly across Claude Code, Codex, Gemini CLI, Grok Build, OpenCode, OpenClaw, Hermes Agent, Pi, and more.
- 🔄 **Resilient Routing & Conversion**: Built-in hot switching, automated failover queues, and model request rectification.
- 🐧 **Engineered for Headless Environments**:
  - Completely stripped of WebKitGTK / GTK3 dependencies with minimal memory overhead;
  - Native file/directory dialogs automatically replaced with server filesystem path inputs;
  - Native clipboard operations seamlessly bridged to the browser Web Clipboard API;
  - External documentation links open automatically in your local browser;
  - Built-in vendored OpenSSL to prevent compilation failures on minimal server OS installs without `pkg-config` / `libssl-dev`.
- 📦 **Dual-Target Codebase**: The `desktop` feature flag is fully preserved; the same codebase can still be compiled into a native desktop app on systems with a GUI.

---

## 🚀 Quick Start

### 1. Prerequisites
Ensure your server has the following installed:
* **Node.js** (>= 18) and **pnpm**
* **Rust** (>= 1.85) and **Cargo**

### 2. Build

Run the following in the repository root:

```bash
# 1. Install frontend dependencies
pnpm install

# 2. Build the release bundle (compiles frontend static assets + Rust web binary)
pnpm build:web
```

> **Step-by-step debug build (optional)**:
> ```bash
> # Build frontend static assets into ./dist
> pnpm build:renderer
> 
> # Build debug web binary
> cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --features web --bin cc-switch-web
> ```

### 3. Run

```bash
./src-tauri/target/release/cc-switch-web --port 3927 --dist ./dist
```

Upon starting, you will see output similar to:
```text
CC Switch 网页已启动
  本机地址: http://127.0.0.1:3927/
  第一次打开: http://127.0.0.1:3927/#token=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
  令牌文件: /home/username/.cc-switch/web-token
  页面目录: /path/to/dist
  停止: Ctrl+C
```

### 4. Access the Console

#### Option A: Remote Server via SSH Tunnel (Recommended)
In a terminal on your **local machine**, establish an SSH local port forwarding tunnel:
```bash
ssh -N -L 3927:127.0.0.1:3927 <username>@<server-ip>
```
Then open the initialization URL displayed in the server terminal:
```text
http://127.0.0.1:3927/#token=<your-token>
```
After the browser saves the token, you can simply visit `http://127.0.0.1:3927/` directly.

#### Option B: Local Machine / WSL2 Direct Access
Open `http://127.0.0.1:3927/#token=<your-token>` directly in your browser.

---

## 🛠️ CLI Options

`cc-switch-web` supports the following command-line flags:

| Option | Default | Environment Variable | Description |
| :--- | :--- | :--- | :--- |
| `--host <IP>` | `127.0.0.1` | - | IP address to bind to. For security reasons, loopback binding is strongly recommended. |
| `--port <PORT>` | `3927` | - | Port for the web management console. |
| `--dist <PATH>` | `./dist` | `CC_SWITCH_WEB_DIST` | Path to frontend static build directory (must contain `index.html`). |
| `-h, --help` | - | - | Print help information and exit. |

---

## ⚙️ Production Deployment

To keep the service running continuously in the background and start automatically on boot, manage it with `systemd`.

### 1. Place Binary and Static Files
Install the artifacts to a standard directory (e.g., `/opt/cc-switch-web`):
```bash
sudo mkdir -p /opt/cc-switch-web
sudo cp ./src-tauri/target/release/cc-switch-web /opt/cc-switch-web/
sudo cp -r ./dist /opt/cc-switch-web/
```

### 2. Create Systemd Service File
Create `/etc/systemd/system/cc-switch-web.service` (replace `your_user` with your actual Linux user):

```ini
[Unit]
Description=CC Switch Web Service
After=network.target

[Service]
Type=simple
User=your_user
Group=your_user
WorkingDirectory=/opt/cc-switch-web
ExecStart=/opt/cc-switch-web/cc-switch-web --port 3927 --dist /opt/cc-switch-web/dist
Restart=always
RestartSec=5
# Ensure HOME is defined for user configuration paths
Environment=HOME=/home/your_user

[Install]
WantedBy=multi-user.target
```

### 3. Enable and Start Service
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now cc-switch-web

# Check status
sudo systemctl status cc-switch-web
```

---

## ❓ FAQ

<details>
<summary><strong>Q: Where is the access token stored? How do I reset it?</strong></summary>

The token is persistently stored in `~/.cc-switch/web-token`.
To reset the token:
1. Stop the `cc-switch-web` process;
2. Delete the token file: `rm ~/.cc-switch/web-token`;
3. Restart the service. A new secure random token will be generated and printed to stdout.
</details>

<details>
<summary><strong>Q: Can I bind <code>--host 0.0.0.0</code> for direct public access?</strong></summary>

**Exposing the plain HTTP port to the public internet is strongly discouraged!**
While the web console requires a token, plain HTTP exposes tokens and sensitive requests to interceptors, potentially compromising your API keys and configs.
If you need remote access without SSH tunneling, consider:
1. **SSH Tunneling** (Recommended): `ssh -N -L 3927:127.0.0.1:3927 user@server`;
2. **VPN / Overlay Network**: Connect via Tailscale, WireGuard, or Cloudflare Tunnels;
3. **Nginx Reverse Proxy**: Terminate TLS/HTTPS with a valid SSL certificate and configure HTTP Basic Auth or IP whitelisting.
</details>

<details>
<summary><strong>Q: Where are configurations and databases stored?</strong></summary>

Exactly the same as the official desktop version:
* **SQLite Database**: `~/.cc-switch/cc-switch.db`
* **Device Settings**: `~/.cc-switch/settings.json`
* **Web Access Token**: `~/.cc-switch/web-token`
* **CLI Tool Configs**: `~/.claude/`, `~/.codex/`, `~/.gemini/`, etc.
</details>

<details>
<summary><strong>Q: Can I still build the native desktop app from this repository?</strong></summary>

Yes. The native desktop functionality is preserved under the `desktop` feature flag. On a system with graphical dependencies installed (WebKitGTK and GTK3):
```bash
pnpm install
pnpm dev      # Run desktop app in development
pnpm build    # Build desktop distribution packages
```
</details>

---

## 📄 Acknowledgements & License

* This repository is a derivative of [CC Switch](https://github.com/farion1231/cc-switch), customized for headless server environments. **It is not an official project.**
* Original project copyright belongs to **Jason Young (2025)** under the **[MIT License](LICENSE)**.
* For the official desktop release, official documentation, and sponsorship details, please visit the upstream repository: [farion1231/cc-switch](https://github.com/farion1231/cc-switch) and official website [ccswitch.io](https://ccswitch.io).
