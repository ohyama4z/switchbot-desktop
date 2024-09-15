use crate::switchbot::api::{send_command, CommandBody, Parameter};

#[derive(Debug)]
pub struct Bot {
    device_id: String,
    pub name: String,
}

impl Bot {
    pub async fn press(&self, token: &str, secret: &str) -> Result<(), Box<dyn std::error::Error>> {
        let command_body: CommandBody = CommandBody {
            command: "press".to_string(),
            command_type: "command".to_string(),
            parameter: Parameter::Default,
        };

        send_command(token, secret, &self.device_id, command_body).await?;
        Ok(())
    }
}
