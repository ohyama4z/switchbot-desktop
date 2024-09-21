// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod switchbot;

use confy;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use switchbot::{
    bot::press,
    command::{excuse_command, CommandFunction},
    light::{turn_off, turn_on},
    lock::{self, lock, unlock},
};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub token: String,
    pub secret: String,
}
pub fn get_api_key() -> Result<ApiKey, ConfyError> {
    return confy::load("switchbot-desktop", "switchbot-desktop");
}

#[tauri::command(rename_all = "snake_case")]
fn save_api_key(api_key: ApiKey) -> Result<(), String> {
    let err = confy::store("switchbot-desktop", "switchbot-desktop", api_key);
    match err {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum SwitchBotDeviceType {
    Light,
    Lock,
    Bot,
    // AirConditioner,
    // Fan,
    // Plug,
    // Hub,
}
impl FromStr for SwitchBotDeviceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Light" => Ok(SwitchBotDeviceType::Light),
            "Smart Lock" => Ok(SwitchBotDeviceType::Lock),
            "Bot" => Ok(SwitchBotDeviceType::Bot),
            // "Air Conditioner" => Ok(SwitchBotDeviceType::AirConditioner),
            // "Fan" => Ok(SwitchBotDeviceType::Fan),
            // "Plug Mini (JP)" => Ok(SwitchBotDeviceType::Plug),
            // "Hub 2" => Ok(SwitchBotDeviceType::Hub),
            _ => Err(format!("Unknown SwitchBotDeviceType: {}", s)),
        }
    }
}

// ユーザ視点から見た各機器をSwitchBotデバイスと呼称
#[derive(Debug, Deserialize, Serialize)]
struct SwitchBotDevice {
    device_id: String,
    device_name: String,
    device_type: SwitchBotDeviceType,
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn get_devices() -> Result<Vec<SwitchBotDevice>, String> {
    let api_key = get_api_key().map_err(|e| e.to_string())?;

    let devices = switchbot::api::get_devices(&api_key.token, &api_key.secret)
        .await
        .map_err(|e| e.to_string())?;

    let mut switchbot_devices = vec![];
    for device in devices.device_list {
        let device_type = SwitchBotDeviceType::from_str(&device.device_type)?;
        let switchbot_device = SwitchBotDevice {
            device_id: device.device_id,
            device_name: device.device_name,
            device_type: device_type,
        };
        switchbot_devices.push(switchbot_device);
    }

    for device in devices.infrared_remote_list {
        let device_type = SwitchBotDeviceType::from_str(&device.remote_type)?;
        let switchbot_device = SwitchBotDevice {
            device_id: device.device_id,
            device_name: device.device_name,
            device_type: device_type,
        };
        switchbot_devices.push(switchbot_device);
    }

    Ok(switchbot_devices)
}

#[tauri::command]
pub(crate) async fn excuse(
    device_id: String,
    switch_bot_device: SwitchBotDeviceType,
    command_name: String,
    option: Option<switchbot::command::CommandOption>,
) -> Result<(), String> {
    let mut result: Result<(), String> = Err("Unknown command".to_string());
    match switch_bot_device {
        SwitchBotDeviceType::Bot => {
            if command_name.as_str() == "press" {
                result = excuse_command(device_id, press, option).await;
            }
        }
        SwitchBotDeviceType::Light => {
            if command_name.as_str() == "turn_on" {
                result = excuse_command(device_id, turn_on, option).await;
            } else if command_name.as_str() == "turn_off" {
                result = excuse_command(device_id, turn_off, option).await;
            }
        }
        SwitchBotDeviceType::Lock => {
            if command_name.as_str() == "lock" {
                result = excuse_command(device_id, lock, option).await;
            } else if command_name.as_str() == "unlock" {
                result = excuse_command(device_id, unlock, option).await;
            }
        }
    }

    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_api_key, get_devices, excuse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
