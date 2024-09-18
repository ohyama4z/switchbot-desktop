use crate::switchbot::api::{send_command, CommandBody, Parameter};

#[derive(Debug)]
pub struct Lock {
    pub name: String,
}

impl Lock {
    pub async fn lock(
        &self,
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
        &self,
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
}
