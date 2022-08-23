use yew::prelude::*;
use yew_interop::script::wasm_bindgen_futures::spawn_local;
use share::article::ArticleHttp;
use crate::index::article_list_item::ArticleListItem;
use crate::index::http::list_article_from_http;

pub struct AsIndex {
    article_list: Vec<ArticleHttp>,
}

pub enum AsIndexMsg {
    HttpFetchArticleList(Vec<ArticleHttp>),
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
                if let Ok(articles) = list_article_from_http().await {
                    link.send_message(AsIndexMsg::HttpFetchArticleList(articles));
                }
            })
        }

        as_index
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AsIndexMsg::HttpFetchArticleList(articles) => {
                self.article_list = articles;

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        // 可以使用多个 style，例如 class={ vec![style, test] }
        // let test = Style::new(ARTICLE_CSS).unwrap();

        html! {
            <>
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
