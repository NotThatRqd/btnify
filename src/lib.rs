use std::net::SocketAddr;
use axum::{Json, Router};
use axum::handler::Handler;
use axum::routing::get;
use crate::button::{Button, ButtonInfo, ButtonResponse};

mod html_utils;
mod button;

pub struct BtnifyServer<H, T, S>
where
    H: Handler<T, S, Json<ButtonInfo>>,
    T: 'static,
    S: Clone + Send + Sync + 'static
{
    buttons: Vec<Button<H, T, S>>
}

impl<H, T, S> BtnifyServer<H, T, S>
where
    H: Handler<T, S, Json<ButtonInfo>>,
    T: 'static,
    S: Clone + Send + Sync + 'static
{
    pub fn new() -> BtnifyServer<H, T, S> {
        BtnifyServer { buttons: vec![] }
    }

    pub async fn bind(&self, addr: &SocketAddr) {
        let app = Router::new()
            .route("/", get(get_root).post(post_root));

        axum::Server::bind(addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

async fn get_root() -> &'static str {
    "hello"
}

async fn post_root() -> Json<ButtonResponse> {
    Json(ButtonResponse {
        message: "hello".to_string()
    })
}

