<template>
  <div class="video-info" v-if="info">
    <div class="info-grid">
      <div class="info-item">
        <span class="info-label">时长</span>
        <span class="info-value">{{ formatDuration(info.duration) }}</span>
      </div>
      <div class="info-item">
        <span class="info-label">分辨率</span>
        <span class="info-value">{{ info.width }}x{{ info.height }}</span>
      </div>
      <div class="info-item">
        <span class="info-label">帧率</span>
        <span class="info-value">{{ info.fps.toFixed(2) }} fps</span>
      </div>
      <div class="info-item">
        <span class="info-label">编码</span>
        <span class="info-value">{{ info.codec }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { VideoInfo } from '../types'

defineProps<{
  info: VideoInfo | null
}>()

function formatDuration(seconds: number): string {
  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  const s = Math.floor(seconds % 60)

  if (h > 0) {
    return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`
  }
  return `${m}:${s.toString().padStart(2, '0')}`
}
</script>

<style scoped>
.video-info {
  padding: 1rem;
  background-color: #1a1a1a;
  border-radius: 8px;
  border: 1px solid #333;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 0.8rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  padding: 0.5rem 0;
  border-bottom: 1px solid #333;
}

.info-item:last-child {
  border-bottom: none;
}

.info-label {
  color: #999;
  font-size: 0.85rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.info-value {
  color: #fff;
  font-size: 1rem;
  font-weight: 500;
}

@media (min-width: 1024px) {
  .video-info {
    min-height: 300px;
  }
}

@media (max-width: 1024px) {
  .video-info {
    height: auto;
  }

  .info-grid {
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  }

  .info-item {
    border-bottom: none;
    padding: 0;
  }
}
</style>
