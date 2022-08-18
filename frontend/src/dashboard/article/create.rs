use gloo::console::log;
use gloo::utils::document;
use js_sys::{Object, Reflect};
use stylist::Style;
use yew::{Component, Context, Html, html};
use yew_interop::ScriptEffect;
use crate::css::{DASHBOARD_ARTICLE_CREATE_CSS, DASHBOARD_MAIN_COMMON};
use crate::dashboard::article::create_interop;
use crate::dashboard::article::create_interop::SimpleMDE;
use crate::dashboard::article::for_editor::ForEditor;

pub struct DashboardArticleCreate {

}

pub enum DashboardArticleCreateMsg {
    EditorInit,
}

impl Component for DashboardArticleCreate {
    type Message = DashboardArticleCreateMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardArticleCreate {

        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardArticleCreateMsg::EditorInit => {
                /*let config = Object::new();

                Reflect::set(
                    &config,
                    &"element".into(),
                    &document().get_element_by_id("editor").unwrap(),
                ).ok();

                SimpleMDE::new(&config);*/

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_MAIN_COMMON).unwrap();
        let create_css = Style::new(DASHBOARD_ARTICLE_CREATE_CSS).unwrap();
        // let simplemde_css = Style::new(SIMPLEMDE_CSS).unwrap();

        html! {
            <>
                <div class={ vec![create_css, dashboard_css] }>
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
                                <div>
                                    <ForEditor />
                                </div>
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
                // <link rel="stylesheet" href="simplemde/simplemde.min.css" />
                // <script src="/static/js/simplemde/simplemde.min.js"></script>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(DashboardArticleCreateMsg::EditorInit);
        }
    }
}

