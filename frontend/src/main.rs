mod tests;
mod css;
mod article;
mod index;
mod dashboard;

use yew::prelude::*;
use yew_router::{Routable, BrowserRouter};
use yew_router::prelude::*;
use crate::article::Article;
use crate::index::Index;
use crate::dashboard::index::DashboardIndex;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/articles")]
    Article,

    #[at("/dashboard")]
    Dashboard,
}


fn switch(routes: &Route) -> Html {
    match routes {
        Route::Index => {
            html! {
                <Index />
            }
        },

        Route::Article => {
            html! {
                <Article />
            }
        },

        Route::Dashboard => {
            html! {
                <DashboardIndex />
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
                </BrowserRouter>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
