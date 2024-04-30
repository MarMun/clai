use crate::actions::user::{tell, MessageType, UserMessage};

pub fn put(command: &str) {
    tell(UserMessage {
        message_type: MessageType::Neutral,
        body: format!("Would clipboard {command}"),
    });
}
