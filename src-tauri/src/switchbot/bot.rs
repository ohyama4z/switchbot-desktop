#[derive(Debug)]
pub struct Bot {
    device_id: String,
    pub name: String,
}

impl Bot {
    pub fn press(&self, token: &str, secret: &str) {
        let command_body: CommandBody = CommandBody {
            command: "press".to_string(),
            command_type: "command".to_string(),
            parameter: Parameter::Default,
        };
        send_command(token, secret, &self.device_id, command_body);
    }
}
