use yew::{Component, Context, Html, html};
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::Link;
use share::article::article_base::ArticleHttp;
use crate::index::article_dispatcher::ArticleDispatcher;
use crate::Route;

pub struct IndexHeader {
    _producer: Box<dyn Bridge<ArticleDispatcher>>,
    article: Option<ArticleHttp>,
}

pub enum IndexHeaderMsg {
    GetArticle(ArticleHttp),
}

impl Component for IndexHeader {
    type Message = IndexHeaderMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        IndexHeader {
            _producer: ArticleDispatcher::bridge(ctx.link().callback(IndexHeaderMsg::GetArticle)),
            article: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexHeaderMsg::GetArticle(article) => {
                self.article = Some(article);

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                    <nav id="header-nav">
                        <div class="nav-side"></div>
                        <div class="nav-middle">
                            <div class="nav-button-left">
                                <img class="log-img" src="/static/resource/img/logo-w.png" alt="#"/>
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
                                    <Link<Route> to={ Route::DashboardIndex }>
                                        { "登录" }
                                    </Link<Route>>
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
            </>
        }
    }
}