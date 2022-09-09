use web_sys::HtmlInputElement;
use yew::NodeRef;

pub struct InputRef {
    node_ref: NodeRef,
}

impl Default for InputRef {
    fn default() -> Self {
        InputRef {
            node_ref: NodeRef::default(),
        }
    }
}

impl InputRef {

    pub fn set_value(&mut self, value: String) {
        self.node_ref.cast::<HtmlInputElement>()
            .map(|input| input.set_value(value.as_str()))
            .unwrap();
    }

    pub fn get_value(&self) -> String {
        self.node_ref.cast::<HtmlInputElement>()
            .map(|input| input.value())
            .unwrap_or("".to_string()).clone()
    }

    pub fn get_node_ref(&self) -> NodeRef {
        self.node_ref.clone()
    }
}
