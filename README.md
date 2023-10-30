# Btnify
Hosts a website with buttons for you so you can focus on what matters!

Btnify is a small library that lets you host a website with some buttons that will call a function or closure
when clicked. Under the hood, Btnify uses [Axum](https://crates.io/crates/axum). This library is, I must admit,
rather crude, but it works, and it's open source! Please leave a pull request with any improvements you have :)
I would appreciate it very much.

# Examples

```rust
use btnify::{
	bind_server,
	button::{Button, ButtonResponse}
};

fn greet_handler(_: &()) -> ButtonResponse {
    ButtonResponse::from("Hello world!")
}

let greet_button = Button::new("Greet", greet_handler);

let buttons = vec![greet_button];

// Notice: bind_server is async and you must await it
bind_server(&"0.0.0.0:3000".parse().unwrap(), buttons, ())
    .await
    .unwrap();
```

```rust
use std::sync::Mutex;
use btnify::{
	bind_server,
	button::{Button, ButtonResponse}
};

struct Counter {
    count: Mutex<i32>
}

fn count_handler(state: &Counter) -> ButtonResponse {
    let mut count = state.count.lock().unwrap();
    *count += 1;
    format!("The count now is: {count}").into()
}

let count_button = Button::new("Count", count_handler);

let buttons = vec![count_button];

// Notice: bind_server is async and you must await it
bind_server(&"0.0.0.0:3000".parse().unwrap(), buttons, ())
    .await
    .unwrap();
```
