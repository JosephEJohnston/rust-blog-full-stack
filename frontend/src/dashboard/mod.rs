pub mod index;
pub mod feather;
pub mod article;
mod side_bar;

use stylist::Style;
use yew::{Component, Context, Html, html};
use yew_router::{Routable, BrowserRouter, Switch};
use crate::css::DASHBOARD_CSS;
use crate::dashboard::index::DashboardIndex;
use crate::dashboard::side_bar::DashboardSideBar;
use crate::dashboard::article::manage::DashboardArticleManage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum DashboardRoute {
    #[at("/dashboard")]
    Index,

    #[at("/dashboard/article")]
    Article,
}

fn switch(routes: &DashboardRoute) -> Html {
    match routes {
        DashboardRoute::Index => {
            html! {
                <DashboardIndex />
            }
        },

        DashboardRoute::Article => {
            html! {
                <h1>{ "test" }</h1>
            }
        },
    }
}

pub struct Dashboard {

}

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Dashboard {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_CSS).unwrap();

        html! {
            <>
                <BrowserRouter>
                    <div id="page" class={ dashboard_css }>
                        <DashboardSideBar />
                        <div id="content" class="page-column-right">
                            <div class="page-right-header">
                                <i class="header-icon" data-feather="align-justify"></i>
                            </div>
                            <Switch<DashboardRoute> render={Switch::render(switch)} />
                        </div>
                        <footer></footer>
                    </div>
                    <script src="feather/feather.min.js"></script>
                    <script>
                        { "setTimeout(function() { feather.replace(); }, 100)" }
                    </script>
                </BrowserRouter>
            </>
        }
    }
}

