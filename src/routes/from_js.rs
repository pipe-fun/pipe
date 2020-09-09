use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub fn addEditEvent();
    pub fn show();
    pub fn unShow();
    pub fn deleteBackDrop();
    pub fn login_btn_disable();
    pub fn login_btn_enable();
    pub fn register_btn_disable();
    pub fn register_btn_enable();
    pub fn active_btn_disable();
    pub fn active_btn_enable();
    pub fn send_btn_disable();
    pub fn send_btn_enable();
    pub fn change_btn_disable();
    pub fn change_btn_enable();
}