
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::css::{INDEX_CSS};
use crate::Route;

pub struct Index {

}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Index {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style = Style::new(INDEX_CSS).unwrap();
        // 可以使用多个 style，例如 class={ vec![style, test] }
        // let test = Style::new(ARTICLE_CSS).unwrap();

        html! {
            <>
                <div class={ style } id="index-page">
                    <header>
                        <nav id="header-nav">
                            <div class="nav-side"></div>
                            <div class="nav-middle">
                                <div class="nav-button-left">
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
                        <div id="header-banner">
                            <p class="banner-p">
                                { "Nothing is impossible." }
                            </p>
                        </div>
                    </header>
                    <div id="content">
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
                                            <Link<Route> to={ Route::Article }>
                                                { "Read More >" }
                                            </Link<Route>>
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
                    </div>
                    <footer id="footer">
                        <nav>
                            <button class="footer-button footer-button-img"></button>
                            <button class="footer-button footer-button-img"></button>
                            <button class="footer-button footer-button-img"></button>
                        </nav>
                    </footer>
                </div>
            </>
        }
    }
}
