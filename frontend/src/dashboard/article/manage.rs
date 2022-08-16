use yew::{Component, Context, Html, html};
use yew_router::prelude::Link;
use stylist::Style;
use crate::css::{DASHBOARD_ARTICLE_MANAGE_CSS, DASHBOARD_CSS};
use crate::Route;
use crate::dashboard::side_bar::DashboardSideBar;

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
                    <DashboardSideBar />
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