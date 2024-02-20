use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum ActionEnum {
    Establish,
    Ping {
        data: Option<usize>,
    },
    MessageSend {
        content: String,
        reply: Option<String>,
        channel: String,
    },
    MessageEdit {
        message: String,
        channel: String,
    },
    MessageDelete {
        message: String,
        channel: String,
    },
    TypeStatus {
        typing: bool,
        channel: String,
    },
}
