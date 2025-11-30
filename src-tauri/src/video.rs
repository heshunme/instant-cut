use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub duration: f64,      // 时长（秒）
    pub width: u32,         // 宽度
    pub height: u32,        // 高度
    pub fps: f64,           // 帧率
    pub codec: String,      // 编码格式
    pub format: String,     // 容器格式
}

impl VideoInfo {
    pub fn new(duration: f64, width: u32, height: u32, fps: f64, codec: String, format: String) -> Self {
        Self {
            duration,
            width,
            height,
            fps,
            codec,
            format,
        }
    }
}
