use wasm_bindgen::prelude::*;
use yew_interop::declare_resources;

declare_resources!(
    showdown
    "https://cdnjs.cloudflare.com/ajax/libs/showdown/2.1.0/showdown.min.js"
);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = showdown)]
    pub type Showdown;

    pub static SHOWDOWN: Showdown;

    #[wasm_bindgen(js_name = Converter)]
    pub type Converter;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn Converter(this: &Showdown) -> Converter;

    #[wasm_bindgen(method, structural)]
    pub fn makeHtml(this: &Converter, text: String) -> String;
}

