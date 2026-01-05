use crate::modules::{AppHandleExt, *};

use std::path::PathBuf;
use tauri::{AppHandle, State};
use tracing::{debug, info};

// 获取配置信息的命令
#[tauri::command]
pub fn get_app_config(config: State<AppConfig>) -> AppConfig {
    // 这里暂时返回一个默认配置，后面会从文件读取
    config.inner().clone()
}

// 获取当前NVRAM信息的命令
#[tauri::command]
pub fn get_current_nvram_info_command() -> NvramBackup {
    // 从配置中获取NVRAM路径，实际实现时会从配置文件读取
    let nvram_path = PathBuf::from(r"C:\opt\nvram");
    get_current_nvram_info(&nvram_path)
}

// 检查NVRAM是否存在的命令
#[tauri::command]
pub fn check_nvram_existence() -> bool {
    // // 从配置中获取NVRAM路径，实际实现时会从配置文件读取
    // let nvram_path = PathBuf::from(r"C:\opt\nvram");

    // // 检查NVRAM路径是否存在
    // if !nvram_path.exists() {
    //     return false;
    // }

    // // 检查路径是否是目录
    // if !nvram_path.is_dir() {
    //     return false;
    // }

    // // 检查目录下是否有文件
    // match std::fs::read_dir(nvram_path) {
    //     Ok(mut entries) => {
    //         // 如果目录下有至少一个文件，则认为NVRAM存在
    //         entries.next().is_some()
    //     }
    //     Err(_) => false,
    // }
    false
}

// 获取当前游戏路径的命令
#[tauri::command]
pub fn get_current_game_path() -> String {
    // 实际实现时，这里应该从配置或系统中获取当前游戏路径
    // 目前返回一个默认路径
    String::from(r"C:\Games\CurrentGame")
}

// 验证游戏路径匹配的命令
#[tauri::command]
pub fn validate_game_path(
    backup_id: u32,
    list: State<NvramBackupList>,
) -> GamePathValidationResult {
    // 从配置或系统中获取当前游戏路径
    let current_game_path = get_current_game_path();

    // 根据备份ID查找对应的备份信息
    let backup = list
        .inner()
        .nvrams
        .iter()
        .find(|b| b.id == backup_id)
        .unwrap_or_else(|| {
            // 如果找不到备份，返回第一个备份
            &list.inner().nvrams[0]
        });

    // 获取备份的游戏路径
    let backup_game_path = backup.game_path.to_string_lossy().to_string();

    // 比较当前游戏路径与备份的游戏路径是否匹配
    let is_match = current_game_path == backup_game_path;

    // 返回验证结果
    GamePathValidationResult {
        is_match,
        current_path: current_game_path,
        backup_path: backup_game_path,
    }
}

// 获取备份列表的命令
#[tauri::command]
pub fn get_backup_list(list: State<NvramBackupList>) -> Vec<NvramBackup> {
    // 模拟从本地文件读取备份列表，目前使用随机生成的备份信息
    list.inner().nvrams.clone()
}

// 备份操作命令
#[tauri::command]
pub fn backup_nvram(handle: AppHandle, clear_after: bool) -> Result<bool, String> {
    // 发送开始备份状态
    handle.update_state_info("开始执行NVRAM备份操作");

    info!("执行备份操作: clear_after={}", clear_after);
    debug!("备份操作详细参数: clear_after={}", clear_after);

    // 模拟备份操作，实际实现时会执行备份逻辑并同步到磁盘

    // 发送备份完成状态
    handle.update_state_info("NVRAM备份操作完成");
    info!("备份操作完成");
    Ok(true)
}

// 还原操作命令
#[tauri::command]
pub fn restore_backup(handle: AppHandle, backup_id: u32) -> Result<bool, String> {
    // 发送开始还原状态
    handle.update_state_info("开始执行NVRAM还原操作");

    info!("执行还原操作: backup_id={}", backup_id);
    debug!("还原操作详细参数: backup_id={}", backup_id);

    // 模拟还原操作，实际实现时会执行还原逻辑并同步到磁盘

    // 发送还原完成状态
    handle.update_state_info("NVRAM还原操作完成");
    info!("还原操作完成");
    Ok(true)
}

// 删除备份操作命令
#[tauri::command]
pub fn delete_backup(app_handle: AppHandle, backup_id: u32) -> Result<bool, String> {
    // 发送开始删除状态
    app_handle.update_state_info(format!("开始执行删除备份操作: ID={}", backup_id));

    info!("执行删除备份操作: backup_id={}", backup_id);
    debug!("删除备份操作详细参数: backup_id={}", backup_id);

    // 模拟删除操作，实际实现时会执行删除逻辑并同步到磁盘

    // 发送删除完成状态
    app_handle.update_state_info(format!("删除备份操作完成: ID={}", backup_id));
    info!("删除备份操作完成");
    Ok(true)
}
