use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use crate::button::{Button, ButtonInfo, ButtonResponse};
use crate::html_utils::create_page_html;

mod html_utils;
pub mod button;

struct BtnifyState<S> {
    buttons_map: HashMap<String, Box<dyn (Fn(&S) -> ButtonResponse) + Send + Sync>>,
    user_state: S,
    page: Html<String>
}

/// Start your btnify server on the specified address with the specified buttons
///
/// Addr is the address to host the server
///
/// Buttons is a list of the buttons to put on the website
pub async fn bind_server<M: Send + Sync + 'static>(addr: SocketAddr, buttons: Vec<Button<M>>, user_state: M) {
    let page = Html(create_page_html(buttons.iter()));

    // todo: what if two buttons have the same id?
    let buttons_map = buttons.into_iter()
        .map(|b| (b.id, b.handler))
        .collect();

    let btnify_state = Arc::new(BtnifyState {
        buttons_map,
        user_state,
        page
    });

    let app = Router::new()
        .route("/", get(get_root).post(post_root))
        .with_state(btnify_state);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_root<S: Send + Sync>(State(state): State<Arc<BtnifyState<S>>>) -> Html<String> {
    // TODO: DONT USE CLONE
    state.page.clone()
}

async fn post_root<S: Send + Sync>(State(state): State<Arc<BtnifyState<S>>>, Json(info): Json<ButtonInfo>) -> Json<ButtonResponse> {
    let handler = state.buttons_map.get(&info.id);

    let res = match handler {
        Some(handler) => handler(&state.user_state),
        None => ButtonResponse::unknown_id()
    };

    Json(res)
}
