/**
 * 时间处理工具函数
 * 统一处理时间格式化和解析逻辑，避免在多个组件中重复代码
 */

/**
 * 将秒数格式化为时间字符串
 * @param seconds 秒数
 * @returns 格式化的时间字符串，支持 HH:MM:SS 或 MM:SS 格式
 */
export function formatTimeInput(seconds: number): string {
  if (seconds < 0) return '00:00'

  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  const s = Math.floor(seconds % 60)

  if (h > 0) {
    return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`
  }
  return `${m}:${s.toString().padStart(2, '0')}`
}

/**
 * 将时间字符串解析为秒数
 * 支持格式：
 * - SS (例如: 30)
 * - MM:SS (例如: 1:30)
 * - HH:MM:SS (例如: 1:30:45)
 * @param input 时间字符串
 * @returns 秒数，解析失败返回 null
 */
export function parseTimeInput(input: string): number | null {
  if (!input || input.trim() === '') return null

  const trimmedInput = input.trim()
  const parts = trimmedInput.split(':').map(p => {
    const num = parseInt(p, 10)
    return isNaN(num) ? null : num
  })

  // 检查是否有无效的部分
  if (parts.some(p => p === null)) return null

  const validParts = parts as number[]

  if (validParts.length === 1) {
    // SS 格式
    return validParts[0] >= 0 ? validParts[0] : null
  } else if (validParts.length === 2) {
    // MM:SS 格式
    const [minutes, seconds] = validParts
    if (minutes >= 0 && seconds >= 0 && seconds < 60) {
      return minutes * 60 + seconds
    }
    return null
  } else if (validParts.length === 3) {
    // HH:MM:SS 格式
    const [hours, minutes, seconds] = validParts
    if (hours >= 0 && minutes >= 0 && minutes < 60 && seconds >= 0 && seconds < 60) {
      return hours * 3600 + minutes * 60 + seconds
    }
    return null
  }

  return null
}

/**
 * 验证时间字符串是否有效
 * @param input 时间字符串
 * @returns 是否有效
 */
export function isValidTimeInput(input: string): boolean {
  return parseTimeInput(input) !== null
}

/**
 * 格式化时间显示（更详细的格式，包含毫秒）
 * @param seconds 秒数
 * @param includeMilliseconds 是否包含毫秒
 * @returns 格式化的时间字符串
 */
export function formatTimeDisplay(seconds: number, includeMilliseconds: boolean = false): string {
  if (seconds < 0) return '00:00:00'

  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  const s = Math.floor(seconds % 60)

  let result = `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`

  if (includeMilliseconds) {
    const ms = Math.floor((seconds % 1) * 1000)
    result += `.${ms.toString().padStart(3, '0')}`
  }

  return result
}

/**
 * 验证时间范围是否有效
 * @param startTime 开始时间（秒）
 * @param endTime 结束时间（秒）
 * @param duration 视频总时长（秒）
 * @returns 验证结果和错误信息
 */
export function validateTimeRange(startTime: number, endTime: number, duration: number): {
  isValid: boolean
  error: string | null
} {
  if (startTime < 0) {
    return { isValid: false, error: '开始时间不能为负数' }
  }

  if (endTime <= startTime) {
    return { isValid: false, error: '结束时间必须大于开始时间' }
  }

  if (startTime > duration) {
    return { isValid: false, error: '开始时间超出视频时长' }
  }

  if (endTime > duration) {
    return { isValid: false, error: '结束时间超出视频时长' }
  }

  return { isValid: true, error: null }
}

/**
 * 计算时间段的持续时间
 * @param startTime 开始时间（秒）
 * @param endTime 结束时间（秒）
 * @returns 持续时间（秒）
 */
export function calculateDuration(startTime: number, endTime: number): number {
  return Math.max(0, endTime - startTime)
}

/**
 * 将时间段格式化为可读的描述
 * @param startTime 开始时间（秒）
 * @param endTime 结束时间（秒）
 * @returns 时间段描述
 */
export function formatDurationDescription(startTime: number, endTime: number): string {
  const duration = calculateDuration(startTime, endTime)

  if (duration === 0) {
    return '无效的时间段'
  }

  const h = Math.floor(duration / 3600)
  const m = Math.floor((duration % 3600) / 60)
  const s = Math.floor(duration % 60)

  const parts: string[] = []

  if (h > 0) {
    parts.push(`${h}小时`)
  }
  if (m > 0) {
    parts.push(`${m}分钟`)
  }
  if (s > 0 || parts.length === 0) {
    parts.push(`${s}秒`)
  }

  return parts.join('')
}

/**
 * 安全地解析时间输入，提供默认值
 * @param input 输入字符串
 * @param defaultValue 默认值
 * @returns 解析后的值或默认值
 */
export function safeParseTimeInput(input: string, defaultValue: number = 0): number {
  const parsed = parseTimeInput(input)
  return parsed !== null ? parsed : defaultValue
}