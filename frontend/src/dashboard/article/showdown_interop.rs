use wasm_bindgen::prelude::*;
use yew_interop::declare_resources;

declare_resources!(
    showdown
    "https://cdnjs.cloudflare.com/ajax/libs/showdown/2.1.0/showdown.min.js"
);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Converter)]
    pub type Converter;

    #[wasm_bindgen(constructor, js_class = "Converter")]
    pub fn new() -> Converter;

    #[wasm_bindgen(method, structural, js_class = "Converter", js_name = makeHtml)]
    pub fn makeHtml(this: &Converter, text: String) -> String;
}

