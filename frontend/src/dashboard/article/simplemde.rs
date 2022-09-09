use std::fmt::{Debug, Formatter};
use gloo::utils::document;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = SimpleMDE)]
    pub type SimpleMDE;

    #[wasm_bindgen(constructor, js_class = "SimpleMDE")]
    pub fn new(config: &JsValue) -> SimpleMDE;

    #[wasm_bindgen(method, structural, js_name = "value", js_class = "SimpleMDE")]
    pub fn get_value(this: &SimpleMDE) -> String;

    #[wasm_bindgen(method, structural, js_name = "value", js_class = "SimpleMDE")]
    pub fn set_value(this: &SimpleMDE, value: String);
}

impl Debug for SimpleMDE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpleMDE[value: {:?}]", self.get_value())
    }
}

pub fn create_editor() -> SimpleMDE {
    let config = Object::new();

    Reflect::set(
        &config,
        &"element".into(),
        &document().get_element_by_id("editor").unwrap(),
    ).ok();

    SimpleMDE::new(&config)
}