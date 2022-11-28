pub const NOTIFICATION_EVENT_NAME: &str = "notification";

#[derive(Clone, serde::Serialize)]
pub struct NotificationPayload {
    pub title: String,
    pub body: Option<String>,
    pub icon: Option<String>,
}
