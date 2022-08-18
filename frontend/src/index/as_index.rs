use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::css::{ARTICLE_CSS, INDEX_CSS};
use crate::index::IndexRoute;

pub struct AsIndex {

}

impl Component for AsIndex {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AsIndex {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style = Style::new(INDEX_CSS).unwrap();
        // 部分样式在 article_css 中，之后拆分
        let article_css = Style::new(ARTICLE_CSS).unwrap();

        // 可以使用多个 style，例如 class={ vec![style, test] }
        // let test = Style::new(ARTICLE_CSS).unwrap();

        html! {
            <>
                <article class="article-container">
                    <div class="for-article-container">
                        <div class="for-article">
                            <img class="article-img" src="img/article-img.jpg" alt="" />
                            <div class="article-text">
                                <button class="article-title">{ "Ubuntu 16.04 环境安装部署" }</button>
                                <p class="article-brief">{ "项目部署文档" }</p>
                                <hr class="article-border-line" />
                                <div class="article-tag">
                                    <button class="for-article-tag">{ "入门" }</button>
                                    <button class="for-article-tag">{ "Linux" }</button>
                                    <button class="for-article-tag">{ "Laravel" }</button>
                                    <button class="for-article-tag">{ "Ubuntu" }</button>
                                    <button class="for-article-tag">{ "PHP7" }</button>
                                </div>
                                <hr class="article-border-line" />
                                <div class="article-info">
                                    <div class="for-article-info">{ "Jiajian" }</div>
                                    <div class="for-article-info">{ "3年前" }</div>
                                    <div class="for-article-info">{ "15820" }</div>
                                </div>
                                <button class="article-detail-button">
                                    <Link<IndexRoute> to={ IndexRoute::Article }>
                                        { "Read More >" }
                                    </Link<IndexRoute>>
                                </button>
                            </div>
                        </div>
                        <hr class="article-bottom-line" />
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
