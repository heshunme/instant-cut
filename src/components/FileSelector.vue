<template>
  <div class="file-selector">
    <div v-if="!selectedPath" class="compact-drop-zone" :class="{ 'dragging': isDragging }" @click="handleClickSelect">
      <div class="drop-zone-content">
        <div class="drop-icon">📹</div>
        <p class="drop-text">拖放视频文件到此处</p>
        <p class="drop-hint">或点击选择文件</p>
      </div>
    </div>

    <!-- 已选择文件后显示紧凑提示 -->
    <div v-else class="current-file">
      <div class="file-info">
        <span class="file-name">{{ fileName }}</span>
        <button @click.stop="clearFile" class="clear-btn">✕</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { getCurrentWindow } from '@tauri-apps/api/window'

const props = defineProps<{
  selectedPath?: string | null
}>()

const emit = defineEmits<{
  fileSelected: [path: string]
  fileCleared: []
}>()

const isDragging = ref(false)
let unlistenDrop: (() => void) | null = null

const selectedPath = computed(() => props.selectedPath)
const fileName = computed(() => {
  if (!selectedPath.value) return ''
  return selectedPath.value.split(/[\\/]/).pop() || ''
})

// 点击选择文件（使用 Tauri Dialog）
async function handleClickSelect() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Video',
        extensions: ['mp4', 'mov', 'avi', 'mkv', 'flv', 'wmv', 'webm', 'm4v']
      }]
    })

    if (selected && typeof selected === 'string') {
      emit('fileSelected', selected)
    }
  } catch (error) {
    console.error('文件选择失败:', error)
  }
}

// 监听拖拽放置事件 - 使用正确的 Tauri 2.0 API
onMounted(async () => {
  try {
    console.log('初始化 Tauri 2.0 拖拽事件监听...')

    // 使用正确的 Tauri 2.0 API
    unlistenDrop = await getCurrentWindow().onDragDropEvent((event) => {
      console.log('拖拽事件触发:', event)

      // 处理不同类型的拖拽事件
      if (event.payload.type === 'enter' || event.payload.type === 'over') {
        console.log('文件悬停在窗口上')
        isDragging.value = true
      } else if (event.payload.type === 'drop') {
        console.log('文件已拖拽:', event.payload.paths)
        isDragging.value = false

        const paths = event.payload.paths
        if (paths && paths.length > 0) {
          const filePath = paths[0]
          console.log('拖拽文件路径:', filePath)

          // 验证是否为视频文件
          const videoExtensions = ['.mp4', '.mov', '.avi', '.mkv', '.flv', '.wmv', '.webm', '.m4v']
          const isVideo = videoExtensions.some(ext =>
            filePath.toLowerCase().endsWith(ext.toLowerCase())
          )

          if (isVideo) {
            emit('fileSelected', filePath)
          } else {
            alert('不支持的文件类型，请选择视频文件')
          }
        }
      } else if (event.payload.type === 'leave') {
        console.log('拖拽操作已取消')
        isDragging.value = false
      }
    })

    console.log('Tauri 2.0 拖拽事件监听器设置完成')
  } catch (error) {
    console.error('拖拽监听初始化失败:', error)
    alert('拖拽功能初始化失败，请使用点击选择文件')
  }
})

// 清理监听器
onUnmounted(() => {
  if (unlistenDrop) {
    unlistenDrop()
  }
})

// 清除文件
function clearFile() {
  emit('fileCleared')
}
</script>

<style scoped>
.file-selector {
  padding: 0.5rem;
  margin-bottom: 1rem;
}

.compact-drop-zone {
  border: 2px dashed #666;
  border-radius: 8px;
  padding: 1rem;
  text-align: center;
  transition: all 0.3s ease;
  background-color: #1a1a1a;
  cursor: pointer;
  position: relative;
}

.compact-drop-zone:hover,
.compact-drop-zone.dragging {
  border-color: #999;
  background-color: #252525;
}

.drop-zone-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

.drop-icon {
  font-size: 2rem;
  margin-bottom: 0.3rem;
}

.drop-text {
  font-size: 1rem;
  color: #fff;
  margin: 0;
}

.drop-hint {
  font-size: 0.85rem;
  color: #999;
  margin: 0;
}

.current-file {
  background-color: #1a1a1a;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 0.8rem 1rem;
}

.file-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.file-name {
  color: #fff;
  font-size: 0.9rem;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.clear-btn {
  background: #444;
  color: #999;
  border: none;
  border-radius: 4px;
  width: 24px;
  height: 24px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-left: 0.8rem;
  transition: all 0.2s ease;
  font-size: 0.8rem;
}

.clear-btn:hover {
  background: #555;
  color: #fff;
}
</style>
