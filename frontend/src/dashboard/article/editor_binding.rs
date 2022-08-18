use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/dashboard/article/editor.js")]
extern "C" {
    pub type Editor;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Editor;

    #[wasm_bindgen(method)]
    pub fn init(this: &Editor, element_id: &str);
}