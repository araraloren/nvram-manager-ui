use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};

// PS版本结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsVersion {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "build")]
    pub build: String,
    #[serde(rename = "path")]
    pub path: PathBuf,
}

// JChip版本结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JChipVersion {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "build")]
    pub build: String,
    #[serde(rename = "path")]
    pub path: PathBuf,
}

// NVRAM备份结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvramBackup {
    #[serde(rename = "id")]
    pub id: u32,
    #[serde(rename = "nvramName")]
    pub nvram_name: String,
    #[serde(rename = "psVersion")]
    pub ps_version: PsVersion,
    #[serde(rename = "jchipVersion")]
    pub jchip_version: JChipVersion,
    #[serde(rename = "backupTime")]
    pub backup_time: DateTime<Utc>,
    #[serde(rename = "fileList")]
    pub file_list: Vec<String>,
    #[serde(rename = "totalSize")]
    pub total_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Level {
    Error,
    Warn,
    Info,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Error => "error",
            Level::Warn => "warning",
            Level::Info => "info",
        }
    }
}

// 状态更新结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    level: Level,
    message: String,
}

impl AppState {
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            level: Level::Error,
            message: msg.into(),
        }
    }

    pub fn info(msg: impl Into<String>) -> Self {
        Self {
            level: Level::Info,
            message: msg.into(),
        }
    }

    pub fn warn(msg: impl Into<String>) -> Self {
        Self {
            level: Level::Warn,
            message: msg.into(),
        }
    }

    pub fn emit_by(self, handle: &AppHandle) {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StatusUpdate {
            level: String,
            message: String,
        }

        let _ = handle.emit(
            "status_updated",
            StatusUpdate {
                level: self.level.as_str().to_string(),
                message: self.message,
            },
        );
    }
}

// App配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "nvramPath")]
    pub nvram_path: PathBuf,
    #[serde(rename = "backupPath")]
    pub backup_path: PathBuf,
    #[serde(rename = "forceBackup")]
    pub force_backup: bool,
    #[serde(rename = "clearAfterBackup")]
    pub clear_after_backup: bool,
    #[serde(rename = "clearNvramOnRestore")]
    pub clear_nvram_on_restore: bool,
    #[serde(rename = "clearBackupOnRestore")]
    pub clear_backup_on_restore: bool,
}

impl AppConfig {
    pub async fn load(path: &Path) {}
}
