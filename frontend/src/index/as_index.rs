use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use share::article::article_base::ArticleListItemHttp;
use crate::index::article_list_item::ArticleListItem;
use crate::index::http::list_article_http;

pub struct AsIndex {
    article_list: Vec<ArticleListItemHttp>,
}

pub enum AsIndexMsg {
    FetchArticleListHttp(Vec<ArticleListItemHttp>),
}

impl Component for AsIndex {
    type Message = AsIndexMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let as_index = AsIndex {
            article_list: Vec::new(),
        };

        {
            let link = ctx.link().clone();
            spawn_local(async move {
                if let Ok(articles) = list_article_http().await {
                    link.send_message(AsIndexMsg::FetchArticleListHttp(articles));
                }
            })
        }

        as_index
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AsIndexMsg::FetchArticleListHttp(articles) => {
                self.article_list = articles.into_iter()
                    .filter(|article| article.status == 1)
                    .collect();

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
                        {
                            for self.article_list.iter().map(|article| -> Html {
                                html! {
                                    <ArticleListItem article={article.clone()}/>
                                }
                            })
                        }
                    </div>
                </article>
                <aside>
                </aside>
                <div id="page-transform">
                    <button class="button-transform-left">{ "<" }</button>
                    <button class="button-transform-middle">{ "1" }</button>
                    <button class="button-transform-middle">{ "2" }</button>
                    <button class="button-transform-middle">{ "3" }</button>
                    <button class="button-transform-right">{ ">" }</button>
                </div>
            </>
        }
    }
}
