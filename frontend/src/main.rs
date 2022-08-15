mod tests;
mod css;
mod article;
mod index;

use yew::prelude::*;
use yew_router::{Routable, BrowserRouter};
use yew_router::prelude::*;
use crate::article::Article;
use crate::index::Index;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/articles")]
    Article,
}


fn switch(routes: &Route) -> Html {
    match routes {
        Route::Article => {
            html! {
                <Article />
            }
        }
    }
}

pub struct App {

}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                    <Index />
                </BrowserRouter>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
