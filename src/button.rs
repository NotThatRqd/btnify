use axum::handler::Handler;
use axum::Json;
use serde::{Deserialize, Serialize};

pub struct Button<H, T, S>
where
    H: Handler<T, S, Json<ButtonInfo>>,
    T: 'static,
    S: Clone + Send + Sync + 'static
{
    pub(crate) name: String,
    pub(crate) id: String,
    handler: H
}

impl<H, T, S> Button<H, T, S>
where
    H: Handler<T, S, Json<ButtonInfo>>,
    T: 'static,
    S: Clone + Send + Sync + 'static
{
    pub fn new(name: &str, handler: H) -> Button<H, T, S> {
        Button {
            name: name.to_string(),
            id: name.to_lowercase(),
            handler
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
    fn new(message: &str) -> ButtonResponse {
        ButtonResponse { message: message.to_string() }
    }
}
