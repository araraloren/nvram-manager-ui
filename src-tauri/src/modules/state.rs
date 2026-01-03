use serde::{Deserialize, Serialize};

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
    pub level: Level,
    pub message: String,
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
}
