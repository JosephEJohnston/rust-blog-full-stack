use yew::{Component, Context, Html, html};
use yew_router::prelude::Link;
use yew_feather::home::Home;
use yew_feather::user::User;
use yew_feather::settings::Settings;
use yew_feather::layout::Layout;
use yew_feather::book::Book;
use yew_feather::message_square::MessageSquare;
use yew_feather::message_circle::MessageCircle;
use yew_feather::tag::Tag;
use yew_feather::tablet::Tablet;
use yew_feather::users::Users;
use yew_feather::file::File;
use yew_feather::fast_forward::FastForward;
use yew_feather::key::Key;
use yew_feather::server::Server;
use crate::dashboard::DashboardRoute;
use crate::index::IndexRoute;

pub struct DashboardSideBar {

}

impl Component for DashboardSideBar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardSideBar {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <aside class="page-column-left">
                    <div class="user-info">
                        <div class="user-info-detail">
                            <img class="user-img" src="/static/resource/img/dashboard-img.jpg" alt="" />
                            <div class="user-name">{"admin"}</div>
                            <div class="user-email">{"admin@noob.com"}</div>
                        </div>
                        <div class="user-info-button">
                            <button class="for-user-info-button">
                                <Link<IndexRoute> to={ IndexRoute::AsIndex }>
                                    <Home class="user-info-icon" />
                                </Link<IndexRoute>>
                            </button>
                            <button class="for-user-info-button">
                                <User class="user-info-icon" />
                            </button>
                            <button class="for-user-info-button">
                                <Settings class="user-info-icon" />
                            </button>
                        </div>
                    </div>
                    <div class="function">
                        <hr/>
                        <div class="module">
                            <div class="for-each-module">
                                <Layout class="module-icon" />
                                <Link<DashboardRoute> to={ DashboardRoute::Index }>
                                    {"面板"}
                                </Link<DashboardRoute>>
                            </div>
                        </div>
                        <div class="module">
                            <div class="module-title">{"内容模块"}</div>
                            <div class="for-each-module">
                                <Book class="module-icon" />
                                <Link<DashboardRoute> to={ DashboardRoute::ArticleIndex }>
                                    {"文章管理"}
                                </Link<DashboardRoute>>
                            </div>
                            <div class="for-each-module">
                                <MessageSquare class="module-icon" />
                                {"讨论管理"}
                            </div>
                            <div class="for-each-module">
                                <MessageCircle class="module-icon" />
                                {"评论管理"}
                            </div>
                            <div class="for-each-module">
                                <Tag class="module-icon" />
                                {"标签管理"}
                            </div>
                            <div class="for-each-module">
                                <Tablet class="module-icon" />
                                {"分类管理"}
                            </div>
                            <div class="for-each-module">
                                <yew_feather::link::Link class="module-icon" />
                                {"友链管理"}
                            </div>
                        </div>
                        <div class="module">
                            <div class="module-title">{"基础模块"}</div>
                            <div class="for-each-module">
                                <Users class="module-icon" />
                                {"用户管理"}
                            </div>
                            <div class="for-each-module">
                                <File class="module-icon" />
                                {"文件管理"}
                            </div>
                        </div>
                        <div class="module">
                            <div class="module-title">{"系统模块"}</div>
                            <div class="for-each-module">
                                <FastForward class="module-icon" />
                                {"访问列表"}
                            </div>
                            <div class="for-each-module">
                                <Key class="module-icon" />
                                {"角色列表"}
                            </div>
                            <div class="for-each-module">
                                <Server class="module-icon" />
                                {"系统配置"}
                            </div>
                        </div>
                    </div>
                </aside>
            </>
        }
    }
}