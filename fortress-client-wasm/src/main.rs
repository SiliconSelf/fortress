#![doc = include_str!("../README.md")]

use leptos::{mount_to_body, view};

fn main() {
    mount_to_body(|| view! { <p class="text-red-500">"Hello, world!"</p> });
}
