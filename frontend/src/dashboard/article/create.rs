use gloo::console::log;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{Component, Context, Html, html, NodeRef};
use share::article::article_complete::ArticleCompleteHttp;
use crate::css::{DASHBOARD_ARTICLE_CREATE_CSS, DASHBOARD_MAIN_COMMON};
use crate::dashboard::article::for_editor::{ForEditor};
use crate::dashboard::article::http::add_article_http;
use crate::dashboard::article::simplemde::SimpleMDE;
use crate::utils::node_ref_transfer::to_input;

pub struct ArticleCreateContent {
    pub input_title: NodeRef,
    pub editor: Option<SimpleMDE>,
    pub input_outline: NodeRef,
}

impl Default for ArticleCreateContent {
    fn default() -> Self {
        Self {
            input_title: NodeRef::default(),
            editor: None,
            input_outline: NodeRef::default(),
        }
    }
}

pub struct DashboardArticleCreate {
    create_content: ArticleCreateContent,
}

pub enum DashboardArticleCreateMsg {
    FetchEditor(SimpleMDE),
    Create,
}

impl Component for DashboardArticleCreate {
    type Message = DashboardArticleCreateMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardArticleCreate {
            create_content: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardArticleCreateMsg::FetchEditor(editor) => {
                self.create_content.editor = Some(editor);

                false
            },

            DashboardArticleCreateMsg::Create => {
                let content = self.create_content.editor.as_ref()
                    .map(|editor| editor.value())
                    .unwrap_or("".to_string()).clone();

                let input_title = to_input(&self.create_content.input_title)
                    .map(|input| input.value())
                    .unwrap_or("".to_string()).clone();

                let input_outline = to_input(&self.create_content.input_outline)
                    .map(|input| input.value())
                    .unwrap_or("".to_string()).clone();

                spawn_local(async move {
                    if let Ok(id) = add_article_http(ArticleCompleteHttp {
                        id: None,
                        user_id: 1,
                        title: input_title,
                        outline: input_outline,
                        content: Some(content),
                        tag_list: None
                    }).await {
                        log!(format!("Article id: {:?}", id));
                    }
                });

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_MAIN_COMMON).unwrap();
        let create_css = Style::new(DASHBOARD_ARTICLE_CREATE_CSS).unwrap();

        let editor_callback = ctx.link()
            .callback(|editor| DashboardArticleCreateMsg::FetchEditor(editor));

        let create_callback = ctx.link()
            .callback(|_| DashboardArticleCreateMsg::Create);

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
                                <div class="input-name">{"标题"}</div>
                            </div>
                            <label>
                                <input class="each-input" type="text"
                                    ref={self.create_content.input_title.clone()}/>
                            </label>
                        </div>
                        /*<div class="for-each-input-container">
                            <div class="input-name-container">
                                <div class="input-name">{"副标题"}</div>
                            </div>
                            <label>
                                <input class="each-input" type="text"/>
                            </label>
                        </div>*/
                        /*<div class="for-each-input-container">
                            <div class="input-name-container">
                                <div class="input-name">{"页面图像"}</div>
                            </div>
                            <label>
                                <input class="each-input" type="text" placeholder=""/>
                            </label>
                            <button class="update-file-button">{"上传文件"}</button>
                        </div>*/
                        <div class="editor-container">
                            <div class="input-name-container content-input">
                                <div class="input-name">{"内容"}</div>
                            </div>
                            <ForEditor editor_callback={editor_callback} />
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
                                <input class="each-input" type="text"
                                    ref={self.create_content.input_outline.clone()}/>
                            </label>
                        </div>
                        /*<div class="for-each-input-container">
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
                        </div>*/
                        <button class="article-create-create-button" onclick={ create_callback }>
                            {"创建"}
                        </button>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
        }
    }
}

