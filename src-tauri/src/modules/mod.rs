mod backup;
mod commands;
mod models;
mod state;
mod utils;

use std::path::Path;

pub use backup::*;
pub use models::*;
pub use utils::*;
pub use commands::*;

use crate::modules::state::AppState;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

pub trait AppHandleExt {
    fn update_state_info(&self, msg: impl Into<String>) {
        self.update_state(AppState::info(msg));
    }

    fn update_state_error(&self, msg: impl Into<String>) {
        self.update_state(AppState::error(msg));
    }

    fn update_state_warn(&self, msg: impl Into<String>) {
        self.update_state(AppState::warn(msg));
    }

    fn update_state(&self, state: AppState);

    async fn try_load_config(&self, path: &Path) -> Option<AppConfig>;
}

impl AppHandleExt for AppHandle {
    fn update_state(&self, state: AppState) {
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

    async fn try_load_config(&self, path: &Path) -> Option<AppConfig> {
        todo!()
    }
}
