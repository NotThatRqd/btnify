<div align="center">
<pre>
██████╗ ████████╗███╗   ██╗██╗███████╗██╗   ██╗
██╔══██╗╚══██╔══╝████╗  ██║██║██╔════╝╚██╗ ██╔╝
██████╔╝   ██║   ██╔██╗ ██║██║█████╗   ╚████╔╝ 
██╔══██╗   ██║   ██║╚██╗██║██║██╔══╝    ╚██╔╝  
██████╔╝   ██║   ██║ ╚████║██║██║        ██║   
╚═════╝    ╚═╝   ╚═╝  ╚═══╝╚═╝╚═╝        ╚═╝   

---------------------------------------------------
rust library to simplify allowing user input over the web
</pre>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/NotThatRqd/btnify/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/btnify)](https://docs.rs/btnify)

</div>

> Hosts a website with buttons for you so you can focus on what matters!

Btnify is a small library that lets you host a website with some buttons that will call a function or closure
when clicked. Under the hood, Btnify uses [Axum](https://github.com/tokio-rs/axum). This library is pretty simple,
but it works, and it's open source! Please leave a pull request with any improvements you have :) I would appreciate it
very much.

## Installation

Run `cargo add btnify`

or

Add `btnify = "2.0.0"` to your `Cargo.toml`

## How to use

[Docs are here](https://docs.rs/btnify)

## Examples

Hello World

```rust
use btnify::button::{Button, ButtonResponse};

fn greet_handler() -> ButtonResponse {
    ButtonResponse::from("hello world!")
}

// this button doesn't use any state so we will mark the state generic as unit
let greet_button: Button<()> = Button::create_basic_button("Greet!", Box::new(greet_handler));
```

Hello World 2.0

```rust
use btnify::button::{Button, ButtonResponse};

fn better_greet_handler(responses: Vec<Option<String>>) -> ButtonResponse {
    // responses is guaranteed to be the same length as the number of extra prompts
    // specified when creating a button
    let name = &responses[0];
    match name {
        Some(name) => format!("Hello, {name}").into(),
        None => format!("You didn't provide a name! :(").into()
    }
}

let better_greet_button: Button<()> = Button::create_button_with_prompts(
    "Greet 2.0",
    Box::new(better_greet_handler),
    vec!["What's your name?".to_string()]
);
```

Counter App

```rust
use std::sync::Mutex;
use tokio::sync::oneshot;
use btnify::bind_server;
use btnify::ShutdownConfig;
use btnify::button::{Button, ButtonResponse};

struct Counter {
    // must use Mutex for interior mutability
    count: Mutex<i32>,
    end_server_tx: Mutex<Option<oneshot::Sender<()>>>,
}

impl Counter {
    fn new(tx: oneshot::Sender<()>) -> Counter {
        Counter {
            count: Mutex::new(0),
            end_server_tx: Mutex::new(Some(tx)),
        }
    }

    fn end_server(&self) {
        // Acquire the Mutex to modify
        let mut tx = self.end_server_tx.lock().unwrap();

        // Take the sender
        let tx = tx.take().unwrap();

        // Send the signal to end the server
        tx.send(()).unwrap();
    }
}

fn count_handler(state: &Counter) -> ButtonResponse {
    let count  = state.count.lock().unwrap();
    format!("The count is: {count}").into()
}

fn plus_handler(state: &Counter, responses: Vec<Option<String>>) -> ButtonResponse {
    match &responses[0] {
        Some(response_str) => {
            if let Ok(amount) = response_str.parse::<i32>() {
                let mut count = state.count.lock().unwrap();
                *count += amount;
                format!("The count now is: {}", *count).into()
            } else {
                "You did not provide a number.".into()
            }
        }
        None => "You didn't provide any input.".into(),
    }
}

fn end_button_handler(state: &Counter) -> ButtonResponse {
    state.end_server();
    "Server is ending. Goodbye!".into()
}

fn server_end(state: &Counter) {
    println!("goodbye world. ;(");
}

let count_button = Button::create_button_with_state("Counter", Box::new(count_handler));

let plus_button = Button::create_button_with_state_and_prompts(
    "add to counter",
    Box::new(plus_handler),
    vec!["How much do you want to add?".to_string()]
);

let end_button = Button::create_button_with_state("End Server", Box::new(end_button_handler));

let buttons = vec![count_button, plus_button, end_button];

let (tx, rx) = oneshot::channel();

let shutdown_config = ShutdownConfig::new(Some(rx), Some(Box::new(server_end)));

bind_server(&"0.0.0.0:3000".parse().unwrap(), buttons, Counter::new(tx), None);
// uncomment to actually run the server:
//    .await
//    .unwrap();
```
