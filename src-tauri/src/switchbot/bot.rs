use crate::switchbot::api::{send_command, CommandBody, Parameter};

#[derive(Debug)]
pub struct Bot {
    pub name: String,
}

impl Bot {
    pub async fn press(
        &self,
        device_id: &str,
        token: &str,
        secret: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command_body: CommandBody = CommandBody {
            command: "press".to_string(),
            command_type: "command".to_string(),
            parameter: Parameter::Default,
        };

        send_command(token, secret, device_id, command_body).await?;
        Ok(())
    }
}
