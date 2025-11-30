<template>
  <div class="video-player" v-if="videoPath">
    <video
      ref="videoRef"
      :src="videoSrc"
      @loadedmetadata="onLoadedMetadata"
      @timeupdate="onTimeUpdate"
      @error="onError"
    />
    <div class="controls">
      <button @click="togglePlay" class="control-button">
        {{ isPlaying ? '暂停' : '播放' }}
      </button>
      <div class="time-display">
        {{ formatTime(currentTime) }} / {{ formatTime(duration) }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'

const props = defineProps<{
  videoPath: string | null
  seekTime?: number
}>()

const emit = defineEmits<{
  timeUpdate: [time: number]
  durationChange: [duration: number]
}>()

const videoRef = ref<HTMLVideoElement | null>(null)
const isPlaying = ref(false)
const currentTime = ref(0)
const duration = ref(0)

const videoSrc = computed(() => {
  if (!props.videoPath) return ''
  return convertFileSrc(props.videoPath)
})

watch(() => props.seekTime, (newTime) => {
  if (newTime !== undefined && videoRef.value) {
    videoRef.value.currentTime = newTime
  }
})

function togglePlay() {
  if (!videoRef.value) return

  if (isPlaying.value) {
    videoRef.value.pause()
  } else {
    videoRef.value.play()
  }
  isPlaying.value = !isPlaying.value
}

function onLoadedMetadata() {
  if (videoRef.value) {
    duration.value = videoRef.value.duration
    emit('durationChange', duration.value)
  }
}

function onTimeUpdate() {
  if (videoRef.value) {
    currentTime.value = videoRef.value.currentTime
    emit('timeUpdate', currentTime.value)
  }
}

function onError(e: Event) {
  console.error('视频加载错误:', e)
}

function formatTime(seconds: number): string {
  const m = Math.floor(seconds / 60)
  const s = Math.floor(seconds % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}
</script>

<style scoped>
.video-player {
  background-color: #000;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #333;
  height: 100%;
  display: flex;
  flex-direction: column;
}

video {
  width: 100%;
  height: auto;
  display: block;
  max-height: 300px;
  object-fit: contain;
  flex: 1;
}

.controls {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.6rem 1rem;
  background-color: #1a1a1a;
  flex-shrink: 0;
}

.control-button {
  background-color: #fff;
  color: #000;
  border: none;
  padding: 0.5rem 1.2rem;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  font-size: 0.9rem;
  transition: all 0.2s ease;
}

.control-button:hover {
  background-color: #e0e0e0;
}

.control-button:active {
  transform: scale(0.98);
}

.time-display {
  color: #999;
  font-family: monospace;
  font-size: 0.9rem;
}

@media (max-height: 800px) {
  video {
    max-height: 250px;
  }
  .controls {
    padding: 0.5rem 0.8rem;
  }
  .control-button {
    padding: 0.4rem 1rem;
    font-size: 0.85rem;
  }
  .time-display {
    font-size: 0.85rem;
  }
}
</style>
