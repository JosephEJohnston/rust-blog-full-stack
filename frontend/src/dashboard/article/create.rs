use stylist::Style;
use yew::{Component, Context, Html, html, NodeRef};
use crate::css::{DASHBOARD_ARTICLE_CREATE_CSS, DASHBOARD_MAIN_COMMON};
use crate::dashboard::article::for_editor::ForEditor;
use crate::dashboard::article::simplemde_interop::SimpleMDE;

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
    EditorInit,
    FetchEditor(SimpleMDE),
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
            DashboardArticleCreateMsg::EditorInit => {

                true
            },

            DashboardArticleCreateMsg::FetchEditor(editor) => {
                self.create_content.editor = Some(editor);

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_MAIN_COMMON).unwrap();
        let create_css = Style::new(DASHBOARD_ARTICLE_CREATE_CSS).unwrap();
        // let simplemde_css = Style::new(SIMPLEMDE_CSS).unwrap();

        let editor_callback = ctx.link()
            .callback(|editor| DashboardArticleCreateMsg::FetchEditor(editor));

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
                                <div class="input-name">{"标签"}</div>
                            </div>
                            <label>
                                <input class="each-input" type="text" placeholder="选择标签"/>
                            </label>
                        </div>
                        <div class="for-each-input-container">
                            <div class="input-name-container">
                                <div class="input-name">{"标题"}</div>
                            </div>
                            <label>
                                <input class="each-input" type="text"
                                    ref={self.create_content.input_title.clone()}/>
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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
        }
    }
}

