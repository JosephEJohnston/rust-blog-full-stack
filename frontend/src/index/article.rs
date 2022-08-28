use gloo::console::log;
use yew::{Component, Context, Html, html, Properties};
use yew_interop::script::wasm_bindgen_futures::spawn_local;
use share::article::article_complete::ArticleCompleteHttp;
use crate::index::http::get_article_from_http;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ArticleProps {
    pub article_id: i64,
}

pub struct Article {
    pub article: Option<ArticleCompleteHttp>,
}

pub enum ArticleMsg {
    HttpFetchArticle(ArticleCompleteHttp),
}

impl Component for Article {
    type Message = ArticleMsg;
    type Properties = ArticleProps;

    fn create(ctx: &Context<Self>) -> Self {
        {
            let link = ctx.link().clone();
            let article_id = ctx.props().article_id;
            spawn_local(async move {
                if let Ok(article) = get_article_from_http(article_id).await {
                    link.send_message(ArticleMsg::HttpFetchArticle(article));
                }
            })
        }

        Article {
            article: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ArticleMsg::HttpFetchArticle(article) => {
                self.article = Some(article);

                log!(format!("{:?}", self.article));

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(article) = self.article.as_ref() {
            html! {
                <>
                    <div id="content-banner" class="article-banner">
                        <div class="banner-title">{ article.title.clone() }</div>
                        <p class="banner-brief">{ article.outline.clone() }</p>
                        <div>
                            {
                                for article.tag_list.as_ref().unwrap().iter().map(|tag| {
                                    html! {
                                        <button class="banner-tag">{ tag.name.clone() }</button>
                                    }
                                })
                            }
                        </div>
                    </div>
                    <article>
                        <div id="user-article">
                            <article class="markdown-body">
                                { article.content.as_ref().unwrap().clone() }
                            </article>
                        </div>
                    </article>
                </>
            }
        } else {
            html! {
                <>
                    <div id="content-banner">
                        <p class="banner-p">
                            { "Nothing is impossible." }
                        </p>
                    </div>
                    <article>
                        <div id="user-article">
                            <article class="markdown-body">
                                { "内容加载中，请稍作等待。" }
                            </article>
                        </div>
                    </article>
                </>
            }
        }
    }
}