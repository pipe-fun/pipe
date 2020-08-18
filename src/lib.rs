#![recursion_limit="256"]
use yew::App;
use wasm_bindgen::prelude::*;
use crate::routes::login::Login;

mod routes;
mod types;
mod services;
mod error;
mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Login>::new().mount_to_body();
}