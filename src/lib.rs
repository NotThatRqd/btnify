//! # Examples
//!
//! Hello World
//!
//! ```
//! use btnify::button::{Button, ButtonResponse};
//!
//! fn greet_handler() -> ButtonResponse {
//!     ButtonResponse::from("hello world!")
//! }
//!
//! // this button doesn't use any state so we will mark the state generic as unit
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
//! use tokio::sync::oneshot;
//! use btnify::bind_server;
//! use btnify::ShutdownConfig;
//! use btnify::button::{Button, ButtonResponse};
//!
//! struct Counter {
//!     // must use Mutex for interior mutability
//!     count: Mutex<i32>,
//!     end_server_tx: Mutex<Option<oneshot::Sender<()>>>,
//! }
//!
//! impl Counter {
//!     fn new(tx: oneshot::Sender<()>) -> Counter {
//!         Counter {
//!             count: Mutex::new(0),
//!             end_server_tx: Mutex::new(Some(tx)),
//!         }
//!     }
//!
//!     fn end_server(&self) {
//!         // Acquire the Mutex to modify
//!         let mut tx = self.end_server_tx.lock().unwrap();
//!
//!         // Take the sender
//!         let tx = tx.take().unwrap();
//!
//!         // Send the signal to end the server
//!         tx.send(()).unwrap();
//!     }
//! }
//!
//! fn count_handler(state: &Counter) -> ButtonResponse {
//!     let count  = state.count.lock().unwrap();
//!     format!("The count is: {count}").into()
//! }
//!
//! fn plus_handler(state: &Counter, responses: Vec<Option<String>>) -> ButtonResponse {
//!     match &responses[0] {
//!         Some(response_str) => {
//!             if let Ok(amount) = response_str.parse::<i32>() {
//!                 let mut count = state.count.lock().unwrap();
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
//! fn end_button_handler(state: &Counter) -> ButtonResponse {
//!     state.end_server();
//!     "Server is ending. Goodbye!".into()
//! }
//!
//! fn server_end(state: &Counter) {
//!     println!("goodbye world. ;(");
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
//! let end_button = Button::create_button_with_state("End Server", Box::new(end_button_handler));
//!
//! let buttons = vec![count_button, plus_button, end_button];
//!
//! let (tx, rx) = oneshot::channel();
//!
//! let shutdown_config = ShutdownConfig::new(Some(rx), Box::new(server_end));
//!
//! bind_server(&"0.0.0.0:3000".parse().unwrap(), buttons, Counter::new(tx), None);
//! // uncomment to actually run the server:
//! //    .await
//! //    .unwrap();
//! ```

use crate::button::{Button, ButtonHandlerVariant, ButtonInfo, ButtonResponse};
use crate::html_utils::create_page_html;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;

pub mod button;
mod html_utils;

pub use tokio::sync::oneshot;

/// When the Btnify server is about to shut down the specified handler
/// will be called. The server will be triggered to shut down when
/// either ctrl+c is pressed or the `shutdown_rx` receiver is triggered.
/// I recommended that you store the sender in your server's state.
///
/// Also see: [tokio::sync::oneshot]
pub struct ShutdownConfig<S: Send + Sync + 'static> {
    pub shutdown_rx: Option<oneshot::Receiver<()>>,
    pub handler: Box<dyn FnOnce(&S)>,
}

impl<S: Send + Sync + 'static> ShutdownConfig<S> {
    pub fn new(
        shutdown_rx: Option<oneshot::Receiver<()>>,
        handler: Box<dyn FnOnce(&S)>,
    ) -> ShutdownConfig<S> {
        ShutdownConfig {
            shutdown_rx,
            handler,
        }
    }
}

/// Start your btnify server on the specified address with the specified [Button]s and [state],
/// along with the specified [ShutdownConfig]. If you don't need any custom state then use a unit (`()`)
///
/// [state]: Button::create_button_with_state
///
/// # Errors
///
/// Returns an error if there is a problem actually running the HTTP server, like if the address
/// is already being used by another application.
pub async fn bind_server<S: Send + Sync + 'static>(
    addr: &SocketAddr,
    buttons: Vec<Button<S>>,
    user_state: S,
    shutdown_config: Option<ShutdownConfig<S>>,
) -> hyper::Result<()> {
    let page = Html(create_page_html(buttons.iter()));

    let button_handlers = buttons.into_iter().map(|b| b.handler).collect();

    let btnify_state = Arc::new(BtnifyState {
        button_handlers,
        user_state,
        page,
    });

    let app = Router::new()
        .route("/", get(get_root).post(post_root))
        .with_state(Arc::clone(&btnify_state));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_handler(shutdown_config, btnify_state))
        .await
}

async fn shutdown_handler<S: Send + Sync + 'static>(
    config: Option<ShutdownConfig<S>>,
    state: Arc<BtnifyState<S>>,
) {
    if let Some(config) = config {
        if let Some(shutdown_rx) = config.shutdown_rx {
            tokio::select! {
                _ = ctrl_c_signal() => {},
                _ = shutdown_rx => {},
            }
            (config.handler)(&state.user_state);
            return;
        }
    }

    ctrl_c_signal().await;
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
            ButtonHandlerVariant::Basic(handler) => handler(),
            ButtonHandlerVariant::WithState(handler) => handler(&state.user_state),
            ButtonHandlerVariant::WithExtraPrompts(handler, extra_prompts) => {
                if info.extra_responses.len() == extra_prompts.len() {
                    handler(info.extra_responses)
                } else {
                    "Error parsing extra responses (extra responses length does not match extra prompts length)".into()
                }
            }
            ButtonHandlerVariant::WithBoth(handler, extra_prompts) => {
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
    button_handlers: Vec<ButtonHandlerVariant<S>>,
    user_state: S,
    page: Html<String>,
}

async fn ctrl_c_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("install ctrl+c signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("install terminate signal handler")
            .recv()
            .await;
    };

    // If not on unix, use a placeholder that will not ever resolve
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Wait for ctrl+c or terminate signal before this future completes
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
