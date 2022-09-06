use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use share::article::article_complete::ArticleCompleteHttp;
use crate::css::{DASHBOARD_ARTICLE_CREATE_CSS, DASHBOARD_MAIN_COMMON};
use crate::dashboard::article::DashboardArticleRoute;
use crate::dashboard::article::for_editor::{ForEditor};
use crate::dashboard::article::http::add_article_http;
use crate::dashboard::article::simplemde::SimpleMDE;
use crate::dashboard::article::create_input::{CreateInput, ValidateMaintain};
use crate::utils::node_ref_transfer::to_input;

pub struct ArticleCreateContent {
    pub title_validate: ValidateMaintain,
    pub sub_title_validate: ValidateMaintain,

    pub editor_validate: ValidateMaintain,
    pub editor: Option<SimpleMDE>,

    pub outline_validate: ValidateMaintain,
}

impl Default for ArticleCreateContent {
    fn default() -> Self {
        Self {
            title_validate: ValidateMaintain::new(),
            sub_title_validate: ValidateMaintain::new(),

            editor_validate: ValidateMaintain::new(),
            editor: None,

            outline_validate: ValidateMaintain::new(),
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

impl DashboardArticleCreate {
    fn create_article(&self) -> ArticleCompleteHttp {
        let content = self.create_content.editor.as_ref()
            .map(|editor| editor.value())
            .unwrap_or("".to_string()).clone();

        let input_title = to_input(&self.create_content.title_validate.input)
            .map(|input| input.value())
            .unwrap_or("".to_string()).clone();

        let input_outline = to_input(&self.create_content.outline_validate.input)
            .map(|input| input.value())
            .unwrap_or("".to_string()).clone();

        ArticleCompleteHttp {
            id: None,
            user_id: 1,
            title: input_title,
            outline: input_outline,
            content: Some(content),
            tag_list: None
        }
    }

    fn validate_article(&mut self, article: &ArticleCompleteHttp) -> bool {
        if article.title.len() <= 0 {
            self.create_content.title_validate.set_wrong("标题不能为空".to_string());
        } else {
            self.create_content.title_validate.set_right();
        }

        if article.outline.len() <= 0 {
            self.create_content.outline_validate.set_wrong("主要描述不能为空".to_string());
        } else {
            self.create_content.outline_validate.set_right();
        }

        if let Some(content) = article.content.as_ref() {
            if content.len() <= 0 {
                self.create_content.editor_validate.set_wrong("内容不能为空".to_string());
            } else {
                self.create_content.editor_validate.set_right();
            }
        }

        return false;
    }

    fn send_article_and_redirect(&mut self, any_history: AnyHistory, article: ArticleCompleteHttp) {
        spawn_local(async move {
            if let Ok(_id) = add_article_http(&article).await {
                any_history.push(DashboardArticleRoute::Manage);
            }
        });
    }
}

impl Component for DashboardArticleCreate {
    type Message = DashboardArticleCreateMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardArticleCreate {
            create_content: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardArticleCreateMsg::FetchEditor(editor) => {
                self.create_content.editor = Some(editor);

                false
            },

            DashboardArticleCreateMsg::Create => {
                let article = self.create_article();

                if self.validate_article(&article) {
                    let history = ctx.link().history().unwrap();

                    self.send_article_and_redirect(history, article);
                }

                true
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
                        <CreateInput input_name={"标题"}
                                    validate_result={self.create_content.title_validate.validate_result.clone()}
                                    validate_msg={self.create_content.title_validate.validate_msg.clone()}>
                            <label>
                                <input class="each-input" type="text"
                                    ref={self.create_content.title_validate.input.clone()}/>
                            </label>
                        </CreateInput>
                        <CreateInput input_name={"副标题"}
                                    validate_result={self.create_content.sub_title_validate.validate_result.clone()}
                                    validate_msg={self.create_content.sub_title_validate.validate_msg.clone()}>
                            <label>
                                <input class="each-input" type="text"
                                    ref={self.create_content.sub_title_validate.input.clone()}/>
                            </label>
                        </CreateInput>
                        <CreateInput input_name={"内容"}
                                    validate_result={self.create_content.sub_title_validate.validate_result.clone()}
                                    validate_msg={self.create_content.sub_title_validate.validate_msg.clone()}>
                            <ForEditor editor_callback={editor_callback} />
                        </CreateInput>
                        <div class="for-each-input-container">
                            <div class="input-name-container">
                                <div class="input-name">{"标签"}</div>
                            </div>
                            <label>
                                <input class="each-input" type="text" placeholder="选择标签"/>
                            </label>
                        </div>
                        <CreateInput input_name={"主要描述"}
                                    validate_result={self.create_content.sub_title_validate.validate_result.clone()}
                                    validate_msg={self.create_content.sub_title_validate.validate_msg.clone()}>
                            <label>
                                <input class="each-input" type="text"
                                    ref={self.create_content.outline_validate.input.clone()}/>
                            </label>
                        </CreateInput>
                        <button class="article-create-create-button" onclick={ create_callback }>
                            {"创建"}
                        </button>
                    </div>
                </div>
            </>
        }
    }
}

