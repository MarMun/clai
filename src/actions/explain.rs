use crate::actions::user::{tell, MessageType, UserMessage};

pub fn show(command: &str) {
    tell(UserMessage {
        message_type: MessageType::Neutral,
        body: format!("Would explain {command}"),
    });
}
