#![doc=include_str!("../README.md")]

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use crate::button::{Button, ButtonInfo, ButtonResponse, ExtraResponse};
use crate::html_utils::create_page_html;

mod html_utils;
pub mod button;

/// Start your btnify server on the specified address with the specified [Button]s and [state].
/// If you don't need any custom state then use a unit (`()`)
///
/// [state]: button
///
/// # Errors
///
/// Returns an error if there is a problem actually running the HTTP server, like if the address
/// is already being used by another application.
pub async fn bind_server<S: Send + Sync + 'static>(addr: &SocketAddr, buttons: Vec<Button<S>>, user_state: S) -> hyper::Result<()> {
    let page = Html(create_page_html(buttons.iter()));

    let buttons_map = buttons
        .into_iter()
        .map(|b| b.handler)
        .collect();

    let btnify_state = Arc::new(BtnifyState {
        button_handlers: buttons_map,
        user_state,
        page
    });

    let app = Router::new()
        .route("/", get(get_root).post(post_root))
        .with_state(btnify_state);

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
}

async fn get_root<S: Send + Sync>(State(state): State<Arc<BtnifyState<S>>>) -> Html<String> {
    // TODO: DONT USE CLONE
    state.page.clone()
}

async fn post_root<S: Send + Sync>(State(state): State<Arc<BtnifyState<S>>>, Json(info): Json<ButtonInfo>) -> Json<ButtonResponse> {
    let handler = state.button_handlers.get(info.id);

    let res = match handler {
        Some(handler) => handler(&state.user_state, info.extra_responses),
        None => "Unknown button id".into()
    };

    Json(res)
}

struct BtnifyState<S> {
    button_handlers: Vec<Box<dyn (Fn(&S, Option<Vec<ExtraResponse>>) -> ButtonResponse) + Send + Sync>>,
    user_state: S,
    page: Html<String>
}
