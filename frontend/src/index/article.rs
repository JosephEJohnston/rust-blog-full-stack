use stylist::{Style};
use yew::{Component, Context, Html, html};
use crate::css::{ARTICLE_CSS, INDEX_CSS, GITHUB_MARKDOWN_DARK_CSS};

pub struct Article {

}

impl Component for Article {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Article {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let index_css = Style::new(INDEX_CSS).unwrap();
        let article_css = Style::new(ARTICLE_CSS).unwrap();
        let markdown_css = Style::new(GITHUB_MARKDOWN_DARK_CSS).unwrap();

        html! {
            <>
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