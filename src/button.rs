//! # Button Related Structs
//!
//! This module contains structs related to buttons and explains how b buttons work in general using
//! Btnify. In case my explanations below don't make any sense please check the examples at the
//! bottom of the page :)
//!
//! # Handlers
//!
//! Every [Button] has a handler, which is a function/closure that takes a reference to a global
//! state (represented with the generic type parameter `S`) that all buttons in a Btnify server
//! will be given. If you don't need any state then use a unit (`()`). Handlers are also given an
//! optional vec of [ExtraResponse]s, which are the responses to any custom questions/prompts of
//! the button. Handlers return a [ButtonResponse] which just holds a String that will be shown
//! to the user. [ButtonResponse] implements `From<&str>` and `From<String>` for convince.
//!
//! # State System
//!
//! Btnify allows you to save state between button presses and buttons themselves using its state
//! system. Because handlers are given an immutable reference to the state, you will need to use
//! interior mutability. States must implement `Send` and `Sync`, so you will need a `Mutex`
//! instead of a `RefCell`.
//!
//! # Extra Prompt and Response System
//!
//! Btnify allows you to ask the user for any extra data when they click your button using the
//! extra prompt/response system. When a user presses a button, its extra prompts will be given
//! to the user and their response will be given to the button's handler.
//!
//! # Examples
//!
//! Hello World
//!
//! ```
//! use btnify::button::{Button, ButtonResponse, ExtraResponse};
//!
//! fn greet_handler(_: &(), _:Option<Vec<ExtraResponse>>) -> ButtonResponse {
//!     ButtonResponse::from("hello world!")
//! }
//!
//! // No extra prompts for this button
//! let greet_button = Button::new("Greet!", greet_handler, None);
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
//! fn count_handler(state: &Counter, _:Option<Vec<ExtraResponse>>) -> ButtonResponse {
//!     let mut count  = state.count.lock().unwrap();
//!     *count += 1;
//!     format!("The count is now: {count}").into()
//! }
//!
//! // Also no extra prompts
//! let count_button = Button::new("Counter", count_handler, None);
//! ```
//!

use serde::{Deserialize, Serialize};

/// When a user is asked for an [extra response], there is the option to click "cancel" on the
/// prompt which will result in a `None` variant.
///
/// [extra response]: crate::button
pub type ExtraResponse = Option<String>;

/// Represents a button you can put on your btnify server.
///
/// `Name` is the text that will be on the button.
///
/// Check [here] for explanations for `handler` and `extra_prompts` along with examples
///
/// [here]: crate::button
pub struct Button<S: Send + Sync + 'static> {
    pub name: String,
    pub handler: Box<dyn (Fn(&S, Option<Vec<ExtraResponse>>) -> ButtonResponse) + Send + Sync>,
    pub extra_prompts: Option<Vec<String>>
}

impl<S: Send + Sync + 'static> Button<S> {
    /// Creates a new [Button] struct.
    ///
    /// Check [Button]'s documentation for explanations of the fields.
    pub fn new<T: Send + Sync + Fn(&S, Option<Vec<ExtraResponse>>) -> ButtonResponse + 'static>(name: &str, handler: T, extra_prompts: Option<Vec<String>>) -> Button<S> {
        Button {
            name: name.to_string(),
            handler: Box::new(handler),
            extra_prompts
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ButtonInfo {
    pub id: usize,
    pub extra_responses: Option<Vec<ExtraResponse>>
}

/// Represents the server's response to a [Button] being pressed. Currently only has a message field.
#[derive(Serialize)]
pub struct ButtonResponse {
    pub message: String
}

impl From<&str> for ButtonResponse {
    fn from(message: &str) -> Self {
        ButtonResponse { message: message.to_string() }
    }
}

impl From<String> for ButtonResponse {
    fn from(message: String) -> Self {
        ButtonResponse { message }
    }
}
