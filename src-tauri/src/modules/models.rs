use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    #[serde(rename = "gamePath")]
    pub game_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvramBackupList {
    pub nvrams: Vec<NvramBackup>,
}

// 游戏路径验证结果结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePathValidationResult {
    pub is_match: bool,
    pub current_path: String,
    pub backup_path: String,
}

// App配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "nvramPath")]
    pub nvram_path: PathBuf,
    #[serde(rename = "backupPath")]
    pub backup_path: PathBuf,
    #[serde(rename = "clearAfterBackup")]
    pub clear_after_backup: bool,
    #[serde(rename = "clearNvramOnRestore")]
    pub clear_nvram_on_restore: bool,
    #[serde(rename = "clearBackupOnRestore")]
    pub clear_backup_on_restore: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            nvram_path: PathBuf::from(r"c:\opt\nvram"),
            backup_path: PathBuf::from("."),
            clear_after_backup: false,
            clear_nvram_on_restore: true,
            clear_backup_on_restore: false,
        }
    }
}
