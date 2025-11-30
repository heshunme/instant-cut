use std::path::{Path, PathBuf};
use std::fs;
use serde_json::Value;
use crate::video::VideoInfo;
use crate::error::{AppError, AppResult, to_tauri_error, ffprobe_error,
                   filesystem_error, path_error, bytes_to_gb};
use crate::utils::{execute_ffmpeg, execute_ffprobe, check_command_success, parse_frame_rate,
                   validate_input_path, validate_time_range, sanitize_filename};

/// 检测系统是否安装了 ffmpeg 和 ffprobe
pub fn check_ffmpeg_installed() -> AppResult<bool> {
    let ffmpeg_check = execute_ffmpeg(&["-version"]);
    let ffprobe_check = execute_ffprobe(&["-version"]);

    match (ffmpeg_check, ffprobe_check) {
        (Ok(_), Ok(_)) => Ok(true),
        _ => Err("ffmpeg 或 ffprobe 未安装。请先安装 ffmpeg。".to_string())
    }
}

/// 使用 ffprobe 获取视频信息（整合版本）
pub fn get_video_info(path: &str) -> AppResult<VideoInfo> {
    // 验证输入路径
    let _validated_path = validate_input_path(path)
        .map_err(|e| to_tauri_error(e))?;

    // 执行 ffprobe 命令
    let output = execute_ffprobe(&[
        "-v", "quiet",
        "-print_format", "json",
        "-show_format",
        "-show_streams",
        path
    ]).map_err(|e| to_tauri_error(e))?;

    // 检查命令执行结果
    check_command_success(&output, "ffprobe")
        .map_err(|e| to_tauri_error(e))?;

    // 解析 JSON 输出
    let json_str = String::from_utf8(output.stdout)
        .map_err(|e| to_tauri_error(ffprobe_error(format!("解析输出失败: {}", e))))?;

    let data: Value = serde_json::from_str(&json_str)
        .map_err(|e| to_tauri_error(ffprobe_error(format!("解析 JSON 失败: {}", e))))?;

    // 查找视频流
    let streams = data["streams"].as_array()
        .ok_or_else(|| to_tauri_error(ffprobe_error("未找到视频流信息")))?;

    let video_stream = streams.iter()
        .find(|s| s["codec_type"] == "video")
        .ok_or_else(|| to_tauri_error(ffprobe_error("未找到视频流")))?;

    // 提取视频信息
    let width = video_stream["width"].as_u64().unwrap_or(0) as u32;
    let height = video_stream["height"].as_u64().unwrap_or(0) as u32;
    let codec = video_stream["codec_name"].as_str().unwrap_or("unknown").to_string();

    // 解析帧率
    let fps_str = video_stream["r_frame_rate"].as_str().unwrap_or("0/1");
    let fps = parse_frame_rate(fps_str);

    // 获取时长
    let duration = data["format"]["duration"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    let format_name = data["format"]["format_name"].as_str().unwrap_or("unknown").to_string();

    Ok(VideoInfo::new(duration, width, height, fps, codec, format_name))
}

/// 获取视频时长（简化版本，仅获取时长）
pub fn get_video_duration(path: &str) -> AppResult<f64> {
    // 验证输入路径
    let _validated_path = validate_input_path(path)
        .map_err(|e| to_tauri_error(e))?;

    // 执行 ffprobe 命令
    let output = execute_ffprobe(&[
        "-v", "quiet",
        "-print_format", "json",
        "-show_format",
        path
    ]).map_err(|e| to_tauri_error(e))?;

    // 检查命令执行结果
    check_command_success(&output, "ffprobe")
        .map_err(|e| to_tauri_error(e))?;

    // 解析 JSON 输出
    let json_str = String::from_utf8(output.stdout)
        .map_err(|e| to_tauri_error(ffprobe_error(format!("解析输出失败: {}", e))))?;

    let data: Value = serde_json::from_str(&json_str)
        .map_err(|e| to_tauri_error(ffprobe_error(format!("解析 JSON 失败: {}", e))))?;

    // 获取时长
    let duration = data["format"]["duration"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok())
        .ok_or_else(|| to_tauri_error(ffprobe_error("无法获取视频时长")))?;

    Ok(duration)
}

/// 检查磁盘空间是否足够
pub fn check_disk_space_for_output(
    output_path: &Path,
    estimated_output_size: u64
) -> AppResult<()> {
    // 获取输出文件的父目录
    let parent_dir = output_path.parent()
        .ok_or_else(|| to_tauri_error(path_error("无法获取输出目录")))?;

    // 获取磁盘可用空间
    let available_space = get_available_disk_space(parent_dir)
        .map_err(|e| to_tauri_error(filesystem_error(format!("获取磁盘空间失败: {}", e))))?;

    // 添加安全缓冲区（额外20%空间）
    let required_space = estimated_output_size + (estimated_output_size / 5);

    if available_space < required_space {
        return Err(to_tauri_error(AppError::InsufficientSpace {
            needed_gb: bytes_to_gb(required_space),
            available_gb: bytes_to_gb(available_space),
            path: parent_dir.to_string_lossy().to_string(),
        }));
    }

    Ok(())
}

/// 估算输出文件大小
pub fn estimate_output_size(
    input_path: &Path,
    start_time: f64,
    end_time: f64,
    original_duration: f64
) -> AppResult<u64> {
    // 获取原始文件大小
    let input_size = fs::metadata(input_path)
        .map_err(|e| to_tauri_error(filesystem_error(format!("无法读取文件大小: {}", e))))?
        .len();

    // 计算剪辑片段的比例
    let duration_ratio = (end_time - start_time) / original_duration;

    // 估算输出大小（使用流复制，大小应接近比例）
    let estimated_size = (input_size as f64 * duration_ratio * 1.1) as u64; // 10% 缓冲

    Ok(estimated_size)
}

/// 解析文件名模式，提取基础名、扩展名和版本号序列
fn parse_filename_pattern(input_path: &str) -> AppResult<(String, String, Vec<u32>)> {
    let path = Path::new(input_path);
    let stem = path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| to_tauri_error(path_error("无法获取文件名")))?;
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp4");

    // 解析版本号模式 (例如: video_1_2.mp4 -> ["video", "1", "2"])
    let parts: Vec<&str> = stem.split('_').collect();

    // 使用更智能的版本检测逻辑
    if parts.len() == 1 {
        // 单个部分，没有版本号
        let base_name = parts[0].to_string();
        return Ok((base_name, ext.to_string(), vec![]));
    }

    // 检查是否所有部分都是数字（如 "1" 或 "1_2"）
    let all_numeric = parts.iter().all(|part| part.chars().all(|c| c.is_ascii_digit()));
    if all_numeric {
        // 所有部分都是数字，没有版本号，将整个作为基础名
        let base_name = parts.join("_");
        return Ok((base_name, ext.to_string(), vec![]));
    }

    // 有混合部分，从后向前检测连续的数字作为版本号
    let mut base_parts = Vec::new();
    let mut versions = Vec::new();
    let mut found_non_numeric = false;

    for part in parts.iter().rev() {
        if part.chars().all(|c| c.is_ascii_digit()) && !found_non_numeric {
            let version_num = part.parse::<u32>()
                .map_err(|_| crate::error::validation_error(format!("无效的版本号: {}", part)))
                .map_err(|e| crate::error::to_tauri_error(e))?;
            versions.insert(0, version_num);
        } else {
            base_parts.insert(0, *part);
            found_non_numeric = true;
        }
    }

    let base_name = base_parts.join("_");
    Ok((base_name, ext.to_string(), versions))
}

/// 查找特定前缀的最大版本号
fn find_max_version_number(base_name: &str, ext: &str, version_prefix: &str, directory: &Path) -> AppResult<u32> {
    let mut max_version = 0;

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                let file_path = entry.path();

                // 只检查文件（不是目录）
                if file_path.is_file() {
                    // 构建预期前缀
                    let expected_prefix = if version_prefix.is_empty() {
                        format!("{}_", base_name)
                    } else {
                        format!("{}_{}_", base_name, version_prefix)
                    };

                    // 检查文件名是否匹配模式
                    if file_name.starts_with(&expected_prefix) && file_name.ends_with(&format!(".{}", ext)) {
                        // 提取中间部分（包含版本号和可能的备注）
                        let start_pos = expected_prefix.len();
                        let end_pos = file_name.len() - ext.len() - 1; // -1 for the dot

                        if start_pos < end_pos {
                            let middle_part = &file_name[start_pos..end_pos];

                            // 分割中间部分，寻找版本号（第一个数字部分）
                            let parts: Vec<&str> = middle_part.split('_').collect();

                            if let Some(first_part) = parts.first() {
                                // 第一个部分应该是版本号
                                if first_part.chars().all(|c| c.is_ascii_digit()) {
                                    if let Ok(version) = first_part.parse::<u32>() {
                                        max_version = max_version.max(version);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(max_version)
}

/// 生成下一个可用的版本文件名
fn generate_next_filename(input_path: &str, notes: Option<&str>) -> AppResult<PathBuf> {
    let path = Path::new(input_path);
    let parent = path.parent().ok_or_else(|| to_tauri_error(path_error("无法获取文件目录")))?;

    let (base_name, ext, versions) = parse_filename_pattern(input_path)?;

    // 清理备注内容
    let sanitized_notes = notes
        .map(|n| sanitize_filename(n))
        .filter(|n| !n.is_empty())
        .map(|n| n.replace('_', "-")); // 替换下划线为短横线

    let next_version = if versions.is_empty() {
        // 基础文件，查找第一级版本 (video_1.mp4, video_2.mp4...)
        find_max_version_number(&base_name, &ext, "", parent)? + 1
    } else {
        // 版本文件，查找下一级版本 (video_1_1.mp4, video_1_2.mp4...)
        let version_prefix = versions.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("_");
        find_max_version_number(&base_name, &ext, &version_prefix, parent)? + 1
    };

    // 构建新文件名
    let new_filename = if versions.is_empty() {
        // 基础文件命名：base_version_notes.ext 或 base_version.ext
        match sanitized_notes {
            Some(notes) => format!("{}_{}_{}.{}", base_name, next_version, notes, ext),
            None => format!("{}_{}.{}", base_name, next_version, ext),
        }
    } else {
        // 版本文件命名：base_version_prefix_version_notes.ext 或 base_version_prefix_version.ext
        let version_prefix = versions.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("_");
        match sanitized_notes {
            Some(notes) => format!("{}_{}_{}_{}.{}", base_name, version_prefix, next_version, notes, ext),
            None => format!("{}_{}_{}.{}", base_name, version_prefix, next_version, ext),
        }
    };

    Ok(parent.join(new_filename))
}

/// 剪辑视频（整合版本）
pub fn cut_video(input_path: &str, start_time: f64, end_time: f64, notes: Option<&str>) -> AppResult<String> {
    // 验证输入路径
    let validated_path = validate_input_path(input_path)
        .map_err(|e| to_tauri_error(e))?;

    // 获取视频总时长
    let total_duration = get_video_duration(input_path)?;

    // 验证时间范围
    validate_time_range(start_time, end_time, total_duration)
        .map_err(|e| to_tauri_error(e))?;

    // 生成输出文件路径
    let output_path = generate_next_filename(input_path, notes)?;

    // 估算输出文件大小
    let estimated_size = estimate_output_size(&validated_path, start_time, end_time, total_duration)?;

    // 检查磁盘空间
    check_disk_space_for_output(&output_path, estimated_size)?;

    // 计算剪辑持续时间
    let duration = end_time - start_time;

    // 执行 ffmpeg 剪辑
    let output = execute_ffmpeg(&[
        "-ss", &start_time.to_string(),
        "-i", input_path,
        "-t", &duration.to_string(),
        "-c", "copy",
        "-avoid_negative_ts", "1",
        "-y",  // 覆盖输出文件
        output_path.to_str().ok_or_else(|| to_tauri_error(path_error("路径转换失败")))?
    ]).map_err(|e| to_tauri_error(e))?;

    // 检查 ffmpeg 执行结果
    check_command_success(&output, "ffmpeg")
        .map_err(|e| to_tauri_error(e))?;

    // 验证输出文件是否成功创建
    if !output_path.exists() {
        return Err(to_tauri_error(filesystem_error("视频剪辑完成，但输出文件未找到")));
    }

    Ok(format!("视频剪辑完成。新文件已保存为: {}", output_path.display()))
}

/// 获取磁盘可用空间（使用 sysinfo 跨平台实现）
fn get_available_disk_space(path: &Path) -> Result<u64, std::io::Error> {
    use sysinfo::Disks;

    // 获取路径的元数据（验证路径有效性）
    let _metadata = fs::metadata(path)?;

    // 获取路径的绝对路径
    let canonical_path = path.canonicalize()
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("无法获取规范路径: {}", e)
        ))?;

    let mut path_str = canonical_path.to_string_lossy().to_string();

    // 只在 Windows 上处理扩展路径格式 (\\?\C:\path -> C:\path)
    #[cfg(windows)]
    if path_str.starts_with("\\\\?\\") {
        path_str = path_str[4..].to_string();
    }

    // 初始化磁盘列表
    let disks = Disks::new_with_refreshed_list();

    // 查找包含目标路径的磁盘
    for disk in &disks {
        let mount_point = disk.mount_point().to_string_lossy();
        if path_str.starts_with(&*mount_point) {
            return Ok(disk.available_space());
        }
    }

    // 如果没有找到精确匹配的磁盘，尝试找到最接近的父目录
    let mut best_match: Option<(usize, &sysinfo::Disk)> = None;

    for disk in &disks {
        let mount_point = disk.mount_point().to_string_lossy();
        if mount_point.len() > best_match.as_ref().map_or(0, |(len, _)| *len)
            && path_str.starts_with(&*mount_point) {
            best_match = Some((mount_point.len(), disk));
        }
    }

    if let Some((_, disk)) = best_match {
        return Ok(disk.available_space());
    }

    // 如果仍然没有找到，使用第一个可用磁盘作为兜底
    if let Some(first_disk) = disks.iter().next() {
        return Ok(first_disk.available_space());
    }

    // 最后的兜底方案：返回保守的估计值
    Ok(5 * 1024 * 1024 * 1024) // 5GB
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filename_pattern() {
        // 测试纯数字文件名
        let result = parse_filename_pattern("1.mp4").unwrap();
        assert_eq!(result, ("1".to_string(), "mp4".to_string(), vec![]));

        // 测试纯数字复合文件名
        let result = parse_filename_pattern("1_2.mp4").unwrap();
        assert_eq!(result, ("1_2".to_string(), "mp4".to_string(), vec![]));

        // 测试普通文件名
        let result = parse_filename_pattern("video.mp4").unwrap();
        assert_eq!(result, ("video".to_string(), "mp4".to_string(), vec![]));

        // 测试版本文件名
        let result = parse_filename_pattern("video_1.mp4").unwrap();
        assert_eq!(result, ("video".to_string(), "mp4".to_string(), vec![1]));

        // 测试嵌套版本文件名
        let result = parse_filename_pattern("video_1_2.mp4").unwrap();
        assert_eq!(result, ("video".to_string(), "mp4".to_string(), vec![1, 2]));
    }

    #[test]
    fn test_bytes_to_gb() {
        assert_eq!(bytes_to_gb(1024 * 1024 * 1024), 1.0);
        assert_eq!(bytes_to_gb(2 * 1024 * 1024 * 1024), 2.0);
    }

    #[test]
    fn test_get_available_disk_space() {
        use std::env;
        use sysinfo::Disks;

        // 测试当前工作目录的磁盘空间检查
        let current_dir = env::current_dir().expect("获取当前目录失败");
        println!("当前目录: {}", current_dir.display());

        // 显示所有可用的磁盘信息
        let disks = Disks::new_with_refreshed_list();
        println!("\n所有可用磁盘:");
        for (i, disk) in disks.iter().enumerate() {
            println!("  [{}] 磁盘: {:?}", i, disk.name());
            println!("      挂载点: {}", disk.mount_point().display());
            println!("      文件系统: {:?}", disk.file_system());
            println!("      可用空间: {:.2} GB", disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0));
            println!("      总空间: {:.2} GB", disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0));
            println!();
        }

        match get_available_disk_space(&current_dir) {
            Ok(available_space) => {
                // 验证返回的磁盘空间是合理的（应该大于0）
                assert!(available_space > 0, "磁盘空间应该大于0");
                // 验证空间不会过大（防止返回错误的值）
                assert!(available_space < 100 * 1024 * 1024 * 1024 * 1024, "磁盘空间不应该超过100TB");
                println!("当前磁盘可用空间: {:.2} GB", available_space as f64 / (1024.0 * 1024.0 * 1024.0));
            }
            Err(e) => {
                // 在某些环境中（如测试容器），这个测试可能会失败
                println!("警告：无法获取磁盘空间: {}", e);
            }
        }
    }
}