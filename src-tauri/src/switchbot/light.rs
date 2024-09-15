use crate::switchbot::api::{send_command, CommandBody, Parameter};

#[derive(Debug)]
pub struct Light {
    device_id: String,
    pub name: String,
}

impl Light {
    pub async fn turnOn(
        &self,
        token: &str,
        secret: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command_body: CommandBody = CommandBody {
            command: "turnOn".to_string(),
            command_type: "command".to_string(),
            parameter: Parameter::Default,
        };

        send_command(token, secret, &self.device_id, command_body).await?;
        Ok(())
    }

    pub async fn turnOff(
        &self,
        token: &str,
        secret: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command_body: CommandBody = CommandBody {
            command: "turnOff".to_string(),
            command_type: "command".to_string(),
            parameter: Parameter::Default,
        };

        send_command(token, secret, &self.device_id, command_body).await?;
        Ok(())
    }
}
