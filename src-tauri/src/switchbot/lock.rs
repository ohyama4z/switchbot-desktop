use crate::switchbot::api::{send_command, CommandBody, Parameter};
use crate::switchbot::command::{CommandFunctionParameter, CommandFunctionReturn};

pub fn lock(
    CommandFunctionParameter {
        token,
        secret,
        device_id,
        option: _,
    }: CommandFunctionParameter,
) -> CommandFunctionReturn {
    let command_body: CommandBody = CommandBody {
        command: "lock".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };

    Box::pin(async move {
        send_command(&token, &secret, &device_id, command_body).await?;
        Ok(())
    })
}

pub fn unlock(
    CommandFunctionParameter {
        token,
        secret,
        device_id,
        option: _,
    }: CommandFunctionParameter,
) -> CommandFunctionReturn {
    let command_body: CommandBody = CommandBody {
        command: "unlock".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };

    Box::pin(async move {
        send_command(&token, &secret, &device_id, command_body).await?;
        Ok(())
    })
}
