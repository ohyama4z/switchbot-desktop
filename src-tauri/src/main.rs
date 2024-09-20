// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod switchbot;

use confy;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiKey {
    token: String,
    secret: String,
}
fn get_api_key() -> Result<ApiKey, ConfyError> {
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
    AirConditioner,
    Fan,
    Plug,
    Hub,
}
impl FromStr for SwitchBotDeviceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Light" => Ok(SwitchBotDeviceType::Light),
            "Smart Lock" => Ok(SwitchBotDeviceType::Lock),
            "Bot" => Ok(SwitchBotDeviceType::Bot),
            "Air Conditioner" => Ok(SwitchBotDeviceType::AirConditioner),
            "Fan" => Ok(SwitchBotDeviceType::Fan),
            "Plug Mini (JP)" => Ok(SwitchBotDeviceType::Plug),
            "Hub 2" => Ok(SwitchBotDeviceType::Hub),
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

// #[tauri::command(rename_all = "snake_case")]
// fn bot_press(device_id: String) -> Result<(), String> {
//     let api_key = get_api_key().map_err(|e| e.to_string())?;
//     tauri::async_runtime::block_on(switchbot::bot::press(
//         &device_id,
//         &api_key.token,
//         &api_key.secret,
//     ))
//     .map_err(|e| e.to_string())?;

//     Ok(())
// }

#[tauri::command(rename_all = "snake_case")]
fn light_turn_on(device_id: String) -> Result<(), String> {
    let api_key = get_api_key().map_err(|e| e.to_string())?;
    tauri::async_runtime::block_on(switchbot::light::turn_on(
        &device_id,
        &api_key.token,
        &api_key.secret,
    ))
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn light_turn_off(device_id: String) -> Result<(), String> {
    let api_key = get_api_key().map_err(|e| e.to_string())?;
    tauri::async_runtime::block_on(switchbot::light::turn_off(
        &device_id,
        &api_key.token,
        &api_key.secret,
    ))
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn lock_lock(device_id: String) -> Result<(), String> {
    let api_key = get_api_key().map_err(|e| e.to_string())?;
    tauri::async_runtime::block_on(switchbot::lock::lock(
        &device_id,
        &api_key.token,
        &api_key.secret,
    ))
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_api_key,
            get_devices,
            bot_press,
            light_turn_on,
            light_turn_off,
            lock_lock
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
