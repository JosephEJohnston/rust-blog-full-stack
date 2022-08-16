use yew::{Component, Context, Html, html};
use yew_router::prelude::Link;
use crate::Route;

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
                            <img class="user-img" src="../resource/img/dashboard-img.jpg" alt="" />
                            <div class="user-name">{"admin"}</div>
                            <div class="user-email">{"admin@pigjian.com"}</div>
                        </div>
                        <div class="user-info-button">
                            <button class="for-user-info-button">
                                <i class="user-info-icon" data-feather="home"></i>
                            </button>
                            <button class="for-user-info-button">
                                <i class="user-info-icon" data-feather="user"></i>
                            </button>
                            <button class="for-user-info-button">
                                <i class="user-info-icon" data-feather="settings"></i>
                            </button>
                        </div>
                    </div>
                    <div class="function">
                        <hr/>
                        <div class="module">
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="layout"></i>
                                {"面板"}
                            </div>
                        </div>
                        <div class="module">
                            <div class="module-title">{"内容模块"}</div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="book"></i>
                                <Link<Route> to={ Route::DashboardArticleManage }>
                                    {"文章管理"}
                                </Link<Route>>
                            </div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="message-square"></i>
                                {"讨论管理"}
                            </div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="message-circle"></i>
                                {"评论管理"}
                            </div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="tag"></i>
                                {"标签管理"}
                            </div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="table"></i>
                                {"分类管理"}
                            </div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="link"></i>
                                {"友链管理"}
                            </div>
                        </div>
                        <div class="module">
                            <div class="module-title">{"基础模块"}</div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="users"></i>
                                {"用户管理"}
                            </div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="file"></i>
                                {"文件管理"}
                            </div>
                        </div>
                        <div class="module">
                            <div class="module-title">{"系统模块"}</div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="fast-forward"></i>
                                {"访问列表"}
                            </div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="key"></i>
                                {"角色列表"}
                            </div>
                            <div class="for-each-module">
                                <i class="module-icon" data-feather="server"></i>
                                {"系统配置"}
                            </div>
                        </div>
                    </div>
                </aside>
            </>
        }
    }
}