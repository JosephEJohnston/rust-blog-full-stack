use yew::{Component, Context, Html, html};

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
        html! {
            <>
                <div id="page" class="dashboard-page">
                    <aside class="page-column-left">
                        <div class="user-info">
                            <div class="user-info-detail">
                                <img class="user-img" src="../resource/img/dashboard-img.jpg" alt=""/>
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

