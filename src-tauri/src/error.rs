use std::fmt;

/// 应用程序的统一错误类型
#[derive(Debug)]
pub enum AppError {
    /// FFmpeg 相关错误
    FFmpegError(String),

    /// FFprobe 相关错误
    FFprobeError(String),

    /// 文件系统错误
    FilesystemError(String),

    /// 存储空间不足错误
    InsufficientSpace {
        needed_gb: f64,
        available_gb: f64,
        path: String
    },

    /// JSON 解析错误
    JsonError(String),

    /// 输入验证错误
    ValidationError(String),

    /// IO 错误
    IoError(std::io::Error),

    /// 路径相关错误
    PathError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::FFmpegError(msg) => write!(f, "FFmpeg 错误: {}", msg),
            AppError::FFprobeError(msg) => write!(f, "FFprobe 错误: {}", msg),
            AppError::FilesystemError(msg) => write!(f, "文件系统错误: {}", msg),
            AppError::InsufficientSpace { needed_gb, available_gb, path } => {
                write!(f, "磁盘空间不足。需要: {:.2} GB，可用: {:.2} GB，路径: {}",
                       needed_gb, available_gb, path)
            }
            AppError::JsonError(msg) => write!(f, "JSON 解析错误: {}", msg),
            AppError::ValidationError(msg) => write!(f, "输入验证错误: {}", msg),
            AppError::IoError(err) => write!(f, "IO 错误: {}", err),
            AppError::PathError(msg) => write!(f, "路径错误: {}", msg),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::JsonError(format!("JSON 解析失败: {}", err))
    }
}

/// 用于 Tauri 命令的 Result 类型别名
pub type AppResult<T> = Result<T, String>;

/// 将 AppError 转换为 Tauri 命令可接受的 String 错误
pub fn to_tauri_error(err: AppError) -> String {
    err.to_string()
}

/// 创建 FFmpeg 相关错误
pub fn ffmpeg_error<S: Into<String>>(msg: S) -> AppError {
    AppError::FFmpegError(msg.into())
}

/// 创建 FFprobe 相关错误
pub fn ffprobe_error<S: Into<String>>(msg: S) -> AppError {
    AppError::FFprobeError(msg.into())
}

/// 创建文件系统相关错误
pub fn filesystem_error<S: Into<String>>(msg: S) -> AppError {
    AppError::FilesystemError(msg.into())
}

/// 创建验证错误
pub fn validation_error<S: Into<String>>(msg: S) -> AppError {
    AppError::ValidationError(msg.into())
}

/// 创建路径相关错误
pub fn path_error<S: Into<String>>(msg: S) -> AppError {
    AppError::PathError(msg.into())
}

/// 将字节转换为 GB
pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::FFmpegError("执行失败".to_string());
        assert_eq!(err.to_string(), "FFmpeg 错误: 执行失败");
    }

    #[test]
    fn test_insufficient_space_error() {
        let err = AppError::InsufficientSpace {
            needed_gb: 10.5,
            available_gb: 5.0,
            path: "/tmp".to_string()
        };
        let display = err.to_string();
        assert!(display.contains("磁盘空间不足"));
        assert!(display.contains("10.50 GB"));
        assert!(display.contains("5.00 GB"));
    }

    #[test]
    fn test_bytes_to_gb() {
        assert_eq!(bytes_to_gb(1024 * 1024 * 1024), 1.0);
        assert_eq!(bytes_to_gb(2 * 1024 * 1024 * 1024), 2.0);
    }
}