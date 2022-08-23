pub mod article;
mod header;
mod footer;
mod as_index;
mod article_list_item;
mod http;

use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::css::{ARTICLE_CSS, GITHUB_MARKDOWN_DARK_CSS, INDEX_CSS};
use crate::index::as_index::AsIndex;
use crate::index::header::IndexHeader;
use crate::index::footer::IndexFooter;
use crate::index::article::Article;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum IndexRoute {
    #[at("/")]
    AsIndex,

    #[at("/article")]
    Article,
}

fn switch(routes: &IndexRoute) -> Html {
    match routes {
        IndexRoute::AsIndex => {
            html! {
                <AsIndex />
            }
        },

        IndexRoute::Article => {
            html! {
                <Article />
            }
        },
    }
}

pub struct Index {

}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Index {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style = Style::new(INDEX_CSS).unwrap();
        // 部分样式在 article_css 中，之后拆分
        let article_css = Style::new(ARTICLE_CSS).unwrap();

        let markdown_css = Style::new(GITHUB_MARKDOWN_DARK_CSS).unwrap();
        // 可以使用多个 style，例如 class={ vec![style, test] }
        // let test = Style::new(ARTICLE_CSS).unwrap();

        html! {
            <>
                <div id="index-page" class={ vec![style, article_css, markdown_css] }>
                    <IndexHeader />
                    <div id="content">
                        <Switch<IndexRoute> render={Switch::render(switch)} />
                    </div>
                    <IndexFooter />
                </div>
            </>
        }
    }
}
