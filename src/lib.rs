use wasm_bindgen::prelude::*;
pub mod wieszcz;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    //pub fn how_much_clicked() -> usize;
}

#[wasm_bindgen]
pub fn how_much_clicked() -> String {
    crate::wieszcz::go(3)
}

#[wasm_bindgen]
pub fn go(strophe: usize) -> String {
    crate::wieszcz::go(strophe)
}

#[wasm_bindgen]
pub fn title() -> String {
    use crate::wieszcz::title;
    title()
}

