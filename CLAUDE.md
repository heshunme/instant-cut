# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Fast Video Cutter 是一个极简的本地视频剪辑工具，基于 Tauri + Vue 3 构建。本质上是为 ffmpeg 提供了一个 GUI 壳，用于高度定制化的快速视频剪辑流程。使用黑白极简 UI 风格。

**核心特性**：使用 ffmpeg 的 `-c copy` 参数进行无重编码剪辑，速度快但精度受限于关键帧。
**版本控制系统**：支持版本化文件命名（如 `video_1.mp4`, `video_1_2.mp4`），自动递增版本号。

## 开发命令

### 前端开发
```bash
npm install              # 安装依赖
npm run dev              # 启动 Vite 开发服务器（仅前端）
npm run build            # 构建前端生产版本
npm run preview          # 预览构建结果
```

### Tauri 应用开发
```bash
npm run tauri dev        # 开发模式运行完整应用（Rust + Vue）
npm run tauri build      # 构建生产版本
```

### Rust 后端
```bash
cd src-tauri
cargo build              # 构建 debug 版本
cargo build --release    # 构建 release 版本
cargo test               # 运行测试
```

## 项目架构

### 前后端通信架构

这是一个典型的 Tauri 应用，采用**前后端分离架构**：

- **前端（Vue 3）**：运行在 WebView 中，处理 UI 交互和状态管理
- **后端（Rust）**：通过 Tauri Commands 暴露给前端调用，处理视频相关逻辑

**通信方式**：前端通过 `@tauri-apps/api` 的 `invoke()` 函数调用后端的 Tauri Commands。

### 核心模块

#### Rust 后端 (`src-tauri/src/`)

1. **main.rs**：Tauri 应用入口，注册三个核心命令：
   - `check_ffmpeg()`: 检测 ffmpeg/ffprobe 是否安装
   - `get_video_info(path)`: 使用 ffprobe 获取视频元数据
   - `cut_video(input, start, end, notes)`: 执行视频剪辑（支持备注功能）

2. **media.rs**：所有媒体处理逻辑的核心模块
   - `get_video_info()`: 解析 ffprobe JSON 输出，提取视频流信息
   - `cut_video()`: 完整的剪辑流程，包括版本管理、磁盘空间检查、文件命名
   - `generate_next_filename()`: 智能文件命名系统，支持版本递增
   - `check_disk_space_for_output()`: 使用 sysinfo 进行跨平台磁盘空间检查

3. **video.rs**：定义 `VideoInfo` 结构体，存储视频元数据

4. **utils.rs**：工具函数集合
   - `execute_ffmpeg()` / `execute_ffprobe()`: 跨平台命令执行（Windows 隐藏控制台）
   - `validate_input_path()`: 路径验证
   - `validate_time_range()`: 时间参数验证
   - `parse_frame_rate()`: 帧率字符串解析（如 "30000/1001"）
   - `sanitize_filename()`: 文件名清理，过滤非法字符

5. **error.rs**：统一错误处理系统
   - `AppError` 枚举：包含 FFmpeg、文件系统、验证等各种错误类型
   - 自动转换为 Tauri 兼容的 String 错误格式

#### Vue 前端 (`src/`)

1. **App.vue**：主应用组件，管理全局状态 `AppState`：
   - `selectedFile`: 当前选择的视频文件路径
   - `videoInfo`: 视频元数据
   - `startTime/endTime`: 剪辑时间范围
   - `currentTime`: 播放器当前时间
   - `isProcessing`: 是否正在处理

2. **组件结构**（所有组件位于 `src/components/`）：
   - `FileSelector.vue`: 文件选择（支持拖拽和 Tauri 对话框）
   - `VideoInfo.vue`: 显示视频信息（时长、分辨率、帧率、编码）
   - `VideoPlayer.vue`: HTML5 视频预览播放器，支持精确跳转
   - `Timeline.vue`: 时间线，用于可视化选择剪辑片段
   - `TimelineControls.vue`: 整合了时间线、时间输入和剪辑控制的复合组件

3. **工具和状态管理**：
   - `types.ts`: TypeScript 类型定义（VideoInfo, AppState）
   - `composables/useToast.ts`: Toast 通知系统
   - `utils/timeUtils.ts`: 时间处理工具函数（格式化、解析、验证）

### 数据流

1. 用户选择文件 → `FileSelector` 触发 `fileSelected` 事件
2. `App.vue` 调用 `invoke('get_video_info')` 获取元数据
3. 更新 `state.videoInfo`，显示视频信息和预览
4. 用户在 `Timeline` 或 `CutControls` 调整时间范围
5. 点击"开始剪辑" → 调用 `invoke('cut_video')`
6. Rust 后端执行 ffmpeg 命令，完成后重命名文件

## 关键技术细节

### ffmpeg 剪辑策略

使用 `-c copy` 参数进行流复制（stream copy），避免重新编码：
- **优点**：速度极快（仅文件复制操作）
- **缺点**：精度受限于关键帧位置，实际剪辑点可能略有偏差

### 文件处理流程

1. **版本化命名系统**：
   - 基础文件：`video.mp4` → `video_1.mp4`, `video_2.mp4`...
   - 嵌套版本：`video_1.mp4` → `video_1_1.mp4`, `video_1_2.mp4`...
   - 支持备注：`video_1_highlight.mp4`

2. **剪辑流程**：
   - 验证输入文件和时间范围
   - 检查磁盘空间（自动估算输出大小并添加20%缓冲）
   - 执行 ffmpeg 剪辑：`-ss START -i INPUT -t DURATION -c copy -avoid_negative_ts 1`
   - 输出到新生成的版本文件，不影响原文件

3. **错误处理**：
   - 统一的错误类型系统
   - 详细的用户友好的错误信息
   - 自动回滚机制（如果操作失败）

### 时间格式支持

支持多种输入格式（在 `timeUtils.ts` 中统一处理）：
- `HH:MM:SS` → 1:30:45（1小时30分45秒）
- `MM:SS` → 30:45（30分45秒）
- `SS` → 45（45秒）
- 自动验证和错误提示
- 支持时间范围验证和持续时长计算

## 系统要求

- **必需**：系统已安装 ffmpeg 和 ffprobe，且在 PATH 中可用
- Node.js 18+
- Rust 1.70+

## 常见开发场景

### 添加新的视频处理功能

1. 在 `src-tauri/src/media.rs` 中添加处理函数
2. 在 `src-tauri/src/main.rs` 中注册为 Tauri Command
3. 在前端通过 `invoke('command_name')` 调用
4. 如需新数据结构，在 `src/types.ts` 和 `src-tauri/src/video.rs` 中同步定义
5. 使用统一的错误处理模式（返回 `AppResult<T>`）

### 修改 UI 组件

所有组件采用 Vue 3 Composition API（`<script setup>`）编写。样式使用 scoped CSS，遵循黑白极简设计风格。

### 调试 Rust 后端

使用 `cargo build` 编译时，Rust 的 `println!` 输出会在 `npm run tauri dev` 的控制台中显示。

## 测试

### Rust 测试
```bash
cd src-tauri
cargo test                # 运行所有测试
cargo test test_name      # 运行特定测试
```

主要测试模块：
- `media.rs` 中的文件名解析、磁盘空间检查、时间验证等
- `utils.rs` 中的帧率解析、文件名清理、时间格式化等
- `error.rs` 中的错误类型和转换

## 注意事项

- Windows 路径使用反斜杠，Rust 的 `std::path::Path` 已自动处理
- 大文件处理时间主要取决于硬盘速度（文件复制操作）
- 视频预览使用本地文件路径，通过 Tauri 的文件系统访问
- 版本化命名系统会自动处理文件名冲突，无需手动管理
- 磁盘空间检查包含安全缓冲区（20%），防止空间不足导致的失败
- ffmpeg 的 `-avoid_negative_ts 1` 参数确保输出文件的时间戳兼容性
