use yew::{Component, Context, Html, html};
use yew_router::prelude::Link;
use stylist::Style;
use crate::css::{DASHBOARD_ARTICLE_MANAGE_CSS, DASHBOARD_CSS};
use crate::Route;


pub struct DashboardArticleManage {

}

impl Component for DashboardArticleManage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardArticleManage {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_CSS).unwrap();
        let manage_css = Style::new(DASHBOARD_ARTICLE_MANAGE_CSS).unwrap();

        html! {
            <>
                <div id="page" class={ vec![dashboard_css, manage_css] }>
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
                                    {"文章管理"}
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
                    <div id="content" class="page-column-right">
                        <div class="page-right-header">
                            <i class="header-icon" data-feather="align-justify"></i>
                        </div>
                        <article class="main-container">
                            <div class="main-border">
                                <div class="main-title">
                                    <div class="main-name article-list-name">{"文章列表"}</div>
                                    <div class="article-list-title-fill"></div>
                                    <div class="article-list-title-search">
                                        <div>
                                            <label>
                                                <input class="article-list-search-input" type="text"/>
                                            </label>
                                            <button class="article-list-search-button">
                                                <i class="article-list-search-button-icon" data-feather="search"></i>
                                            </button>
                                        </div>
                                        <button class="article-list-create-button">
                                            <Link<Route> to={ Route::DashboardArticleCreate }>
                                                {"创建"}
                                            </Link<Route>>
                                        </button>
                                    </div>
                                </div>
                                <hr/>
                                <table class="article-list-table">
                                    <tbody>
                                        <tr class="article-list-row">
                                            <td class="article-id">{"ID"}</td>
                                            <td class="article-list-column">{"标题"}</td>
                                            <td class="article-list-column">{"副标题"}</td>
                                            <td class="article-list-column">{"评论数"}</td>
                                            <td class="article-list-column">{"发布时间"}</td>
                                            <td class="article-list-column">{"操作"}</td>
                                        </tr>
                                        <tr class="article-list-row">
                                            <td class="article-list-column article-id">{"1"}</td>
                                            <td class="article-list-column">{"测试标题"}</td>
                                            <td class="article-list-column">{"测试副标题"}</td>
                                            <td class="article-list-column">{"50"}</td>
                                            <td class="article-list-column">{"2022-8-12"}</td>
                                            <td class="article-list-column">
                                                <button class="article-list-column-button article-list-column-button-update">{"修改"}</button>
                                                <button class="article-list-column-button article-list-column-button-delete">{"删除"}</button>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                            </div>
                        </article>
                    </div>
                    <footer>

                    </footer>
                </div>
            </>
        }
    }
}