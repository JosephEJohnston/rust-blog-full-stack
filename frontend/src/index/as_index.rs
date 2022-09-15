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

    fn render_page_transfer_bar(&self, ctx: &Context<Self>) -> Html {
        if self.page.is_none() {
            html! {

            }
        } else {
            let page = self.page.as_ref().unwrap();
            let total_page = page.total_page;
            let cur_page = page.page;

            html! {
                <div id="page-transform">
                    <button class="button-transform-left"
                        onclick={ ctx.link().callback(move |_| AsIndexMsg::TransferPage(cur_page - 1)) }>
                        { "<" }
                    </button>
                    {
                        for (1..total_page + 1).into_iter().map(|i| {
                            html! {
                                <button class="button-transform-middle"
                                    onclick={ ctx.link().callback(move |_| AsIndexMsg::TransferPage(i)) }>
                                    { i }
                                </button>
                            }
                        })
                    }
                    <button class="button-transform-right" onclick={ ctx.link().callback(move |_| AsIndexMsg::TransferPage(cur_page + 1)) }>
                        { ">" }
                    </button>
                </div>
            }
        }
    }

    fn fetch_article_list_http(&self, ctx: &Context<Self>, page_request: PageRequest) {
        {
            let link = ctx.link().clone();
            spawn_local(async move {
                if let Ok(articles) = list_article_http(ListArticleOptions {
                    user_id: 1,
                    status: StatusOptions {
                        is_all: false,
                        status: Some(1),
                    },
                    page: page_request,
                }).await {
                    link.send_message(AsIndexMsg::FetchArticleListHttp(articles));
                }
            })
        }
    }
}

pub enum AsIndexMsg {
    FetchArticleListHttp(Pagination<Vec<ArticleListItemHttp>>),
    TransferPage(i64),
}

impl Component for AsIndex {
    type Message = AsIndexMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let as_index = AsIndex {
            page: None,
        };

        let page_request = PageRequest::init(5);
        as_index.fetch_article_list_http(ctx, page_request);

        as_index
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AsIndexMsg::FetchArticleListHttp(articles) => {
                self.page = Some(articles);

                true
            },

            AsIndexMsg::TransferPage(page) => {
                let total_page = self.page.as_ref().unwrap().total_page;
                if page == 0 || page > total_page {
                    return false;
                }

                let page_request = PageRequest {
                    page,
                    page_size: 5,
                };

                self.fetch_article_list_http(ctx, page_request);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

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
                { self.render_page_transfer_bar(ctx) }
            </>
        }
    }
}
