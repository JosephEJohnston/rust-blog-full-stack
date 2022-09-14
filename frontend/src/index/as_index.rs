use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use share::article::article_base::ArticleListItemHttp;
use share::article::http::ListArticleOptions;
use crate::index::article_list_item::ArticleListItem;
use crate::index::http::list_article_http;
use share::utils::page::{PageRequest, Pagination};
use share::utils::status::StatusOptions;

pub struct AsIndex {
    page: Option<Pagination<Vec<ArticleListItemHttp>>>,
}

impl AsIndex {
    fn render_contain(&self) -> Html {
        if self.page.is_none() {
            html! {

            }
        } else {
            html! {
                {
                    for self.page.as_ref().unwrap().data.iter().map(|article| -> Html {
                        html! {
                            <ArticleListItem article={article.clone()}/>
                        }
                    })
                }
            }
        }
    }

    fn render_page(&self) -> Html {
        if self.page.is_none() {
            html! {

            }
        } else {
            let total_page = self.page.as_ref().unwrap().total_page;

            html! {
                <div id="page-transform">
                    <button class="button-transform-left">{ "<" }</button>
                    {
                        for (1..total_page + 1).into_iter().map(|i| {
                            html! {
                                <button class="button-transform-middle">{ i }</button>
                            }
                        })
                    }
                    <button class="button-transform-right">{ ">" }</button>
                </div>
            }
        }
    }
}

pub enum AsIndexMsg {
    FetchArticleListHttp(Pagination<Vec<ArticleListItemHttp>>),
}

impl Component for AsIndex {
    type Message = AsIndexMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let as_index = AsIndex {
            page: None,
        };

        {
            let link = ctx.link().clone();
            spawn_local(async move {
                if let Ok(articles) = list_article_http(ListArticleOptions {
                    user_id: 1,
                    status: StatusOptions {
                        is_all: false,
                        status: Some(1),
                    },
                    page: PageRequest::init(5),
                }).await {
                    link.send_message(AsIndexMsg::FetchArticleListHttp(articles));
                }
            })
        }

        as_index
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AsIndexMsg::FetchArticleListHttp(articles) => {
                self.page = Some(articles);

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        // 可以使用多个 style，例如 class={ vec![style, test] }
        // let test = Style::new(ARTICLE_CSS).unwrap();

        html! {
            <>
                <div id="content-banner">
                    <p class="banner-p">
                        { "Nothing is impossible." }
                    </p>
                </div>
                <article class="article-container">
                    <div class="for-article-container">
                        { self.render_contain() }
                    </div>
                </article>
                <aside>
                </aside>
                { self.render_page() }
            </>
        }
    }
}
