use crate::switchbot::api::{send_command, CommandBody, Parameter};

pub async fn lock(
    device_id: &str,
    token: &str,
    secret: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_body: CommandBody = CommandBody {
        command: "lock".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };

    send_command(token, secret, device_id, command_body).await?;
    Ok(())
}

pub async fn unlock(
    device_id: &str,
    token: &str,
    secret: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_body: CommandBody = CommandBody {
        command: "unlock".to_string(),
        command_type: "command".to_string(),
        parameter: Parameter::Default,
    };
    send_command(token, secret, device_id, command_body).await?;
    Ok(())
}
