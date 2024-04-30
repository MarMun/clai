use crate::{
    actions::user::{tell, UserMessage},
    AlertLevel,
};

pub fn show(command: &str) {
    tell(UserMessage {
        level: AlertLevel::Neutral,
        body: format!("Would explain {command}"),
    });
}
