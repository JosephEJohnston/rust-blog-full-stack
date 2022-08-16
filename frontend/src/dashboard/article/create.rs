use stylist::Style;
use yew::{Component, Context, Html, html};
use crate::css::{DASHBOARD_ARTICLE_CREATE_CSS, DASHBOARD_CSS};
use crate::dashboard::side_bar::DashboardSideBar;

pub struct DashboardArticleCreate {

}

impl Component for DashboardArticleCreate {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardArticleCreate {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_CSS).unwrap();
        let create_css = Style::new(DASHBOARD_ARTICLE_CREATE_CSS).unwrap();

        html! {
            <>
                <div id="page" class={ vec![dashboard_css, create_css] }>
                    <DashboardSideBar />
                    <div id="content" class="page-column-right">
                        <div class="page-right-header">
                            <i class="header-icon" data-feather="align-justify"></i>
                        </div>
                        <article class="main-container">
                            <div class="article-create-border main-border">
                                <div class="main-title">
                                    <div class="main-name article-create-name">
                                        {"创建文章"}
                                    </div>
                                    <div class="article-create-title-blank-fill"></div>
                                    <button class="article-create-back-button">
                                        {"返回"}
                                    </button>
                                </div>
                                <hr class="article-create-title-border-line"/>
                                <div class="article-create-input-container">
                                    <div class="for-each-input-container">
                                        <div class="input-name-container">
                                            <div class="input-name">{"分类"}</div>
                                        </div>
                                        <label>
                                            <input class="each-input" type="text" placeholder="选择分类"/>
                                        </label>
                                    </div>
                                    <div class="for-each-input-container">
                                        <div class="input-name-container">
                                            <div class="input-name">{"标题"}</div>
                                        </div>
                                        <label>
                                            <input class="each-input" type="text"/>
                                        </label>
                                    </div>
                                    <div class="for-each-input-container">
                                        <div class="input-name-container">
                                            <div class="input-name">{"副标题"}</div>
                                        </div>
                                        <label>
                                            <input class="each-input" type="text"/>
                                        </label>
                                    </div>
                                    <div class="for-each-input-container">
                                        <div class="input-name-container">
                                            <div class="input-name">{"页面图像"}</div>
                                        </div>
                                        <label>
                                            <input class="each-input" type="text" placeholder=""/>
                                        </label>
                                        <button class="update-file-button">{"上传文件"}</button>
                                    </div>
                                    <div class="for-each-input-container">
                                        <div class="input-name-container">
                                            <div class="input-name">{"内容"}</div>
                                        </div>
                                        <label>
                                            <input class="each-input" type="text"/>
                                        </label>
                                    </div>
                                    <div class="for-each-input-container">
                                        <div class="input-name-container">
                                            <div class="input-name">{"标签"}</div>
                                        </div>
                                        <label>
                                            <input class="each-input" type="text" placeholder="选择标签"/>
                                        </label>
                                    </div>
                                    <div class="for-each-input-container">
                                        <div class="input-name-container">
                                            <div class="input-name">{"主要描述"}</div>
                                        </div>
                                        <label>
                                            <input class="each-input" type="text"/>
                                        </label>
                                    </div>
                                    <div class="for-each-input-container">
                                        <div class="input-name-container">
                                            <div class="input-name">{"日期时间"}</div>
                                        </div>
                                        <label>
                                            <input class="each-input" type="text" placeholder="发布时间"/>
                                        </label>
                                    </div>
                                    <div class="for-each-input-container bottom-check-container">
                                        <label>
                                            <input class="input-check-button" type="text" placeholder="是否草稿？"/>
                                        </label>
                                        <label>
                                            <input class="input-check-button" type="text" placeholder="是否原创？"/>
                                        </label>
                                    </div>
                                    <button class="article-create-create-button">{"创建"}</button>
                                </div>
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

