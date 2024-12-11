use crate::data::{Notification, SenderType};

#[macro_export]
macro_rules! push_notification {
	($title:expr, $message:expr, $sender:expr, $sender_type:expr) => {
		let notification = Notification{
			id: 0,
			title: $title.to_string(),
			message: $message.to_string(),
			read: false,
			archived: false,
			action: vec![],
			sender: $sender.to_string(),
			sender_type: $sender_type,
			date: "".to_string(),
		}
	};
}

fn push(title: String, message: String, sender: String, sender_type: SenderType) -> Notification {
    Notification {
        id: "".to_string(),
        title: title,
        message: message,
        read: false,
        archived: false,
        action: vec![],
        sender: sender,
        receiver: "".to_string(),
        sender_type: sender_type,
        date: "".to_string(),
    }
}
