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
        html! {
            <>
                <div id="content-banner" class="article-banner">
                    <div class="banner-title">{ "Ubuntu 16.04 环境安装部署" }</div>
                    <p class="banner-brief">{ "项目部署文档" }</p>
                    <div>
                        <button class="banner-tag">{ "入门" }</button>
                        <button class="banner-tag">{ "Linux" }</button>
                        <button class="banner-tag">{ "Laravel" }</button>
                        <button class="banner-tag">{ "Ubuntu" }</button>
                        <button class="banner-tag">{ "PHP7" }</button>
                    </div>
                </div>
                <article>
                    <div id="user-article">
                        <article class="markdown-body">
                            { "我是文章内容" }
                        </article>
                    </div>
                </article>
            </>
        }
    }
}