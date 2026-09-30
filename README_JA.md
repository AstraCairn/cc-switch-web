# CC Switch Web

<p align="center">
  <strong>GUIのない Linux サーバー / クラウド VPS / WSL2 向けに最適化された CC Switch Web 管理コンソール</strong>
</p>

<p align="center">
  <a href="README.md">English</a> | <a href="README_ZH.md">简体中文</a> | 日本語 | <a href="README_DE.md">Deutsch</a>
</p>

<p align="center">
  <a href="#-なぜ-cc-switch-web-なのか">主なメリット</a> •
  <a href="#-アーキテクチャと仕組み">アーキテクチャ</a> •
  <a href="#-主な機能">主な機能</a> •
  <a href="#-クイックスタート">クイックスタート</a> •
  <a href="#%EF%B8%8F-コマンドラインオプション">コマンドラインオプション</a> •
  <a href="#%EF%B8%8F-本番環境デプロイ">本番デプロイ</a> •
  <a href="#-よくある質問-faq">FAQ</a> •
  <a href="#-謝辞とライセンス">謝辞とライセンス</a>
</p>

---

## 💡 なぜ CC Switch Web なのか？

[CC Switch](https://github.com/farion1231/cc-switch) は、多様な AI コーディングアシスタント（Claude Code、Codex、Gemini CLI、Grok Build など）の設定やプロバイダーを管理するための強力なツールです。

しかし、公式のデスクトップ版は Tauri 2 ベースで構築されており、システムの GUI ライブラリ（Linux 上の WebKitGTK など）に依存しています：
* **ヘッドレスサーバーで動作しない**: GUI のないクラウド Linux サーバー、VPS、Docker コンテナ、WSL2 環境では、グラフィカルライブラリの欠如によりデスクトップ版を起動できません。
* **ターミナル CLI の操作性の限界**: コミュニティ製の TUI ツールは軽量ですが、複数プロバイダーの比較、リアルタイムのリクエストログ監視、トークン消費の視覚的なダッシュボード、Skills マーケットの管理などの場面では、Web コンソールのような直感的な操作が困難です。

**CC Switch Web** はまさにこの課題を解決します——**デスクトップウィンドウや WebKitGTK への依存を排除し、公式の Rust コアロジックと React コンソール UI を完全な形で保持**しています。サーバー上で軽量な Web デーモンを実行するだけで、ローカル PC のブラウザからデスクトップ版とまったく同じ視覚的管理体験を利用できます！

---

## 🏗️ アーキテクチャと仕組み

CC Switch Web は、**「コントロールプレーン（管理面）とデータプレーン（データ面）の完全分離」**という安全重視の設計を採用しています：

```text
┌─────────────────────────────────────────────────────────────┐
│                 リモートサーバー (Headless Linux / VPS)       │
│                                                             │
│   [ Claude Code / Codex / Gemini CLI / Grok Build ... ]     │
│             │                                               │
│             │ (モデルリクエスト)                             │
│             ▼                                               │
│   [ ローカルモデルルーター ] 127.0.0.1:15721                 │
│             │                                               │
│             │ (自動フェイルオーバー / プロトコル変換)        │
│             ▼                                               │
│     上流プロバイダー (Claude / OpenAI / Gemini ...)          │
│                                                             │
│ ─────────────────────────────────────────────────────────── │
│                                                             │
│   [ CC Switch Web バックグラウンドデーモン ]                │
│      ├── SQLite DB: ~/.cc-switch/cc-switch.db               │
│      ├── CLI ツール設定: ~/.claude.json, ~/.codex 等        │
│      ├── 永続化認証トークン: ~/.cc-switch/web-token (0600)  │
│      └── Web 管理ポート: 127.0.0.1:3927 (ローカルホスト限定) │
└──────────────────────────────▲──────────────────────────────┘
                               │
                               │ SSH ローカルポート転送 (推奨安全経路)
                               │ ssh -N -L 3927:127.0.0.1:3927 user@server
                               │
┌──────────────────────────────┴──────────────────────────────┐
│                    あなたのローカル PC                      │
│                                                             │
│   [ 最新ブラウザ ] ───▶ http://127.0.0.1:3927/#token=...    │
│   (純粋な管理画面：プロバイダー、MCP、Skills、使用量グラフ) │
└─────────────────────────────────────────────────────────────┘
```

### 🔒 コアセキュリティ設計
1. **クレデンシャルとトラフィックはサーバー内に留まる**: API キー、OAuth ログイン情報、SQLite データベース、モデルトラフィックはすべてサーバーローカル内に保持され、ブラウザは軽量な管理コマンドのみを送受信します。
2. **デフォルトでローカルループバックに限定**: Web 管理ポート（デフォルト `3927`）およびプロキシポート（`15721`）は、意図しない外部公開を防ぐためデフォルトで `127.0.0.1` のみをリッスンします。
3. **永続化トークンによる認証**: 初回起動時にランダムなセキュリティトークンが自動生成され、`~/.cc-switch/web-token`（権限 `0600`）に保存されます。ブラウザで初回アクセス時に `#token=...` を付与して認証すれば、以降はパスワードレスでアクセス可能です。

---

## ✨ 主な機能

- 🖥️ **デスクトップ版 UI を完全移植**: デスクトップ版と同一の React フロントエンドを採用。プロバイダー、MCP サーバー、Skills、プロンプト集、セッション履歴、トークン使用状況ダッシュボードをすべて管理可能。
- ⚡ **多様な CLI ツールに対応**: Claude Code、Codex、Gemini CLI、Grok Build、OpenCode、OpenClaw、Hermes Agent、Pi などの設定をシームレスに切り替え。
- 🔄 **堅牢なルーティングとフォールバック**: ワンクリックでのプロバイダー即時切り替え、自動フェイルオーバーキュー、リクエストの自動修正（Rectifier）をサポート。
- 🐧 **ヘッドレス環境に最適化**:
  - WebKitGTK などの GUI 依存を完全排除し、極めて軽量なリソース消費を実現；
  - ファイル・ディレクトリ選択ダイアログをサーバーパスの入力形式に自動適応；
  - クリップボード操作をブラウザの Web Clipboard API と連携；
  - 外部リンクをローカルブラウザで直接オープン；
  - Vendored OpenSSL を内蔵し、最小構成サーバーでのビルドエラーを防止。
- 📦 **デュアルビルド対応**: `desktop` フィーチャーを維持しているため、GUI 環境のある PC では引き続きネイティブデスクトップ版としてビルド可能です。

---

## 🚀 クイックスタート

### 1. 準備
サーバーに以下の開発環境がインストールされていることを確認してください：
* **Node.js** (>= 18) および **pnpm**
* **Rust** (>= 1.85) および **Cargo**

### 2. ビルド

リポジトリのルートディレクトリで実行します：

```bash
# 1. フロントエンド依存関係のインストール
pnpm install

# 2. リリースビルド（フロントエンドビルド + Rust Web バイナリのコンパイル）
pnpm build:web
```

> **個別デバッグビルド（任意）**:
> ```bash
> # フロントエンドの静的アセットを ./dist にビルド
> pnpm build:renderer
> 
> # デバッグ用 Web バイナリをビルド
> cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --features web --bin cc-switch-web
> ```

### 3. 起動

```bash
./src-tauri/target/release/cc-switch-web --port 3927 --dist ./dist
```

起動後、コンソールに以下のような情報が表示されます：
```text
CC Switch 网页已启动
  本机地址: http://127.0.0.1:3927/
  第一次打开: http://127.0.0.1:3927/#token=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
  令牌文件: /home/username/.cc-switch/web-token
  页面目录: /path/to/dist
  停止: Ctrl+C
```

### 4. コンソールへのアクセス

#### 方法 A：リモートサーバーへの SSH トンネル（推奨・安全）
あなたの**ローカル PC** のターミナルで SSH ポート転送を実行します：
```bash
ssh -N -L 3927:127.0.0.1:3927 <ユーザー名>@<サーバーIP>
```
その後、ローカルブラウザでサーバー出力に表示された初期化 URL を開きます：
```text
http://127.0.0.1:3927/#token=<トークン>
```
ブラウザがトークンを保存した後は、次回以降 `http://127.0.0.1:3927/` に直接アクセスできます。

#### 方法 B：ローカル PC / WSL2 での直接アクセス
ブラウザで直接 `http://127.0.0.1:3927/#token=<トークン>` を開きます。

---

## 🛠️ コマンドラインオプション

`cc-switch-web` は以下のオプションをサポートしています：

| オプション | デフォルト値 | 環境変数 | 説明 |
| :--- | :--- | :--- | :--- |
| `--host <IP>` | `127.0.0.1` | - | バインドする IP アドレス（安全のためローカルホスト推奨）。 |
| `--port <PORT>` | `3927` | - | Web 管理コンソールのポート番号。 |
| `--dist <PATH>` | `./dist` | `CC_SWITCH_WEB_DIST` | フロントエンド静的アセットのディレクトリ（`index.html` を含むこと）。 |
| `-h, --help` | - | - | ヘルプメッセージを表示して終了。 |

---

## ⚙️ 本番環境デプロイ

サーバーでバックグラウンド常駐させ、システム起動時に自動実行するには `systemd` の利用を推奨します。

### 1. バイナリと静的ファイルの配置
ビルド生成物を標準的なディレクトリ（例: `/opt/cc-switch-web`）にコピーします：
```bash
sudo mkdir -p /opt/cc-switch-web
sudo cp ./src-tauri/target/release/cc-switch-web /opt/cc-switch-web/
sudo cp -r ./dist /opt/cc-switch-web/
```

### 2. Systemd サービスファイルの作成
`/etc/systemd/system/cc-switch-web.service` を作成します（`your_user` は実際のユーザー名に変更してください）：

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
Environment=HOME=/home/your_user

[Install]
WantedBy=multi-user.target
```

### 3. サービスの有効化と起動
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now cc-switch-web

# 状態を確認
sudo systemctl status cc-switch-web
```

---

## ❓ よくある質問 (FAQ)

<details>
<summary><strong>Q: 認証トークンはどこに保存されますか？リセット方法は？</strong></summary>

トークンは `~/.cc-switch/web-token` に保存されます。
リセットしたい場合：
1. `cc-switch-web` プロセスを停止します。
2. ファイルを削除します: `rm ~/.cc-switch/web-token`。
3. サービスを再起動すると、新しいトークンが自動生成され標準出力に表示されます。
</details>

<details>
<summary><strong>Q: <code>--host 0.0.0.0</code> を指定してインターネットに公開しても安全ですか？</strong></summary>

**暗号化されていない HTTP ポートをインターネットに直接公開することは強くお勧めしません！**
トークン認証はありますが、平文 HTTP 通信ではトークンや API キーが盗聴される危険があります。
外部からアクセスする場合は以下のいずれかを推奨します：
1. **SSH ポート転送**（推奨）: `ssh -N -L 3927:127.0.0.1:3927 user@server`
2. **VPN / メッシュネットワーク**: Tailscale、WireGuard 等の安全なプライベートネットワーク経由でアクセス
3. **Nginx リバースプロキシ**: SSL/TLS 証明書を設定して HTTPS 化し、Basic 認証または IP 制限を付与
</details>

<details>
<summary><strong>Q: データや設定ファイルはどこに保存されますか？</strong></summary>

公式デスクトップ版と同一です：
* **SQLite データベース**: `~/.cc-switch/cc-switch.db`
* **設定ファイル**: `~/.cc-switch/settings.json`
* **Web トークン**: `~/.cc-switch/web-token`
* **各ツール設定**: `~/.claude/`、`~/.codex/`、`~/.gemini/` など
</details>

<details>
<summary><strong>Q: このリポジトリからデスクトップ版をビルドすることはできますか？</strong></summary>

はい、可能です。WebKitGTK や GTK3 がインストールされているデスクトップ環境では、通常通りビルドできます：
```bash
pnpm install
pnpm dev      # デスクトップ版の開発実行
pnpm build    # デスクトップ版のパッケージビルド
```
</details>

---

## 📄 謝辞とライセンス

* 本プロジェクトは [CC Switch](https://github.com/farion1231/cc-switch) の派生版であり、ヘッドレスサーバー環境向けにカスタマイズされたものです。**公式プロジェクトではありません。**
* 元プロジェクトの著作権は **Jason Young (2025)** に帰属し、**[MIT License](LICENSE)** に基づいて提供されます。
* 公式デスクトップ版のリリース、詳細マニュアル、スポンサー情報については、上流のリポジトリ [farion1231/cc-switch](https://github.com/farion1231/cc-switch) および公式サイト [ccswitch.io](https://ccswitch.io) をご参照ください。
