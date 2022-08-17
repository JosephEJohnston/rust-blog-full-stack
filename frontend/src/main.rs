mod tests;
mod css;
mod index;
mod dashboard;

use yew::prelude::*;
use yew_router::{Routable, BrowserRouter};
use yew_router::prelude::*;
use crate::index::article::Article;
use crate::index::Index;
use crate::dashboard::Dashboard;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/article")]
    Article,

    #[at("/dashboard/:s")]
    Dashboard,

    #[at("/dashboard/index")]
    DashboardIndex,
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
                <Dashboard />
            }
        },

        Route::DashboardIndex => {
            html! {
                <Dashboard />
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
