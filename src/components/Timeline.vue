<template>
  <div class="timeline" v-if="duration > 0">
    <div class="timeline-header">
      <span class="timeline-label">选择剪辑范围</span>
      <span class="timeline-duration">{{ formatTime(endTime - startTime) }}</span>
    </div>
    <div class="timeline-track" ref="trackRef" @mousedown="onTrackClick">
      <div class="timeline-bar">
        <div class="timeline-selection" :style="{
          left: `${(startTime / duration) * 100}%`,
          width: `${((endTime - startTime) / duration) * 100}%`
        }" />
        <div class="timeline-handle start" :style="{ left: `${(startTime / duration) * 100}%` }"
          @mousedown.stop="onHandleMouseDown('start', $event)" />
        <div class="timeline-handle end" :style="{ left: `${(endTime / duration) * 100}%` }"
          @mousedown.stop="onHandleMouseDown('end', $event)" />
        <div class="timeline-current" :style="{ left: `${(currentTime / duration) * 100}%` }" />
      </div>
      <div class="timeline-ticks">
        <div v-for="tick in ticks" :key="tick" class="timeline-tick" :style="{ left: `${(tick / duration) * 100}%` }">
          <span class="tick-label">{{ formatTime(tick) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue'

const props = defineProps<{
  duration: number
  currentTime: number
  startTime: number
  endTime: number
}>()

const emit = defineEmits<{
  updateStart: [time: number]
  updateEnd: [time: number]
  previewJump: [time: number]
}>()

const trackRef = ref<HTMLElement | null>(null)
const dragging = ref<'start' | 'end' | null>(null)

const ticks = computed(() => {
  const interval = Math.ceil(props.duration / 10)
  const result: number[] = []
  for (let i = 0; i <= props.duration; i += interval) {
    result.push(i)
  }
  return result
})

function formatTime(seconds: number): string {
  const m = Math.floor(seconds / 60)
  const s = Math.floor(seconds % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}

function onHandleMouseDown(handle: 'start' | 'end', event: MouseEvent) {
  event.preventDefault()
  dragging.value = handle
  document.addEventListener('mousemove', onMouseMove)
  document.addEventListener('mouseup', onMouseUp)
}

function onMouseMove(event: MouseEvent) {
  if (!dragging.value || !trackRef.value) return

  const rect = trackRef.value.getBoundingClientRect()
  const x = Math.max(0, Math.min(event.clientX - rect.left, rect.width))
  const time = (x / rect.width) * props.duration

  if (dragging.value === 'start') {
    emit('updateStart', Math.min(time, props.endTime - 1))
  } else {
    emit('updateEnd', Math.max(time, props.startTime + 1))
  }
}

function onMouseUp() {
  dragging.value = null
  document.removeEventListener('mousemove', onMouseMove)
  document.removeEventListener('mouseup', onMouseUp)
}

function onTrackClick(event: MouseEvent) {
  if (!trackRef.value || event.target !== trackRef.value.querySelector('.timeline-bar')) return

  const rect = trackRef.value.getBoundingClientRect()
  const x = event.clientX - rect.left
  const time = (x / rect.width) * props.duration

  emit('previewJump', time)
}

onUnmounted(() => {
  document.removeEventListener('mousemove', onMouseMove)
  document.removeEventListener('mouseup', onMouseUp)
})
</script>

<style scoped>
.timeline {
  padding: 0.3rem 0.8rem 0.8rem 0.8rem;
  background-color: transparent;
  border-radius: 0;
  border: none;
}

.timeline-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.6rem;
}

.timeline-label {
  color: #999;
  font-size: 0.85rem;
  text-transform: uppercase;
}

.timeline-duration {
  color: #fff;
  font-weight: 600;
}

.timeline-track {
  position: relative;
  padding: 0.3rem 0;
}

.timeline-bar {
  position: relative;
  height: 36px;
  background-color: #2a2a2a;
  border-radius: 4px;
  cursor: pointer;
}

.timeline-selection {
  position: absolute;
  top: 0;
  height: 100%;
  background-color: rgba(255, 255, 255, 0.2);
  border: 2px solid #fff;
  border-radius: 4px;
  pointer-events: none;
}

.timeline-handle {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 16px;
  height: 36px;
  background-color: #fff;
  border-radius: 2px;
  cursor: ew-resize;
  transition: background-color 0.2s ease;
}

.timeline-handle:hover {
  background-color: #e0e0e0;
}

.timeline-current {
  position: absolute;
  top: 0;
  height: 100%;
  width: 2px;
  background-color: #f00;
  pointer-events: none;
}

.timeline-ticks {
  position: relative;
  height: 18px;
  margin-top: 0.3rem;
}

.timeline-tick {
  position: absolute;
  transform: translateX(-50%);
}

.tick-label {
  color: #666;
  font-size: 0.7rem;
  font-family: monospace;
}

@media (max-width: 768px) {
  .timeline-bar {
    height: 32px;
  }

  .timeline-handle {
    height: 32px;
  }

  .timeline-ticks {
    height: 16px;
  }

  .tick-label {
    font-size: 0.65rem;
  }
}
</style>
