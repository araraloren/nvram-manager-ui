use tauri::Emitter;

// 计算文件总大小的函数（模拟）
pub fn calculate_total_size(file_list: &Vec<String>) -> String {
    // 模拟计算文件总大小，实际实现时会遍历文件获取真实大小
    let file_count = file_list.len();
    let total_bytes = file_count * 8192; // 每个文件模拟8KB

    // 格式化大小显示
    if total_bytes < 1024 {
        format!("{} B", total_bytes)
    } else if total_bytes < 1024 * 1024 {
        format!("{:.1} KB", total_bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", total_bytes as f64 / (1024.0 * 1024.0))
    }
}
