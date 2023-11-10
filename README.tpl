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
when clicked. Under the hood, Btnify uses [Axum](https://crates.io/crates/axum). This library is, I must admit,
rather crude, but it works, and it's open source! Please leave a pull request with any improvements you have :)
I would appreciate it very much.

## Installation

Run `cargo add btnify`

or

Add `btnify = "{{version}}"` to your `Cargo.toml`

## How to use

[Docs are here](https://docs.rs/btnify)

{{readme}}
