#![allow(dead_code)]

use yew::{Component, Context, Html, html};
use yew_router::{Routable, Switch};
use crate::dashboard::article::create::DashboardArticleCreate;
use crate::dashboard::article::manage::DashboardArticleManage;

pub mod create;
pub mod manage;
mod simplemde;
mod for_editor;
mod http;
mod create_input;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum DashboardArticleRoute {
    #[at("/dashboard/article")]
    Manage,

    #[at("/dashboard/article/create")]
    Create,

    #[at("/dashboard/article/update/:article_id")]
    Update { article_id: i64 },
}

fn switch(routes: &DashboardArticleRoute) -> Html {
    match routes {
        DashboardArticleRoute::Manage => {
            html! {
                <DashboardArticleManage />
            }
        },

        DashboardArticleRoute::Create => {
            html! {
                <DashboardArticleCreate />
            }
        },

        DashboardArticleRoute::Update { article_id } => {
            html! {
                <DashboardArticleCreate article_id={ Some(article_id.clone()) } />
            }
        }
    }
}

pub struct DashboardArticle {

}

impl Component for DashboardArticle {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardArticle {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Switch<DashboardArticleRoute> render={Switch::render(switch)} />
        }
    }
}

