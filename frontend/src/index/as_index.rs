use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use share::article::article_base::ArticleListItemHttp;
use share::article::http::ListArticleOptions;
use crate::utils::page::Page;
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
    TransferPage(PageRequest),
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

            AsIndexMsg::TransferPage(page_request) => {
                self.fetch_article_list_http(ctx, page_request);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        // 可以使用多个 style，例如 class={ vec![style, test] }
        // let test = Style::new(ARTICLE_CSS).unwrap();
        let callback = ctx.link()
            .callback(|request| AsIndexMsg::TransferPage(request));

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
                <Page page_bar={self.page.as_ref().map(|page| page.make_page_bar())}
                        callback={callback} />
            </>
        }
    }
}
