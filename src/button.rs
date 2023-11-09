//! # Button Related Structs
//!
//! This module contains structs related to buttons and explains how buttons work in general using
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

use serde::{Deserialize, Serialize};

/// When a user is asked for an [extra response], there is the option to click "cancel" on the
/// prompt which will result in a `None` variant.
///
/// [extra response]: crate::button
pub type ExtraResponse = Option<String>;

pub(crate) enum ButtonHandler<S: Send + Sync + 'static> {
    Basic(Box<dyn Send + Sync + Fn() -> ButtonResponse>),
    WithState(Box<dyn Send + Sync + Fn(&S) -> ButtonResponse>),
    WithExtraPrompts(Box<dyn Send + Sync + Fn(Vec<ExtraResponse>) -> ButtonResponse>, Vec<String>),
    WithBoth(Box<dyn Send + Sync + Fn(&S, Vec<ExtraResponse>) -> ButtonResponse>, Vec<String>)
}

/// Represents a button you can put on your btnify server.
pub struct Button<S: Send + Sync + 'static> {
    pub(crate) name: String,

    // TODO: rename "handler"
    pub(crate) handler: ButtonHandler<S>,
}

impl<S: Send + Sync + 'static> Button<S> {
    fn new(name: &str, handler: ButtonHandler<S>) -> Button<S> {
        Button {
            name: name.to_string(),
            handler
        }
    }

    pub fn create_basic_button(name: &str, handler: Box<dyn Send + Sync + Fn() -> ButtonResponse>) -> Button<S> {
        Button::new(name, ButtonHandler::Basic(handler))
    }

    pub fn create_button_with_state(name: &str, handler: Box<dyn Send + Sync + Fn(&S) -> ButtonResponse>) -> Button<S> {
        Button::new(name, ButtonHandler::WithState(handler))
    }

    pub fn create_button_with_prompts(name: &str, handler: Box<dyn Send + Sync + Fn(Vec<ExtraResponse>) -> ButtonResponse>, extra_prompts: Vec<String>) -> Button<S> {
        Button::new(name, ButtonHandler::WithExtraPrompts(handler, extra_prompts))
    }

    pub fn create_button_with_state_and_prompts(name: &str, handler: Box<dyn Send + Sync + Fn(&S, Vec<ExtraResponse>) -> ButtonResponse>, extra_prompts: Vec<String>) -> Button<S> {
        Button::new(name, ButtonHandler::WithBoth(handler, extra_prompts))
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ButtonInfo {
    pub id: usize,
    pub extra_responses: Vec<ExtraResponse>
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
