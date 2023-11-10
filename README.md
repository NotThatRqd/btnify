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

Add `btnify = "1.0.0"` to your `Cargo.toml`

## How to use

[Docs are here](https://docs.rs/btnify)

## Examples

Hello World

```rust
use btnify::button::{Button, ButtonResponse, ExtraResponse};

fn greet_handler() -> ButtonResponse {
    ButtonResponse::from("hello world!")
}

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
use btnify::bind_server;
use btnify::button::{Button, ButtonResponse, ExtraResponse};

struct Counter {
    // must use mutex to be thread-safe
    count: Mutex<i32>
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: Mutex::new(0)
        }
    }
}

fn count_handler(state: &Counter) -> ButtonResponse {
    let count  = state.count.lock().unwrap();
    format!("The count is: {count}").into()
}

fn plus_handler(counter_struct: &Counter, responses: Vec<Option<String>>) -> ButtonResponse {
    match &responses[0] {
        Some(response_str) => {
            if let Ok(amount) = response_str.parse::<i32>() {
                let mut count = counter_struct.count.lock().unwrap();
                *count += amount;
                format!("The count now is: {}", *count).into()
            } else {
                "You did not provide a number.".into()
            }
        }
        None => "You didn't provide any input.".into(),
    }
}

let count_button = Button::create_button_with_state("Counter", Box::new(count_handler));

let plus_button = Button::create_button_with_state_and_prompts(
    "add to counter",
    Box::new(plus_handler),
    vec!["How much do you want to add?".to_string()]
);

let buttons = [count_button, plus_button];

// uncomment to run server on localhost:3000
// bind_server(&"0.0.0.0:3000".parse().unwrap(), buttons, Counter::new())
//    .await
//    .unwrap();
```
