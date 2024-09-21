use crate::switchbot::api::{send_command, CommandBody, Parameter};
use crate::switchbot::command::{CommandFunctionParameter, CommandFunctionReturn};

pub fn turn_on(
    CommandFunctionParameter {
        token,
        secret,
        device_id,
        option: _,
    }: CommandFunctionParameter,
) -> CommandFunctionReturn {
    let command_body: CommandBody = CommandBody {
        command: "turnOn".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };

    Box::pin(async move {
        send_command(&token, &secret, &device_id, command_body).await?;
        Ok(())
    })
}

pub fn turn_off(
    CommandFunctionParameter {
        token,
        secret,
        device_id,
        option: _,
    }: CommandFunctionParameter,
) -> CommandFunctionReturn {
    let command_body: CommandBody = CommandBody {
        command: "turnOff".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };

    Box::pin(async move {
        send_command(&token, &secret, &device_id, command_body).await?;
        Ok(())
    })
}
