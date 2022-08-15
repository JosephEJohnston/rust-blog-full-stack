use stylist::Style;
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
            <div id="index-page" class={ vec![index_css, article_css, markdown_css] }>
                <header>
                    <nav id="header-nav">
                        <div class="nav-side"></div>
                        <div class="nav-middle">
                            <div class="nav-button-left">
                                <img class="log-img" src="img/logo-w.png" alt="#"/>
                                <button class="nav-button nav-button-big">{ "PJ Blog" }</button>
                                <button class="nav-button nav-button-size-small">{ "文章" }</button>
                                <button class="nav-button nav-button-size-small">{ "分享" }</button>
                                <button class="nav-button nav-button-size-small">{ "文档" }</button>
                            </div>
                            <div class="nav-button-right">
                                <div class="nav-button nav-form-div">
                                    <form class="nav-form" action="">
                                        <input class="form-search" type="text" placeholder="搜索"/>
                                    </form>
                                </div>
                                <button class="nav-button nav-button-size-small">
                                    { "登录" }
                                </button>
                                <button class="nav-button nav-button-size-small">
                                    { "注册" }
                                </button>
                            </div>
                        </div>
                    </nav>
                    <div id="header-banner" class="article-banner">
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
                </header>
                <div id="content">
                    <article>
                        <div id="user-article">
                            <article class="markdown-body">
                                { "我是文章内容" }
                            </article>
                        </div>
                    </article>
                </div>
                <footer id="footer">
                    <nav>
                        <button class="footer-button"></button>
                        <button class="footer-button"></button>
                        <button class="footer-button"></button>
                    </nav>
                </footer>
            </div>
        }
    }
}