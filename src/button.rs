use serde::{Deserialize, Serialize};

/// Represents a button you can put on your website :)
pub struct Button<S> {
    // todo: add "get_name" and "get_id" which return immutable str slice
    pub name: String,
    pub id: String,
    pub handler: Box<dyn Fn(&S) -> ButtonResponse>
}

impl<S> Button<S> {
    pub fn new(name: &str, handler: impl Fn(&S) -> ButtonResponse) -> Button<S> {
        Button {
            name: name.to_string(),
            id: name.to_lowercase(),
            handler: Box::new(handler)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ButtonInfo {
    pub id: String
    // todo: allow any extra data to be sent
}

#[derive(Serialize)]
pub struct ButtonResponse {
    pub message: String
}

impl ButtonResponse {
    pub(crate) fn unknown_id() -> ButtonResponse {
        ButtonResponse {
            message: "Unknown button id".to_string()
        }
    }
}

impl ButtonResponse {
    fn new(message: &str) -> ButtonResponse {
        ButtonResponse { message: message.to_string() }
    }
}
