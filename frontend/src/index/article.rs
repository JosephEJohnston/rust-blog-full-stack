use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use yew::{Component, Context, Html, html, Properties};
use share::article::article_complete::ArticleCompleteHttp;
use crate::css::GITHUB_MARKDOWN_DARK_CSS;
use crate::index::http::{get_article_http, GetArticleOptions};
use crate::utils::raw_html::RawHtml;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ArticleProps {
    pub article_id: i64,
}

pub struct Article {
    pub article: Option<ArticleCompleteHttp>,
}

pub enum ArticleMsg {
    FetchArticleHttp(ArticleCompleteHttp),
}

impl Component for Article {
    type Message = ArticleMsg;
    type Properties = ArticleProps;

    fn create(ctx: &Context<Self>) -> Self {
        {
            let link = ctx.link().clone();

            let opts = GetArticleOptions {
                id: ctx.props().article_id.clone(),
                markdown_opt: 1
            };

            spawn_local(async move {
                if let Ok(article) = get_article_http(opts).await {
                    link.send_message(ArticleMsg::FetchArticleHttp(article));
                }
            })
        }

        Article {
            article: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ArticleMsg::FetchArticleHttp(article) => {
                self.article = Some(article);

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style = Style::new(GITHUB_MARKDOWN_DARK_CSS).unwrap();

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
                    <article class={ vec![style] }>
                        <div id="user-article">
                            <article class="markdown-body">
                                <RawHtml inner_html={article.content.as_ref()
                                    .unwrap_or(&"".to_string()).clone()} />
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