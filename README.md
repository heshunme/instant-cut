# 瞬剪 Instant Cut

<div align="center">

![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=for-the-badge)

**一个极简的本地视频剪辑工具，基于 Tauri + Vue 3 构建**

[功能特性](#功能特性) • [安装要求](#安装要求) • [快速开始](#快速开始) • [使用说明](#使用说明) • [本地编译](#本地编译) • [开发指南](#开发指南)

</div>

## 📋 功能特性

- 🚀 **极速剪辑**：使用 FFmpeg 进行无重编码剪辑，速度极快
- 📁 **智能版本管理**：自动版本化文件命名，避免覆盖原始文件
- 🎨 **极简设计**：黑白极简 UI 风格，专注于功能本身
- 💬 **备注支持**：为剪辑片段添加备注，便于管理
- 🔄 **实时预览**：内置视频播放器，支持跳转到指定时间点
- ✅ **安全检查**：自动检查磁盘空间，防止剪辑失败

## ⚠️ 安装要求

### **重要前提条件**

> **🛠️ 本应用要求您的系统已安装 [FFmpeg](https://ffmpeg.org/) 和 FFprobe，并且可以在命令行中直接执行。**

#### 如何检查是否已安装

打开终端（Windows 用户打开 Command Prompt 或 PowerShell），输入以下命令：

```bash
ffmpeg -version
ffprobe -version
```

如果看到版本信息输出，说明已正确安装。如果出现"命令不存在"或类似错误，请先安装 FFmpeg。

#### 安装 FFmpeg

**Windows:**
```bash
# 使用包管理器 (推荐)
winget install Gyan.FFmpeg
# 或使用 Chocolatey
choco install ffmpeg
# 或使用 Scoop
scoop install ffmpeg
```

**macOS:**
```bash
# 使用 Homebrew
brew install ffmpeg
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install ffmpeg
```

**Linux (CentOS/RHEL):**
```bash
sudo yum install epel-release
sudo yum install ffmpeg
```

### 其他系统要求

**对于最终用户（使用预编译版本）：**
- 除了 FFmpeg/FFprobe 外，**无需其他依赖**

**对于开发者（本地编译）：**
- **Node.js** 18.0 或更高版本
- **Rust** 1.70 或更高版本
- **Ubuntu/Linux 额外要求**：
  ```bash
  sudo apt-get update
  sudo apt-get install -y pkg-config build-essential libclang-dev libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev libglib2.0-dev libgtk-3-dev librsvg2-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
  ```

## 🚀 快速开始

### 方式一：下载预编译的可执行文件（推荐）

1. 前往 [Releases](https://github.com/heshunme/instant-cut/releases) 页面
2. 下载适合您操作系统的可执行文件：
   - Windows: `instant-cut.exe`
   - macOS: `instant-cut`
   - Linux: `instant-cut`
3. 直接运行可执行文件（无需安装）
4. 确保 FFmpeg 已安装在系统中（见上述安装要求）

### 方式二：本地编译

详细编译步骤请参考 [本地编译指南](#本地编译) 部分。

## 📖 使用说明

### 基本操作流程

1. **选择视频文件**
   - 点击"拖放视频文件到此处"区域
   - 或直接将视频文件拖拽到应用窗口
   - 支持常见视频格式（MP4、AVI、MOV、MKV 等）

2. **预览和设置时间点**
   - 使用播放器预览视频内容
   - 在时间线上拖拽选择剪辑片段
   - 或在输入框中手动输入开始/结束时间
   - 支持时间格式：`HH:MM:SS`、`MM:SS`、`SS`

3. **添加备注（可选）**
   - 在备注框中输入描述信息
   - 备注将包含在输出文件名中

4. **开始剪辑**
   - 点击"开始剪辑"按钮
   - 等待处理完成
   - 新文件将自动保存在原文件同目录

### 版本管理说明

应用会自动管理文件版本：

- 原始文件：`video.mp4`
- 第一次剪辑：`video_1.mp4`
- 基于剪辑文件再剪辑：`video_1_1.mp4`
- 带备注的剪辑：`video_1_highlight.mp4`

## 🔧 本地编译

> **📌 重要：本地编译前请确保已安装 Node.js 18+ 和 Rust 1.70+**

### 编译步骤

1. **克隆仓库**
   ```bash
   git clone https://github.com/heshunme/instant-cut.git
   cd instant-cut
   ```

2. **安装前端依赖**
   ```bash
   npm install
   ```

3. **开发模式运行（可选）**
   ```bash
   npm run tauri dev
   ```
   这将启动开发模式，可以实时预览修改效果。

4. **构建生产版本**
   ```bash
   npm run tauri build
   ```

   构建完成后，可执行文件将位于：
   - Windows: `src-tauri/target/release/instant-cut.exe`
   - macOS: `src-tauri/target/release/instant-cut`
   - Linux: `src-tauri/target/release/instant-cut`

   > **注意**：本项目已禁用打包功能（bundle），因此只会生成单个可执行文件，不会创建安装包。


## 🛠️ 开发指南

### 项目结构

```
instant-cut/
├── src/                   # Vue 前端源码
│   ├── components/        # Vue 组件
│   ├── composables/       # Vue 组合函数
│   ├── utils/            # 工具函数
│   └── types.ts          # TypeScript 类型定义
├── src-tauri/            # Rust 后端源码
│   ├── src/
│   │   ├── main.rs       # Tauri 主程序
│   │   ├── media.rs      # 媒体处理逻辑
│   │   ├── utils.rs      # 工具函数
│   │   ├── video.rs      # 视频数据结构
│   │   └── error.rs      # 错误处理
│   └── Cargo.toml        # Rust 依赖配置
├── package.json          # Node.js 依赖配置
├── tauri.conf.json       # Tauri 应用配置
└── vite.config.ts        # Vite 构建配置
```

### 开发命令

```bash
# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 仅前端开发 (Vite)
npm run dev

# 构建前端
npm run build

# 运行 Rust 测试
cd src-tauri && cargo test

# 构建 Tauri 应用
npm run tauri build
```

### 贡献指南

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 🤝 技术栈

- **前端框架**: Vue 3 + TypeScript
- **构建工具**: Vite
- **桌面框架**: Tauri 2.0
- **后端语言**: Rust
- **视频处理**: FFmpeg + FFprobe
- **UI 风格**: 原生 CSS (黑白极简设计)

## ⚡ 性能特点

- **无重编码剪辑**: 使用流复制技术，速度比重编码剪辑快 10-100 倍
- **低内存占用**: 不需要加载整个视频到内存
- **智能缓存**: 自动检测磁盘空间，避免剪辑失败
- **跨平台兼容**: Windows、macOS、Linux 全平台支持

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [FFmpeg](https://ffmpeg.org/) - 强大的多媒体处理框架
- [Tauri](https://tauri.app/) - 现代化的桌面应用开发框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架

## ❓ 常见问题

### Ubuntu/Linux 构建问题

**问：在 Ubuntu 上编译时遇到系统库缺失错误？**

答：Tauri 在 Linux 上需要一些系统库。请先安装以下依赖：

```bash
sudo apt-get update
sudo apt-get install -y pkg-config build-essential libclang-dev libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev libglib2.0-dev libgtk-3-dev librsvg2-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
```

### FFmpeg 相关问题

**问：应用提示"FFmpeg 未安装"？**

答：请确保 FFmpeg 和 FFprobe 已正确安装并在 PATH 中可用：

```bash
# 验证安装
ffmpeg -version
ffprobe -version

# Ubuntu/Debian 安装
sudo apt-get install ffmpeg

# macOS 安装
brew install ffmpeg

# Windows 安装
winget install Gyan.FFmpeg
```

### 构建产物问题

**问：构建后没有找到可执行文件？**

答：可执行文件位于：
- Windows: `src-tauri/target/release/instant-cut.exe`
- macOS/Linux: `src-tauri/target/release/instant-cut`

确保构建没有错误输出，文件应该会自动生成。

## 📞 支持

如果您遇到问题或有建议，请：

1. 查看 [常见问题](#常见问题) 部分
2. 搜索 [Issues](https://github.com/heshunme/instant-cut/issues) 页面
3. 创建新的 Issue 描述您的问题
4. 参与 [Discussions](https://github.com/heshunme/instant-cut/discussions) 讨论

---

<div align="center">

**[⬆ 回到顶部](#instant-cut)**

Made with ❤️ by [heshunme](https://github.com/heshunme)

</div>