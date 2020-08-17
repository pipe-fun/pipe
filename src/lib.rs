#![recursion_limit="256"]
use yew::App;
use crate::routes::login::Login;
use wasm_bindgen::prelude::*;

mod routes;
mod types;
mod services;
mod error;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Login>::new().mount_to_body();
}