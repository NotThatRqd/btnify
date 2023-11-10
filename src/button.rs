use serde::{Deserialize, Serialize};

/// Represents a button you can put on your btnify server.
pub struct Button<S: Send + Sync + 'static> {
    pub(crate) name: String,
    pub(crate) handler: ButtonHandlerVariant<S>,
}

impl<S: Send + Sync + 'static> Button<S> {
    fn new(name: &str, handler: ButtonHandlerVariant<S>) -> Button<S> {
        Button {
            name: name.to_string(),
            handler,
        }
    }

    /// Creates a Button with the specified name and handler.
    pub fn create_basic_button(
        name: &str,
        handler: Box<dyn Send + Sync + Fn() -> ButtonResponse>,
    ) -> Button<S> {
        Button::new(name, ButtonHandlerVariant::Basic(handler))
    }

    /// Creates a Button whose handler will be given an immutable reference to a user-defined
    /// state that is initialized when calling [bind_server]. Almost every struct/function in
    /// Btnify has the generic type parameter `S` which represents the user-defined state.
    /// You can see an example [here](crate#examples).
    ///
    /// [bind_server]: crate::bind_server
    pub fn create_button_with_state(
        name: &str,
        handler: Box<dyn Send + Sync + Fn(&S) -> ButtonResponse>,
    ) -> Button<S> {
        Button::new(name, ButtonHandlerVariant::WithState(handler))
    }

    /// Creates a Button that, when clicked, will prompt the user with the `extra_prompts` provided.
    /// Those responses will then be given to the Button's handler. Note that the length of the
    /// Vec of [ExtraResponse]s given to the handler is guaranteed to be the same length as how
    /// many prompts there should be.
    pub fn create_button_with_prompts(
        name: &str,
        handler: Box<dyn Send + Sync + Fn(Vec<ExtraResponse>) -> ButtonResponse>,
        extra_prompts: Vec<String>,
    ) -> Button<S> {
        Button::new(
            name,
            ButtonHandlerVariant::WithExtraPrompts(handler, extra_prompts),
        )
    }

    /// Creates a Button with both [state](Button::create_button_with_state) and [prompts](Button::create_button_with_prompts)
    pub fn create_button_with_state_and_prompts(
        name: &str,
        handler: Box<dyn Send + Sync + Fn(&S, Vec<ExtraResponse>) -> ButtonResponse>,
        extra_prompts: Vec<String>,
    ) -> Button<S> {
        Button::new(name, ButtonHandlerVariant::WithBoth(handler, extra_prompts))
    }
}

pub(crate) enum ButtonHandlerVariant<S: Send + Sync + 'static> {
    Basic(Box<dyn Send + Sync + Fn() -> ButtonResponse>),
    WithState(Box<dyn Send + Sync + Fn(&S) -> ButtonResponse>),
    WithExtraPrompts(
        Box<dyn Send + Sync + Fn(Vec<ExtraResponse>) -> ButtonResponse>,
        Vec<String>,
    ),
    WithBoth(
        Box<dyn Send + Sync + Fn(&S, Vec<ExtraResponse>) -> ButtonResponse>,
        Vec<String>,
    ),
}

#[derive(Deserialize, Debug)]
pub(crate) struct ButtonInfo {
    pub id: usize,
    pub extra_responses: Vec<ExtraResponse>,
}

/// Represents the server's response to a [Button] being pressed. Currently only has a message field.
#[derive(Serialize)]
pub struct ButtonResponse {
    pub message: String,
}

impl From<&str> for ButtonResponse {
    fn from(message: &str) -> Self {
        ButtonResponse {
            message: message.to_string(),
        }
    }
}

impl From<String> for ButtonResponse {
    fn from(message: String) -> Self {
        ButtonResponse { message }
    }
}

/// When a user is asked for an [extra response], there is the option to click "cancel" on the
/// prompt which will result in a `None` variant.
///
/// [extra response]: Button::create_button_with_prompts
pub type ExtraResponse = Option<String>;
