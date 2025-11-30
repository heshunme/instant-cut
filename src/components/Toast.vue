<template>
  <Teleport to="body">
    <div class="toast-container">
      <transition-group name="toast" tag="div">
        <div
          v-for="toast in toasts"
          :key="toast.id"
          class="toast"
          :style="{ top: `${toast.index * 60}px` }"
        >
          <div class="toast-content">
            <span class="toast-icon">✓</span>
            <span class="toast-message">{{ toast.message }}</span>
          </div>
        </div>
      </transition-group>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface ToastItem {
  id: number
  message: string
  timestamp: number
  index: number
}

const toasts = ref<ToastItem[]>([])
let nextId = 1
let nextIndex = 0

function showToast(message: string) {
  const toast: ToastItem = {
    id: nextId++,
    message,
    timestamp: Date.now(),
    index: nextIndex++
  }

  toasts.value.push(toast)

  // 3秒后移除
  setTimeout(() => {
    removeToast(toast.id)
  }, 3000)
}

function removeToast(id: number) {
  const index = toasts.value.findIndex(t => t.id === id)
  if (index > -1) {
    // 重新计算后续toast的索引
    const removed = toasts.value.splice(index, 1)[0]
    toasts.value.forEach(toast => {
      if (toast.index > removed.index) {
        toast.index--
      }
    })
    nextIndex--
  }
}

// 暴露方法给父组件
defineExpose({
  showToast
})
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  pointer-events: none;
}

.toast {
  background-color: #1a1a1a;
  color: #ffffff;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 12px 20px;
  margin-bottom: 10px;
  min-width: 250px;
  max-width: 400px;
  position: absolute;
  right: 0;
  pointer-events: auto;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(10px);
}

.toast-content {
  display: flex;
  align-items: center;
  gap: 10px;
}

.toast-icon {
  color: #4CAF50;
  font-weight: bold;
  font-size: 16px;
}

.toast-message {
  flex: 1;
  font-size: 14px;
  line-height: 1.4;
}

/* 动画效果 */
.toast-enter-active {
  transition: all 0.3s ease-out;
}

.toast-leave-active {
  transition: all 0.3s ease-in;
}

.toast-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.toast-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

.toast-move {
  transition: transform 0.3s ease;
}
</style>