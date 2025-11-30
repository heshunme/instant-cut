<template>
  <div class="timeline-controls">
    <div class="timeline-section">
      <Timeline
        :duration="duration"
        :currentTime="currentTime"
        :startTime="startTime"
        :endTime="endTime"
        @updateStart="handleUpdateStart"
        @updateEnd="handleUpdateEnd"
        @previewJump="handlePreviewJump"
      />
      <div class="comment-section">
        <div class="input-group">
          <label>备注</label>
          <input
            v-model="notesInput"
            placeholder="添加备注内容，下划线将自动替换为短横线"
            @blur="validateNotes"
            :disabled="isProcessing"
          />
          <div v-if="notesError" class="error-message">{{ notesError }}</div>
        </div>
      </div>
    </div>
    <div class="controls-section">
      <div class="time-inputs">
        <div class="input-group">
          <label>开始时间</label>
          <input
            v-model="startInput"
            @blur="updateStartTime"
            @keypress.enter="updateStartTime"
            placeholder="00:00"
          />
        </div>
        <div class="input-group">
          <label>结束时间</label>
          <input
            v-model="endInput"
            @blur="updateEndTime"
            @keypress.enter="updateEndTime"
            placeholder="00:00"
          />
        </div>
      </div>
      <button
        class="cut-button"
        :disabled="!canCut || isProcessing"
        @click="handleCut"
      >
        {{ isProcessing ? '处理中...' : '开始剪辑' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import Timeline from './Timeline.vue'
import { useToast } from '../composables/useToast'
import { formatTimeInput, parseTimeInput, isValidTimeInput } from '../utils/timeUtils'

const props = defineProps<{
  duration: number
  currentTime: number
  startTime: number
  endTime: number
  isProcessing: boolean
  canCut: boolean
}>()

const emit = defineEmits<{
  updateStart: [time: number]
  updateEnd: [time: number]
  cut: [notes?: string]
  previewJump: [time: number]
}>()

const startInput = ref('')
const endInput = ref('')
const notesInput = ref('')
const notesError = ref('')
const { showToast } = useToast()

watch(() => props.startTime, (newTime) => {
  startInput.value = formatTimeInput(newTime)
}, { immediate: true })

watch(() => props.endTime, (newTime) => {
  endInput.value = formatTimeInput(newTime)
}, { immediate: true })

function handleUpdateStart(time: number) {
  emit('updateStart', time)
}

function handleUpdateEnd(time: number) {
  emit('updateEnd', time)
}

function handlePreviewJump(time: number) {
  emit('previewJump', time)
}

function updateStartTime() {
  if (isValidTimeInput(startInput.value)) {
    const time = parseTimeInput(startInput.value)!
    if (time >= 0) {
      emit('updateStart', time)
      showToast('开始时间已更新')
    } else {
      showToast('时间不能为负数')
      startInput.value = formatTimeInput(props.startTime)
    }
  } else {
    showToast('无效的时间格式')
    startInput.value = formatTimeInput(props.startTime)
  }
}

function updateEndTime() {
  if (isValidTimeInput(endInput.value)) {
    const time = parseTimeInput(endInput.value)!
    if (time > 0) {
      emit('updateEnd', time)
      showToast('结束时间已更新')
    } else {
      showToast('时间必须大于0')
      endInput.value = formatTimeInput(props.endTime)
    }
  } else {
    showToast('无效的时间格式')
    endInput.value = formatTimeInput(props.endTime)
  }
}

function validateNotes() {
  if (notesInput.value.length > 50) {
    notesError.value = '备注长度不能超过50个字符'
    return false
  }

  // 检查文件系统非法字符
  const invalidChars = /[<>:"/\\|?*]/
  if (invalidChars.test(notesInput.value)) {
    notesError.value = '备注不能包含文件系统非法字符：<>:"/\\|?*'
    return false
  }

  notesError.value = ''
  return true
}

function handleCut() {
  if (!validateNotes()) {
    showToast('请修正备注内容后再进行剪辑')
    return
  }

  // 传递备注（如果有内容的话）
  const notes = notesInput.value.trim() || undefined
  emit('cut', notes)
}
</script>

<style scoped>
.timeline-controls {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 2rem;
  padding: 1rem;
  background-color: #1a1a1a;
  border-radius: 8px;
  border: 1px solid #333;
}

@media (max-width: 1024px) {
  .timeline-controls {
    grid-template-columns: 1fr;
  }
}

.controls-section {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
  justify-content: space-between;
}

.time-inputs {
  display: grid;
  grid-template-columns: 1fr 1fr; /* Only start/end time inputs */
  gap: 0.8rem;
}

@media (max-width: 1024px) {
  .time-inputs {
    grid-template-columns: 1fr;
  }
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.input-group label {
  color: #999;
  font-size: 0.85rem;
  text-transform: uppercase;
}

.input-group input {
  background-color: #2a2a2a;
  border: 1px solid #444;
  color: #fff;
  padding: 0.6rem;
  border-radius: 4px;
  font-size: 1rem;
  font-family: monospace;
}

.input-group input:focus {
  outline: none;
  border-color: #666;
}

.error-message {
  color: #ff6b6b;
  font-size: 0.75rem;
  margin-top: 0.2rem;
}

.comment-section {
  margin-top: 1rem;
  padding: 0 0.8rem; /* Match timeline padding */
}

.comment-section .input-group {
  width: 100%;
}

.comment-section input {
  width: 100%;
  box-sizing: border-box;
}

@media (max-width: 1024px) {
  .timeline-controls {
    grid-template-columns: 1fr;
  }
}

.cut-button {
  background-color: #fff;
  color: #000;
  border: none;
  padding: 0.8rem 1.5rem;
  font-size: 1.1rem;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 700;
  transition: all 0.2s ease;
}

.cut-button:hover:not(:disabled) {
  background-color: #e0e0e0;
}

.cut-button:active:not(:disabled) {
  transform: scale(0.98);
}

.cut-button:disabled {
  background-color: #444;
  color: #666;
  cursor: not-allowed;
}
</style>