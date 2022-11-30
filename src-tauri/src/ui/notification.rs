use serde_json::Result;

pub const NOTIFICATION_EVENT_NAME: &str = "notification";

#[derive(Clone, serde::Serialize)]
pub struct NotificationPayload {
    pub title: String,
    pub body: Option<String>,
    pub icon: Option<String>,
}

impl NotificationPayload {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}
