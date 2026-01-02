use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

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
}

pub trait AppStateUpdate {
    fn send_info(&self, msg: impl Into<String>) {
        self.send_state(AppState::info(msg));
    }

    fn send_error(&self, msg: impl Into<String>) {
        self.send_state(AppState::error(msg));
    }

    fn send_warn(&self, msg: impl Into<String>) {
        self.send_state(AppState::warn(msg));
    }

    fn send_state(&self, state: AppState);
}

impl AppStateUpdate for AppHandle {
    fn send_state(&self, state: AppState) {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StatusUpdate {
            level: String,
            message: String,
        }

        if let Err(e) = self.emit(
            "status_updated",
            StatusUpdate {
                level: state.level.as_str().to_string(),
                message: state.message,
            },
        ) {
            tracing::error!("failed update state: {e:?}")
        }
    }
}
