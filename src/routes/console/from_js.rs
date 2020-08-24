use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub fn addEditEvent();
    pub fn show();
    pub fn unShow();
    pub fn deleteBackDrop();
}