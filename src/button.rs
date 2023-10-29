use serde::{Deserialize, Serialize};

/// Represents a button you can put on your btnify server.
///
/// `Handler` is a function or closure that takes a reference to a user provided state (`S`) and
/// returns `ButtonResponse`. It will be called whenever this button is pressed.
///
/// # Examples
///
/// ```
/// use btnify::button::{Button, ButtonResponse};
///
/// fn greet_handler(_: &()) -> ButtonResponse {
///     ButtonResponse::from("Hello world!")
/// }
///
/// let greet_button = Button::new("Greet", greet_handler);
/// ```
///
/// ---
///
/// ```
/// use std::sync::Mutex;
/// use btnify::button::{Button, ButtonResponse};
///
/// struct Counter {
///     count: Mutex<i32>
/// }
///
/// fn count_handler(state: &Counter) -> ButtonResponse {
///     let mut count = state.count.lock().unwrap();
///     *count += 1;
///     format!("The count now is: {count}").into()
/// }
///
/// let count_button = Button::new("Count", count_handler);
/// ```
pub struct Button<S: Send + Sync + 'static> {
    pub name: String,
    pub handler: Box<dyn (Fn(&S) -> ButtonResponse) + Send + Sync>
}

impl<S: Send + Sync + 'static> Button<S> {
    /// Creates a new Button struct.
    ///
    /// `Name` is the name of the button that will appear on the website.
    ///
    /// `Handler` is a function or closure that takes a reference to a user provided state (`S`) and
    /// returns `ButtonResponse`. It will be called whenever this button is pressed.
    pub fn new<T: Send + Sync + Fn(&S) -> ButtonResponse + 'static>(name: &str, handler: T) -> Button<S> {
        Button {
            name: name.to_string(),
            handler: Box::new(handler)
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ButtonInfo {
    pub id: usize
    // todo: allow any extra data to be sent
}

/// Represents the server's response to a button being pressed. Currently only has a message field.
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
