// src/main.rs

pub mod domain;
pub mod application;
pub mod infrastructure;

#[tokio::main]
async fn main() {
    println!("Server is starting...");
    // در آینده، کد راه‌اندازی سرور Axum در اینجا قرار می‌گیرد
}