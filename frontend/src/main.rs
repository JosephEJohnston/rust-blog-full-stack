mod tests;

use yew::{Component, Context, Html, html};
use yew::virtual_dom::AttrValue;

pub struct App {
    name: AttrValue,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            name: AttrValue::from("Hello, world!"),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>{ self.name.clone() }</div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
