// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod switchbot;

use confy;
use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiKey {
    pub(crate) token: String,
    pub(crate) secret: String,
}

pub(crate) fn get_api_key() -> Result<ApiKey, ConfyError> {
    return confy::load("switchbot-desktop", "switchbot-desktop");
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn save_api_key(api_key: ApiKey) -> Result<(), String> {
    let err = confy::store("switchbot-desktop", "switchbot-desktop", api_key);
    match err {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_api_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
