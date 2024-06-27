# Session 1

## Intro

[Rust 勉強会 Series](https://tier4.atlassian.net/wiki/x/tgFVvQ)

1. Why?
   * C++ is old, unsafe
   * Rust is as fast but safe (and strict!)
   * We want strict: we are building "killing machines"
2. Resources.

[Session 1](https://tier4.atlassian.net/wiki/x/QYH7vQ)

1. Covering today:
   * installation
   * making a package
   * type system
   * comparing to C++

## Installation
* use rustup
* installs Cargo, rustc, Clippy

## Hello World

* make package
* install rust-analyzer
* compile and run the code

## Types


```rust
// Adapted version of https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Paste(String),
    Click(i32, i32),
    Authentication { username: String, password: String },
}

fn main() {
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::Authentication{username: "autoware".to_owned(), password: "autoware".to_owned()},
        WebEvent::KeyPress('✨'),
        WebEvent::Paste("Hello".to_owned()),
        WebEvent::KeyPress(' '),
        WebEvent::Paste("World!".to_owned()),
        WebEvent::KeyPress('✨'),
        WebEvent::KeyPress('\n'),
        WebEvent::Click(3, 5),
    ];

    for event in events {
        match event {
            WebEvent::PageLoad => println!("Loaded!"),
            WebEvent::KeyPress(c) => print!("{}", c),
            WebEvent::Paste(text) => print!("{}", text),
            WebEvent::Click(x, y) => println!("Clicked at ({}, {}).", x, y),
            WebEvent::Authentication { username, password: _ } => println!("User {} logged in with password {}", username, "*****"),
        }
    }
}
```