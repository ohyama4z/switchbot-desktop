use crate::switchbot::api::{send_command, CommandBody, Parameter};
use crate::switchbot::command::CommandFunctionParameter;

pub async fn press<'a>(
    CommandFunctionParameter {
        token,
        secret,
        device_id,
        option: _,
    }: CommandFunctionParameter<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_body: CommandBody = CommandBody {
        command: "press".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };

    send_command(token, secret, device_id, command_body).await?;
    Ok(())
}
