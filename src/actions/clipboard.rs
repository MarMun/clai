use crate::{
    actions::user::{tell, UserMessage},
    AlertLevel,
};

pub fn put(command: &str) {
    tell(UserMessage {
        level: AlertLevel::Neutral,
        body: format!("Would clipboard {command}"),
    });
}
