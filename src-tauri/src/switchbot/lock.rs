#[derive(Debug)]
pub struct Lock {
    device_id: String,
    pub name: String,
}

impl Lock {
    pub fn lock(&self, token: &str, secret: &str) {
        let command_body: CommandBody = CommandBody {
            command: "lock".to_string(),
            command_type: "command".to_string(),
            parameter: Parameter::Default,
        };
        send_command(token, secret, &self.device_id, command_body);
    }

    pub fn unlock(&self, token: &str, secret: &str) {
        let command_body: CommandBody = CommandBody {
            command: "unlock".to_string(),
            command_type: "command".to_string(),
            parameter: Parameter::Default,
        };
        send_command(token, secret, &self.device_id, command_body);
    }
}
