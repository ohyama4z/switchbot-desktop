use crate::switchbot::api::{send_command, CommandBody, Parameter};
use crate::switchbot::command::{CommandFunctionParameter, CommandFunctionReturn};

pub fn toggle_power(
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

pub fn change_speed(
    CommandFunctionParameter {
        token,
        secret,
        device_id,
        option: _,
    }: CommandFunctionParameter,
) -> CommandFunctionReturn {
    let command_body: CommandBody = CommandBody {
        command: "lowSpeed".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };
    Box::pin(async move {
        send_command(&token, &secret, &device_id, command_body).await?;
        Ok(())
    })
}

pub fn toggle_swing(
    CommandFunctionParameter {
        token,
        secret,
        device_id,
        option: _,
    }: CommandFunctionParameter,
) -> CommandFunctionReturn {
    let command_body: CommandBody = CommandBody {
        command: "swing".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };
    Box::pin(async move {
        send_command(&token, &secret, &device_id, command_body).await?;
        Ok(())
    })
}
