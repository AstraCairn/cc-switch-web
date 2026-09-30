# CC Switch Web

<p align="center">
  <strong>专为无图形界面的 Linux 服务器 / 云主机 / VPS / WSL2 量身打造的 CC Switch 网页管理控制台</strong>
</p>

<p align="center">
  <a href="README.md">English</a> | 简体中文 | <a href="README_JA.md">日本語</a> | <a href="README_DE.md">Deutsch</a>
</p>

<p align="center">
  <a href="#-为什么需要-cc-switch-web">核心优势</a> •
  <a href="#-架构与工作原理">架构设计</a> •
  <a href="#-功能特性">功能特性</a> •
  <a href="#-快速上手">快速上手</a> •
  <a href="#%EF%B8%8F-命令行参数">命令行参数</a> •
  <a href="#%EF%B8%8F-生产环境部署">生产部署</a> •
  <a href="#-常见问题-faq">常见问题</a> •
  <a href="#-致谢与声明">致谢与声明</a>
</p>

---

## 💡 为什么需要 CC Switch Web？

[CC Switch](https://github.com/farion1231/cc-switch) 是一款功能强大的多 AI 编程助手配置与供应商管理工具（支持 Claude Code、Codex、Gemini CLI、Grok Build 等）。

然而，官方桌面版基于 Tauri 2 构建，并依赖系统图形库（如 Linux 下的 WebKitGTK）：
* **无头（Headless）服务器无法运行**：在云端 Linux 服务器、VPS、Docker 或 WSL2 等无桌面环境中，由于缺少图形渲染环境，官方桌面版无法启动；
* **终端 CLI 交互能力受限**：社区的 TUI 终端版虽然轻便，但在多供应商模型对比、实时请求日志追踪、Token 消耗可视化大屏、Skills 技能包市场等场景下，难以提供如 Web 控制台般直观高效的体验。

**CC Switch Web** 解决的正是这一痛点——**剥离对桌面窗口及 WebKitGTK 的强依赖，完整保留官方 Rust 业务核心与 React 控制台前端**。只需在服务器后台运行一个轻量级 Web 服务进程，即可在本地浏览器中享受与桌面端完全一致的可视化管理体验！

---

## 🏗️ 架构与工作原理

CC Switch Web 采用**“控制面与数据面完全隔离”**的轻量安全设计：

```text
┌─────────────────────────────────────────────────────────────┐
│                   远程服务器 (Headless Linux / VPS)          │
│                                                             │
│   [ Claude Code / Codex / Gemini CLI / Grok Build ... ]     │
│             │                                               │
│             │ (模型请求)                                     │
│             ▼                                               │
│   [ 本地模型路由 Local Router ] 127.0.0.1:15721              │
│             │                                               │
│             │ (自动故障转移 / 请求整流 / 协议转换)           │
│             ▼                                               │
│     上游模型供应商 (Claude / OpenAI / Gemini / DeepSeek ...) │
│                                                             │
│ ─────────────────────────────────────────────────────────── │
│                                                             │
│   [ CC Switch Web 后台服务进程 ]                            │
│      ├── 核心数据库: ~/.cc-switch/cc-switch.db (SQLite)     │
│      ├── 各 CLI 工具配置: ~/.claude.json, ~/.codex 等       │
│      ├── 安全访问令牌: ~/.cc-switch/web-token (0600 权限)   │
│      └── Web 控制台端口: 127.0.0.1:3927 (仅本地回环监听)    │
└──────────────────────────────▲──────────────────────────────┘
                               │
                               │ SSH 本地端口转发 (推荐安全通道)
                               │ ssh -N -L 3927:127.0.0.1:3927 user@server
                               │
┌──────────────────────────────┴──────────────────────────────┐
│                    本地电脑 (Your Local PC)                 │
│                                                             │
│   [ 现代浏览器 ] ───▶ http://127.0.0.1:3927/#token=...       │
│   (纯可视化控制面：管理供应商、MCP、Skills、查看用量图表)    │
└─────────────────────────────────────────────────────────────┘
```

### 🔒 核心安全设计
1. **数据与凭据留在服务端**：API Key、OAuth 登录凭据、SQLite 数据库以及高并发的模型数据流，全程仅保留在服务器本地，浏览器仅负责收发轻量管理指令。
2. **默认本地回环隔离**：Web 管理端口（默认 `3927`）与本地路由端口（`15721`）默认均仅监听 `127.0.0.1`，避免在公网裸奔造成凭据泄露。
3. **单机持久化 Token 鉴权**：首次启动自动生成随机安全令牌并写入 `~/.cc-switch/web-token`（权限 `0600`）。浏览器首次使用携带 `#token=...` 的 URL 鉴权后自动持久化，后续可免密访问。

---

## ✨ 功能特性

- 🖥️ **完整移植官方桌面端 UI**：无缝使用官方同款 React 前端，供应商管理、MCP 服务器、Skills 市场、Prompts 提示词库、会话历史、用量大屏一应俱全。
- ⚡ **多工具全面支持**：无缝管理与切换 Claude Code、Codex、Gemini CLI、Grok Build、OpenCode、OpenClaw、Hermes Agent、Pi 等多种 CLI 工具配置。
- 🔄 **强大路由与容灾能力**：支持供应商单键热切换（Hot-switching）、自动故障转移（Failover Queue）、模型请求智能整流（Rectifier）。
- 🐧 **专为无头（Headless）环境适配优化**：
  - 彻底去除 WebKitGTK / GTK3 等桌面库依赖，内存占用极低；
  - 原生系统弹窗（文件/目录选择）自动适配为服务器路径输入；
  - 剪贴板无缝桥接浏览器 Web Clipboard API；
  - 外链与文档直接转由本地浏览器打开；
  - 内置 Vendored OpenSSL 支持，避免轻量服务器因缺少 `pkg-config` / `libssl-dev` 导致编译失败。
- 📦 **双模合一**：依然保留 `desktop` feature 代码分支，有图形环境时仍可自由编译为原生桌面版本。

---

## 🚀 快速上手

### 1. 环境准备
确保服务器已安装基础开发环境：
* **Node.js** (>= 18) 与 **pnpm**
* **Rust** (>= 1.85) 与 **Cargo**

### 2. 编译与构建

在仓库根目录下执行：

```bash
# 1. 安装前端依赖
pnpm install

# 2. 一键构建发布版本（自动完成前端构建 + Rust Web 二进制编译）
pnpm build:web
```

> **分步调试构建（可选）**：
> ```bash
> # 编译前端静态资源到 ./dist
> pnpm build:renderer
> 
> # 编译 Rust Web 调试二进制
> cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --features web --bin cc-switch-web
> ```

### 3. 运行服务

```bash
./src-tauri/target/release/cc-switch-web --port 3927 --dist ./dist
```

服务启动后，终端将输出类似如下信息：
```text
CC Switch 网页已启动
  本机地址: http://127.0.0.1:3927/
  第一次打开: http://127.0.0.1:3927/#token=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
  令牌文件: /home/username/.cc-switch/web-token
  页面目录: /path/to/dist
  停止: Ctrl+C
```

### 4. 访问控制台

#### 方式 A：远程服务器访问（最推荐，安全免公网暴露）
在你的**本地电脑终端**建立 SSH 端口转发隧道：
```bash
ssh -N -L 3927:127.0.0.1:3927 <用户名>@<服务器IP>
```
然后在本地电脑浏览器打开控制台打印的第一次访问地址：
```text
http://127.0.0.1:3927/#token=<你的Token>
```
浏览器记住 Token 后，后续直接访问 `http://127.0.0.1:3927/` 即可。

#### 方式 B：本地开发机 / WSL2 直接访问
在浏览器中直接打开终端打印的 `http://127.0.0.1:3927/#token=<你的Token>`。

---

## 🛠️ 命令行参数

`cc-switch-web` 支持以下命令行选项：

| 参数 | 默认值 | 环境变量 | 说明 |
| :--- | :--- | :--- | :--- |
| `--host <IP>` | `127.0.0.1` | - | 监听绑定的 IP 地址。出于安全考虑，强烈建议仅绑定本地回环。 |
| `--port <PORT>` | `3927` | - | Web 控制台监听端口。 |
| `--dist <PATH>` | `./dist` | `CC_SWITCH_WEB_DIST` | 前端静态资源目录路径（需包含 `index.html`）。 |
| `-h, --help` | - | - | 打印帮助信息并退出。 |

---

## ⚙️ 生产环境部署

为了让服务在服务器后台稳定常驻并在开机时自启，推荐使用 `systemd` 进行托管。

### 1. 安装二进制与前端静态资源
建议将编译好的产物统一放置在规范目录（例如 `/opt/cc-switch-web`）：
```bash
sudo mkdir -p /opt/cc-switch-web
sudo cp ./src-tauri/target/release/cc-switch-web /opt/cc-switch-web/
sudo cp -r ./dist /opt/cc-switch-web/
```

### 2. 创建 Systemd 服务配置
创建 `/etc/systemd/system/cc-switch-web.service` 文件（请将 `your_user` 替换为你的服务器用户名）：

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
# 确保环境变量包含用户目录
Environment=HOME=/home/your_user

[Install]
WantedBy=multi-user.target
```

### 3. 启用与启动服务
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now cc-switch-web

# 查看运行状态
sudo systemctl status cc-switch-web
```

---

## ❓ 常见问题 (FAQ)

<details>
<summary><strong>Q: 访问 Token 保存在哪里？如何重置？</strong></summary>

Token 默认持久化保存在 `~/.cc-switch/web-token` 文件中。
如果需要重置 Token：
1. 停止 `cc-switch-web` 进程；
2. 删除或清空该文件：`rm ~/.cc-switch/web-token`；
3. 重启服务，程序会自动生成一个全新的安全 Token 并打印到终端。
</details>

<details>
<summary><strong>Q: 可以绑定 <code>--host 0.0.0.0</code> 开放给公网直接访问吗？</strong></summary>

**极不推荐直接公网裸奔！** 
虽然 Web 控制台有 Token 认证，但如果直接暴露在公网，明文 HTTP 会导致 Token 在网络传输中泄露，进而暴露你的 API Key 和模型配置。
如果需要远程直连访问，推荐以下方案：
1. **SSH 隧道转发**（最简单、最安全）：`ssh -N -L 3927:127.0.0.1:3927 user@server`；
2. **虚拟内网 / 异地组网**：使用 Tailscale / WireGuard / 蒲公英等内网穿透工具访问；
3. **Nginx 反向代理**：配置 Nginx 并强制开启 HTTPS（SSL/TLS 证书），同时在 Nginx 层面附加 HTTP Basic Auth 或 IP 白名单限制。
</details>

<details>
<summary><strong>Q: 配置文件与数据库存放在哪里？</strong></summary>

与官方桌面版完全一致，所有数据均保留在当前用户的主目录：
* **核心数据库**：`~/.cc-switch/cc-switch.db`（SQLite，管理供应商、MCP、用量等）
* **设备级配置**：`~/.cc-switch/settings.json`
* **Web 访问令牌**：`~/.cc-switch/web-token`
* **各工具本地配置**：`~/.claude/`、`~/.codex/`、`~/.gemini/` 等原有目录
</details>

<details>
<summary><strong>Q: 还能在这个仓库编译原生桌面版吗？</strong></summary>

可以。代码中保留了原版的所有桌面特性。在具备图形环境（已安装 WebKitGTK、GTK3）的机器上，默认编译命令仍会生成桌面应用：
```bash
pnpm install
pnpm dev      # 桌面版开发模式
pnpm build    # 构建桌面版安装包
```
</details>

---

## 📄 致谢与声明

* 本项目是 [CC Switch](https://github.com/farion1231/cc-switch) 的衍生版本，专为服务器与无头环境定制开发，**不是官方项目**。
* 原项目版权归 **Jason Young (2025)** 所有，遵循 **[MIT License](LICENSE)** 开源协议。
* 官方桌面版的发布地址、完整说明文档与赞助信息，请访问上游官方仓库：[farion1231/cc-switch](https://github.com/farion1231/cc-switch) 及官方网站 [ccswitch.io](https://ccswitch.io)。
