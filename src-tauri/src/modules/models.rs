use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
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
    pub backup_time: DateTime<Local>,
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
    #[serde(rename = "psPath")]
    pub ps_path: PathBuf,
    #[serde(rename = "jchipPath")]
    pub jchip_path: PathBuf,
    #[serde(rename = "backupPath")]
    pub backup_path: PathBuf,
    #[serde(rename = "clearAfterBackup")]
    pub clear_after_backup: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ps_path: PathBuf::from(r"c:\opt"),
            jchip_path: PathBuf::from(r"c:\jchip"),
            backup_path: PathBuf::from("."),
            clear_after_backup: false,
        }
    }
}

impl AppConfig {
    // 从exe路径加载配置文件，如果失败则返回默认值
    pub fn load_from_exe_path() -> Self {
        // 获取当前可执行文件的路径
        if let Ok(exe_path) = std::env::current_exe() {
            // 获取exe所在目录
            if let Some(exe_dir) = exe_path.parent() {
                // 构造配置文件路径
                let config_path = exe_dir.join("nvram-manager.json");

                // 尝试打开配置文件
                if let Ok(mut file) = File::open(config_path) {
                    // 读取文件内容
                    let mut content = String::new();
                    if let Ok(_) = file.read_to_string(&mut content) {
                        // 尝试解析JSON
                        if let Ok(config) = serde_json::from_str(&content) {
                            return config;
                        }
                    }
                }
            }
        }

        // 如果任何步骤失败，返回默认值
        Self::default()
    }
}
