use gloo::console::log;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use share::article::article_complete::ArticleCompleteHttp;
use crate::css::{DASHBOARD_ARTICLE_CREATE_CSS, DASHBOARD_MAIN_COMMON};
use crate::dashboard::article::DashboardArticleRoute;
use crate::dashboard::article::for_editor::{ForEditor};
use crate::dashboard::article::http::{add_article_http, update_article_http};
use crate::dashboard::article::simplemde::SimpleMDE;
use crate::dashboard::article::create_input::{CreateInput, ValidateMaintain};
use crate::index::http::{get_article_http, GetArticleOptions};

pub struct ArticleCreateContent {
    pub title_validate: ValidateMaintain,

    pub editor_validate: ValidateMaintain,
    pub editor: Option<SimpleMDE>,

    pub outline_validate: ValidateMaintain,
}

impl Default for ArticleCreateContent {
    fn default() -> Self {
        Self {
            title_validate: ValidateMaintain::new(),

            editor_validate: ValidateMaintain::new(),
            editor: None,

            outline_validate: ValidateMaintain::new(),
        }
    }
}

pub struct DashboardArticleCreate {
    create_content: ArticleCreateContent,
}

impl DashboardArticleCreate {
    fn create_article(&self, ctx: &Context<Self>) -> ArticleCompleteHttp {
        let content = self.create_content.editor.as_ref()
            .map(|editor| editor.get_value())
            .unwrap_or("".to_string()).clone();

        let input_title = self.create_content
            .title_validate.input.get_value();

        let input_outline = self.create_content
            .outline_validate.input.get_value();

        ArticleCompleteHttp {
            id: ctx.props().article_id.clone(),
            user_id: 1,
            title: input_title,
            outline: input_outline,
            content: Some(content),
            tag_list: None
        }
    }

    fn validate_article(&mut self, article: &ArticleCompleteHttp) -> bool {
        let mut res = true;

        if article.title.len() <= 0 {
            self.create_content.title_validate.set_wrong("标题不能为空".to_string());

            res = false;
        } else {
            self.create_content.title_validate.set_right();
        }

        if article.outline.len() <= 0 {
            self.create_content.outline_validate.set_wrong("主要描述不能为空".to_string());

            res = false;
        } else {
            self.create_content.outline_validate.set_right();
        }

        if let Some(content) = article.content.as_ref() {
            if content.len() <= 0 {
                self.create_content.editor_validate.set_wrong("内容不能为空".to_string());

                res = false;
            } else {
                self.create_content.editor_validate.set_right();
            }
        }

        return res;
    }

    fn send_add_article_and_redirect(&mut self, any_history: AnyHistory, article: ArticleCompleteHttp) {
        spawn_local(async move {
            match add_article_http(&article).await {
                Ok(_id) => any_history.push(DashboardArticleRoute::Manage),
                Err(e) => log!(format!("{:?}", e)),
            }
        });
    }

    fn send_update_article_and_redirect(&mut self, any_history: AnyHistory, article: ArticleCompleteHttp) {
        spawn_local(async move {
            match update_article_http(&article).await {
                Ok(_id) => any_history.push(DashboardArticleRoute::Manage),
                Err(e) => log!(format!("{:?}", e)),
            }
        });
    }

    fn render_page_title(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().article_id.is_none() {
            html! {
                <div class="main-name article-create-name">
                    {"创建文章"}
                </div>
            }
        } else {
            html! {
                <div class="main-name article-create-name">
                    {"修改文章"}
                </div>
            }
        }
    }

    fn render_button(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().article_id.is_none() {
            html! {
                <button class="article-create-create-button" onclick={ ctx.link()
                    .callback(|_| DashboardArticleCreateMsg::Create) }>
                    {"创建"}
                </button>
            }
        } else {
            html! {
                <button class="article-create-create-button" onclick={ ctx.link()
                    .callback(|_| DashboardArticleCreateMsg::UpdateConfirm) }>
                    {"修改"}
                </button>
            }
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct DashboardArticleCreateProps {
    #[prop_or_default]
    pub article_id: Option<i64>,
}

pub enum DashboardArticleCreateMsg {
    FetchEditor(SimpleMDE),
    Create,
    UpdateInit(ArticleCompleteHttp),
    UpdateConfirm,
}

impl Component for DashboardArticleCreate {
    type Message = DashboardArticleCreateMsg;
    type Properties = DashboardArticleCreateProps;

    fn create(ctx: &Context<Self>) -> Self {
        if let Some(article_id) =  ctx.props().article_id.as_ref() {
            let link = ctx.link().clone();
            let article_id = article_id.clone();

            spawn_local(async move {
                let opts = GetArticleOptions {
                    id: article_id,
                    markdown_opt: 0
                };

                if let Ok(article) = get_article_http(opts).await {
                    link.send_message(DashboardArticleCreateMsg::UpdateInit(article));
                }
            });
        }

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
                let article = self.create_article(ctx);

                if self.validate_article(&article) {
                    let history = ctx.link().history().unwrap();

                    self.send_add_article_and_redirect(history, article);
                }

                true
            },

            DashboardArticleCreateMsg::UpdateInit(article) => {

                self.create_content.title_validate.input.set_value(article.title);
                self.create_content.outline_validate.input.set_value(article.outline);
                self.create_content.editor.as_ref()
                    .map(|editor| editor.set_value(article.content.unwrap()));

                true
            },

            DashboardArticleCreateMsg::UpdateConfirm => {
                let article = self.create_article(ctx);

                if self.validate_article(&article) {
                    let history = ctx.link().history().unwrap();

                    self.send_update_article_and_redirect(history, article);
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

        html! {
            <>
                <div class={ vec![create_css, dashboard_css] }>
                    <div class="main-title">
                        { self.render_page_title(ctx) }
                        <div class="article-create-title-blank-fill"></div>
                        <button class="article-create-back-button">
                            <Link<DashboardArticleRoute> to={ DashboardArticleRoute::Manage }>
                                {"返回"}
                            </Link<DashboardArticleRoute>>
                        </button>
                    </div>
                    <hr class="article-create-title-border-line"/>
                    <div class="article-create-input-container">
                        <CreateInput input_name={"标题"}
                                    validate_result={self.create_content.title_validate.validate_result.clone()}
                                    validate_msg={self.create_content.title_validate.validate_msg.clone()}>
                            <label>
                                <input class="each-input" type="text"
                                    ref={self.create_content.title_validate.input.get_node_ref()}/>
                            </label>
                        </CreateInput>
                        <CreateInput input_name={"内容"}
                                    validate_result={self.create_content.editor_validate.validate_result.clone()}
                                    validate_msg={self.create_content.editor_validate.validate_msg.clone()}>
                            <ForEditor editor_callback={editor_callback} />
                        </CreateInput>
                        <CreateInput input_name={"主要描述"}
                                    validate_result={self.create_content.outline_validate.validate_result.clone()}
                                    validate_msg={self.create_content.outline_validate.validate_msg.clone()}>
                            <label>
                                <input class="each-input" type="text"
                                    ref={self.create_content.outline_validate.input.get_node_ref()}/>
                            </label>
                        </CreateInput>
                        { self.render_button(ctx) }
                    </div>
                </div>
            </>
        }
    }
}

