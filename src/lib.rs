#![recursion_limit="2048"]

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use app::App;

mod app;
mod routes;
mod types;
mod services;
mod error;
mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}