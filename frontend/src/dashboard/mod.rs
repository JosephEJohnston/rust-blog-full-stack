pub mod index;
pub mod article;

mod side_bar;

use stylist::Style;
use yew::{Component, Context, Html, html};
use yew_router::{Routable, Switch};
use crate::css::DASHBOARD_CSS;
use crate::dashboard::index::DashboardIndex;
use crate::dashboard::side_bar::DashboardSideBar;
use crate::dashboard::article::DashboardArticle;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum DashboardRoute {
    #[at("/dashboard")]
    Index,

    #[at("/dashboard/article/*")]
    Article,

    #[at("/dashboard/article")]
    ArticleIndex,
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
                <DashboardArticle />
            }
        }

        DashboardRoute::ArticleIndex => {
            html! {
                <DashboardArticle />
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
                <div id="page" class={ dashboard_css }>
                    <DashboardSideBar />
                    <div id="content" class="page-column-right">
                        <div class="page-right-header">
                            <i class="header-icon" data-feather="align-justify"></i>
                        </div>
                        <article class="main-container">
                            <Switch<DashboardRoute> render={Switch::render(switch)} />
                        </article>
                    </div>
                    <footer></footer>
                </div>
                <script src="feather/feather.min.js"></script>
                <script>
                    { "setTimeout(function() { feather.replace(); }, 1000)" }
                </script>
            </>
        }
    }
}

