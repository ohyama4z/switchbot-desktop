use api::get_api_key;

#[derive(Debug)]
enum CommandOption {
    AirConditioner,
    Fan,
}

#[derive(Debug)]
pub(crate) struct CommandFunctionParameter<'a> {
    pub(crate) device_id: &'a str,
    pub(crate) token: &'a str,
    pub(crate) secret: &'a str,
    pub(crate) option: Option<CommandOption>,
}

pub(crate) type CommandFunction =
    async fn (CommandFunctionParameter) -> Result<(), Box<dyn std::error::Error>>;

enum CommandName {}

pub(crate) async fn excuse_command(
    device_id: String,
    command_function: CommandFunction,
    option: Option<CommandOption>,
) -> Result<(), String> {
    let api_key = get_api_key().map_err(|e| e.to_string())?;
    tauri::async_runtime::block_on(command_function(CommandFunctionParameter {
        device_id: &device_id,
        token: &api_key.token,
        secret: &api_key.secret,
        option,
    }))
    .map_err(|e| e.to_string())?;

    Ok(())
}
