// 命令定义在 main.rs 中使用 clap derive
// 这个文件保留用于未来的命令相关辅助功能

/// 命令行参数结构的辅助函数
pub mod helpers {
    /// 验证输出格式
    pub fn validate_output_format(format: &str) -> bool {
        matches!(format.to_lowercase().as_str(), "table" | "json")
    }
}
