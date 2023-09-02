use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WebhookData {
		pub content: String,
		pub username: Option<String>,
}

