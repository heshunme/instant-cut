<template>
  <div
    class="app"
    @drop="handleGlobalDrop"
    @dragover.prevent
    @dragenter="handleDragEnter"
    @dragleave="handleDragLeave"
    :class="{ 'drag-over': isDraggingOver }"
  >
    <main class="app-main">
      <!-- FFmpeg 检查提示 -->
      <div v-if="ffmpegError" class="error-banner">
        {{ ffmpegError }}
      </div>

      <!-- 文件选择 -->
      <FileSelector
        :selectedPath="state.selectedFile"
        @fileSelected="onFileSelected"
        @fileCleared="onFileCleared"
      />

      <!-- 视频内容区域 (横向布局) -->
      <div v-if="state.selectedFile" class="main-content-area">
        <!-- 视频预览 -->
        <VideoPlayer
          :videoPath="state.selectedFile"
          :seekTime="seekTime"
          @timeUpdate="onTimeUpdate"
          @durationChange="onDurationChange"
        />

        <!-- 视频信息 -->
        <VideoInfo
          v-if="state.videoInfo"
          :info="state.videoInfo"
        />
      </div>

      <!-- 时间线和控制整合 -->
      <TimelineControls
        v-if="state.videoInfo"
        :duration="state.videoInfo.duration"
        :currentTime="state.currentTime"
        :startTime="state.startTime"
        :endTime="state.endTime"
        :isProcessing="state.isProcessing"
        :canCut="canCut"
        @updateStart="updateStartTime"
        @updateEnd="updateEndTime"
        @cut="handleCut"
        @previewJump="handlePreviewJump"
      />
    </main>

    <!-- Toast容器 -->
    <Toast ref="toastRef" />
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import FileSelector from './components/FileSelector.vue'
import VideoInfo from './components/VideoInfo.vue'
import VideoPlayer from './components/VideoPlayer.vue'
import TimelineControls from './components/TimelineControls.vue'
import Toast from './components/Toast.vue'
import { useToast } from './composables/useToast'
import type { VideoInfo as VideoInfoType, AppState } from './types'

const state = reactive<AppState>({
  selectedFile: null,
  videoInfo: null,
  startTime: 0,
  endTime: 0,
  currentTime: 0,
  isProcessing: false
})

const seekTime = ref<number>()
const ffmpegError = ref<string>('')
const isDraggingOver = ref(false)
const toastRef = ref()
const { showToast, registerToast } = useToast()

const canCut = computed(() => {
  return state.videoInfo !== null &&
    state.startTime < state.endTime &&
    !state.isProcessing
})

onMounted(async () => {
  // 注册Toast实例
  if (toastRef.value) {
    registerToast(toastRef.value)
  }

  try {
    await invoke('check_ffmpeg')
  } catch (error) {
    ffmpegError.value = String(error)
  }
})

async function onFileSelected(path: string) {
  state.selectedFile = path
  state.videoInfo = null
  state.startTime = 0
  state.endTime = 0
  state.currentTime = 0

  try {
    const info = await invoke<VideoInfoType>('get_video_info', { path })
    state.videoInfo = info
    state.startTime = 0
    state.endTime = info.duration
  } catch (error) {
    alert(`获取视频信息失败: ${error}`)
  }
}

function onFileCleared() {
  state.selectedFile = null
  state.videoInfo = null
  state.startTime = 0
  state.endTime = 0
  state.currentTime = 0
}

// Tauri 文件扩展接口
interface TauriFile extends File {
  path?: string
}

// 全窗口拖放处理
function handleGlobalDrop(event: DragEvent) {
  event.preventDefault()
  isDraggingOver.value = false

  if (state.selectedFile) return // 已有文件，不接受新拖放

  const files = event.dataTransfer?.files
  if (files && files.length > 0) {
    const file = files[0] as TauriFile
    // 检查文件类型
    if (file.type.startsWith('video/')) {
      // 获取文件路径 (Tauri 需要特殊处理)
      const path = file.path || file.name
      onFileSelected(path)
    } else {
      alert('请选择视频文件')
    }
  }
}

function handleDragEnter(event: DragEvent) {
  event.preventDefault()
  if (!state.selectedFile) {
    isDraggingOver.value = true
  }
}

function handleDragLeave(event: DragEvent) {
  event.preventDefault()
  // 简化处理：直接设置为false
  isDraggingOver.value = false
}

function onTimeUpdate(time: number) {
  state.currentTime = time
}

function onDurationChange(duration: number) {
  if (state.videoInfo) {
    // 创建新的 VideoInfo 对象来确保响应式更新
    state.videoInfo = {
      ...state.videoInfo,
      duration: duration
    }
    state.endTime = duration
  }
}

function updateStartTime(time: number) {
  state.startTime = Math.max(0, Math.min(time, state.endTime - 1))
  seekTime.value = state.startTime
}

function updateEndTime(time: number) {
  if (state.videoInfo) {
    state.endTime = Math.min(state.videoInfo.duration, Math.max(time, state.startTime + 1))
    seekTime.value = state.endTime
  }
}

function handlePreviewJump(time: number) {
  seekTime.value = time
}

async function handleCut(notes?: string) {
  if (!state.selectedFile || !canCut.value) return

  state.isProcessing = true

  try {
    const result = await invoke<string>('cut_video', {
      input: state.selectedFile,
      start: state.startTime,
      end: state.endTime,
      notes: notes || null
    })
    showToast(result)
  } catch (error) {
    alert(`剪辑失败: ${error}`)
  } finally {
    state.isProcessing = false
  }
}
</script>

<style scoped>
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.app.drag-over {
  background-color: rgba(255, 255, 255, 0.05);
}

/* 全窗口拖放指示器 */
.app::before {
  content: '';
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.1);
  border: 3px dashed #666;
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.3s ease;
  z-index: 1000;
}

.app.drag-over::before {
  opacity: 1;
}


.app-main {
  flex: 1;
  padding: 0.8rem 2rem;
  max-width: 1200px;
  width: 100%;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
  overflow-y: auto;
  min-height: 0;
}

.main-content-area {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 1rem;
  margin-bottom: 1rem;
}

@media (max-width: 1024px) {
  .main-content-area {
    grid-template-columns: 1fr;
  }
}

.error-banner {
  background-color: rgba(255, 0, 0, 0.1);
  border: 1px solid rgba(255, 0, 0, 0.3);
  color: #ff6666;
  padding: 1rem;
  border-radius: 8px;
  text-align: center;
}
</style>
