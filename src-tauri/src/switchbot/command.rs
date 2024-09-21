use serde::{Deserialize, Serialize};

use super::super::get_api_key;
use std::{future::Future, pin::Pin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum CommandOption {
    AirConditioner,
    Fan,
}

#[derive(Debug)]
pub(crate) struct CommandFunctionParameter {
    pub(crate) device_id: String,
    pub(crate) token: String,
    pub(crate) secret: String,
    pub(crate) option: Option<CommandOption>,
}

pub(crate) type CommandFunctionReturn =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send>>;

pub(crate) type CommandFunction = fn(CommandFunctionParameter) -> CommandFunctionReturn;

pub(crate) async fn excuse_command(
    device_id: String,
    command_function: CommandFunction,
    option: Option<CommandOption>,
) -> Result<(), String> {
    let api_key = get_api_key().map_err(|e| e.to_string())?;
    tauri::async_runtime::block_on(command_function(CommandFunctionParameter {
        device_id: device_id,
        token: api_key.token,
        secret: api_key.secret,
        option,
    }))
    .map_err(|e| e.to_string())?;

    Ok(())
}
