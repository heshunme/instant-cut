use std::process::Command;
use std::path::{Path, PathBuf};
use crate::error::{AppError, ffmpeg_error, ffprobe_error};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// 执行 FFmpeg 命令的通用函数
pub fn execute_ffmpeg(args: &[&str]) -> Result<std::process::Output, AppError> {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(args);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    cmd.output().map_err(|e| ffmpeg_error(format!("执行 FFmpeg 命令失败: {}", e)))
}

/// 执行 FFprobe 命令的通用函数
pub fn execute_ffprobe(args: &[&str]) -> Result<std::process::Output, AppError> {
    let mut cmd = Command::new("ffprobe");
    cmd.args(args);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    cmd.output().map_err(|e| ffprobe_error(format!("执行 FFprobe 命令失败: {}", e)))
}

/// 检查命令输出是否成功
pub fn check_command_success(output: &std::process::Output, command_name: &str) -> Result<(), AppError> {
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ffprobe_error(format!("{} 执行失败: {}", command_name, stderr)));
    }
    Ok(())
}

/// 解析帧率字符串 (例如 "30000/1001" -> 29.97)
pub fn parse_frame_rate(fps_str: &str) -> f64 {
    let parts: Vec<&str> = fps_str.split('/').collect();
    if parts.len() == 2 {
        if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            if den != 0.0 {
                return num / den;
            }
        }
    }
    0.0
}

/// 验证输入文件路径
pub fn validate_input_path(path: &str) -> Result<PathBuf, AppError> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(crate::error::path_error(format!("文件不存在: {}", path.display())));
    }

    if !path.is_file() {
        return Err(crate::error::path_error(format!("路径不是文件: {}", path.display())));
    }

    Ok(path.to_path_buf())
}

/// 验证时间参数
pub fn validate_time_range(start: f64, end: f64, duration: f64) -> Result<(), AppError> {
    if start < 0.0 {
        return Err(crate::error::validation_error("开始时间不能为负数".to_string()));
    }

    if end <= start {
        return Err(crate::error::validation_error("结束时间必须大于开始时间".to_string()));
    }

    if start > duration {
        return Err(crate::error::validation_error("开始时间超出视频时长".to_string()));
    }

    if end > duration {
        return Err(crate::error::validation_error("结束时间超出视频时长".to_string()));
    }

    Ok(())
}

/// 生成安全的文件名
/// 对于本地应用，只过滤真正会导致问题的字符
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            // Windows 系统不允许的字符
            '<' | '>' | '"' | '|' | '?' | '*' => '_',
            // 路径分隔符
            '/' | '\\' => '_',
            // 控制字符 (0-31)
            _ if c.is_control() => '_',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frame_rate() {
        assert_eq!(parse_frame_rate("30000/1001"), 29.97002997002997);
        assert_eq!(parse_frame_rate("25/1"), 25.0);
        assert_eq!(parse_frame_rate("30/1"), 30.0);
        assert_eq!(parse_frame_rate("invalid"), 0.0);
        assert_eq!(parse_frame_rate("10/0"), 0.0);
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("test<file>name"), "test_file_name");
        assert_eq!(sanitize_filename("normal filename"), "normal filename");
        assert_eq!(sanitize_filename("path/to\\file"), "path_to_file");
        // 冒号被保留
        assert_eq!(sanitize_filename("video:part1"), "video:part1");
        // 控制字符被过滤
        assert_eq!(sanitize_filename("test\u{0001}control"), "test_control");
        // Windows保留字符被过滤
        assert_eq!(sanitize_filename("file|name"), "file_name");
    }

    #[test]
    fn test_validate_time_range() {
        // 正常范围
        assert!(validate_time_range(0.0, 10.0, 15.0).is_ok());

        // 错误范围
        assert!(validate_time_range(-1.0, 10.0, 15.0).is_err());
        assert!(validate_time_range(10.0, 5.0, 15.0).is_err());
        assert!(validate_time_range(20.0, 30.0, 15.0).is_err());
    }
}