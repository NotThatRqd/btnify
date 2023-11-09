//! # Examples
//!
//! Hello World
//!
//! ```
//! use btnify::button::{Button, ButtonResponse, ExtraResponse};
//!
//! fn greet_handler() -> ButtonResponse {
//!     ButtonResponse::from("hello world!")
//! }
//!
//! // No extra prompts for this button
//! let greet_button: Button<()> = Button::create_basic_button("Greet!", Box::new(greet_handler));
//! ```
//!
//! Counter
//!
//! ```
//! use std::sync::Mutex;
//! use btnify::button::{Button, ButtonResponse, ExtraResponse};
//!
//! struct Counter {
//!     count: Mutex<i32>
//! }
//!
//! fn count_handler(state: &Counter) -> ButtonResponse {
//!     let mut count  = state.count.lock().unwrap();
//!     *count += 1;
//!     format!("The count is now: {count}").into()
//! }
//!
//! // Also no extra prompts
//! let count_button = Button::create_button_with_state("Counter", Box::new(count_handler));
//! ```

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use crate::button::{Button, ButtonHandler, ButtonInfo, ButtonResponse};
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
        Some(handler) => match handler {
            ButtonHandler::Basic(handler) => handler(),
            ButtonHandler::WithState(handler) => handler(&state.user_state),
            ButtonHandler::WithExtraPrompts(handler, extra_prompts) => {
                if info.extra_responses.len() == extra_prompts.len() {
                    handler(info.extra_responses)
                } else {
                    "Error parsing extra responses (extra responses length does not match extra prompts length)".into()
                }
            }
            ButtonHandler::WithBoth(handler, extra_prompts) => {
                if info.extra_responses.len() == extra_prompts.len() {
                    handler(&state.user_state, info.extra_responses)
                } else {
                    "Error parsing extra responses (extra responses length does not match extra prompts length)".into()
                }
            }
        },
        None => "Unknown button id".into()
    };

    Json(res)
}

struct BtnifyState<S: Send + Sync + 'static> {
    button_handlers: Vec<ButtonHandler<S>>,
    user_state: S,
    page: Html<String>
}
