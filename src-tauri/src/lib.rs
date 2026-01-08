// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use tauri::{Emitter, Manager};
use tracing::{debug, error, info, trace, warn};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// 导入自定义模块
mod modules;
use modules::*;

use crate::modules::AppHandleExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统
    if cfg!(debug_assertions) {
        // Debug模式：输出到命令行，彩色格式化
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
            )
            .with_ansi(true)
            .init();
    } else {
        // Release模式：输出到文件，按天滚动
        let app_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("logs");
        let file_appender = rolling::daily(app_dir, "nvram-manager");
        let (non_blocking, _guard) = non_blocking(file_appender);

        tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")))
            .with(fmt::Layer::new().with_writer(non_blocking))
            .init();
    }

    info!("NVRAM Manager 应用启动");

    tauri::Builder::default()
        .manage(AppConfig::load_from_exe_path())
        .manage(generate_backups())
        .invoke_handler(tauri::generate_handler!(
            modules::get_app_config,
            modules::get_backup_list,
            modules::backup_nvram,
            modules::restore_backup,
            modules::delete_backup,
            modules::get_current_nvram_info_command,
            modules::check_nvram_existence,
            modules::validate_game_path
        ))
        .setup(|app| {
            // 创建应用程序的引用，用于在监听器中访问
            let app_handle = app.app_handle().clone();

            // 从配置中获取NVRAM路径，实际实现时会从配置文件读取
            let nvram_path = PathBuf::from(r"D:\Tools");
            let watch_path = nvram_path.clone();

            // 检查目录是否存在，如果不存在则跳过监听
            if watch_path.exists() {
                debug!("开始监听NVRAM目录: {:?}", watch_path);

                // 创建一个克隆的app_handle和watch_path用于闭包
                let app_handle_clone = app_handle.clone();
                let watch_path_clone = watch_path.clone();

                // 创建文件监听器
                let mut watcher = RecommendedWatcher::new(
                    move |res| {
                        match res {
                            Ok(event) => {
                                info!("NVRAM目录发生变化: {:?}", event);
                                debug!("NVRAM目录发生变化: {:?}", event);

                                // 发送目录变化状态
                                app_handle_clone.update_state_info("NVRAM目录发生变化");

                                // 获取更新后的NVRAM信息
                                let updated_info = get_current_nvram_info(&watch_path_clone);

                                // 发送事件通知前端
                                let _ = app_handle_clone.emit("nvram_info_updated", updated_info);
                            }
                            Err(e) => {
                                error!("NVRAM目录监听错误: {:?}", e);
                                // 发送监听错误状态
                                app_handle_clone
                                    .update_state_error(format!("NVRAM目录监听错误: {:?}", e));
                            }
                        }
                    },
                    Config::default(),
                )?;

                // 开始监听NVRAM目录
                watcher.watch(&watch_path, RecursiveMode::Recursive)?;

                // 将watcher存储到堆上，延长其生命周期，直到应用退出
                std::mem::forget(watcher);
            } else {
                debug!("警告: NVRAM目录 {:?} 不存在，跳过监听", watch_path);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
