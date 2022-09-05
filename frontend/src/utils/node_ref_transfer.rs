use web_sys::HtmlInputElement;
use yew::NodeRef;

pub fn to_input(node_ref: &NodeRef) -> Option<HtmlInputElement> {
    node_ref.cast::<HtmlInputElement>()
}

