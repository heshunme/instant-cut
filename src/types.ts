export interface VideoInfo {
  duration: number
  width: number
  height: number
  fps: number
  codec: string
  format: string
}

export interface AppState {
  selectedFile: string | null
  videoInfo: VideoInfo | null
  startTime: number
  endTime: number
  currentTime: number
  isProcessing: boolean
}
