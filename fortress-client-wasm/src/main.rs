#![doc = include_str!("../README.md")]

use leptos::*;

fn main() {
    mount_to_body(|| view! { <p>"Hello, world!"</p> })
}