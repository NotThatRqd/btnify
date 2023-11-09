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
//! let greet_button: Button<()> = Button::create_basic_button("Greet!", Box::new(greet_handler));
//! ```
//!
//! Hello World 2.0
//!
//! ```
//! use btnify::button::{Button, ButtonResponse};
//!
//! fn better_greet_handler(responses: Vec<Option<String>>) -> ButtonResponse {
//!     // responses is guaranteed to be the same length as the number of extra prompts
//!     // specified when creating a button
//!     let name = &responses[0];
//!     match name {
//!         Some(name) => format!("Hello, {name}").into(),
//!         None => format!("You didn't provide a name! :(").into()
//!     }
//! }
//!
//! let better_greet_button: Button<()> = Button::create_button_with_prompts(
//!     "Greet 2.0",
//!     Box::new(better_greet_handler),
//!     vec!["What's your name?".to_string()]
//! );
//! ```
//!
//! Counter App
//!
//! ```
//! use std::sync::Mutex;
//! use btnify::bind_server;
//! use btnify::button::{Button, ButtonResponse, ExtraResponse};
//!
//! struct Counter {
//!     // must use mutex to be thread-safe
//!     count: Mutex<i32>
//! }
//!
//! impl Counter {
//!     fn new() -> Counter {
//!         Counter {
//!             count: Mutex::new(0)
//!         }
//!     }
//! }
//!
//! fn count_handler(state: &Counter) -> ButtonResponse {
//!     let count  = state.count.lock().unwrap();
//!     format!("The count is: {count}").into()
//! }
//!
//! fn plus_handler(counter_struct: &Counter, responses: Vec<Option<String>>) -> ButtonResponse {
//!     match &responses[0] {
//!         Some(response_str) => {
//!             if let Ok(amount) = response_str.parse::<i32>() {
//!                 let mut count = counter_struct.count.lock().unwrap();
//!                 *count += amount;
//!                 format!("The count now is: {}", *count).into()
//!             } else {
//!                 "You did not provide a number.".into()
//!             }
//!         }
//!         None => "You didn't provide any input.".into(),
//!     }
//! }
//!
//! let count_button = Button::create_button_with_state("Counter", Box::new(count_handler));
//!
//! let plus_button = Button::create_button_with_state_and_prompts(
//!     "add to counter",
//!     Box::new(plus_handler),
//!     vec!["How much do you want to add?".to_string()]
//! );
//!
//! let buttons = [count_button, plus_button];
//!
//! // uncomment to run server on localhost:3000
//! // bind_server(&"0.0.0.0:3000".parse().unwrap(), buttons, Counter::new())
//! //    .await
//! //    .unwrap();
//! ```

use crate::button::{Button, ButtonHandler, ButtonInfo, ButtonResponse};
use crate::html_utils::create_page_html;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};
use std::net::SocketAddr;
use std::sync::Arc;

pub mod button;
mod html_utils;

/// Start your btnify server on the specified address with the specified [Button]s and [state].
/// If you don't need any custom state then use a unit (`()`)
///
/// [state]: button
///
/// # Errors
///
/// Returns an error if there is a problem actually running the HTTP server, like if the address
/// is already being used by another application.
pub async fn bind_server<S: Send + Sync + 'static>(
    addr: &SocketAddr,
    buttons: Vec<Button<S>>,
    user_state: S,
) -> hyper::Result<()> {
    let page = Html(create_page_html(buttons.iter()));

    let buttons_map = buttons.into_iter().map(|b| b.handler).collect();

    let btnify_state = Arc::new(BtnifyState {
        button_handlers: buttons_map,
        user_state,
        page,
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

async fn post_root<S: Send + Sync>(
    State(state): State<Arc<BtnifyState<S>>>,
    Json(info): Json<ButtonInfo>,
) -> Json<ButtonResponse> {
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
        None => "Unknown button id".into(),
    };

    Json(res)
}

struct BtnifyState<S: Send + Sync + 'static> {
    button_handlers: Vec<ButtonHandler<S>>,
    user_state: S,
    page: Html<String>,
}
