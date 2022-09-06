use gloo::console::log;
use yew::{Component, Context, Html, html};
use yew_router::prelude::Link;
use yew_feather::search::Search;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use share::article::article_base::ArticleListItemHttp;
use crate::css::{DASHBOARD_ARTICLE_MANAGE_CSS, DASHBOARD_MAIN_COMMON};
use crate::dashboard::article::DashboardArticleRoute;
use crate::index::http::list_article_http;

pub struct DashboardArticleManage {
    article_list: Option<Vec<ArticleListItemHttp>>,
}

impl DashboardArticleManage {
    fn render_article_list(&self, ctx: &Context<Self>) -> Html {
        if let Some(articles) = self.article_list.as_ref() {
            html! {
                {
                    for articles.iter().map(|article| -> Html {
                        let id = article.id.unwrap().clone();
                        html! {
                            <tr class="article-list-row">
                                <td class="article-list-column article-id">{ id }</td>
                                <td class="article-list-column">{ article.title.clone() }</td>
                                <td class="article-list-column">{ article.outline.clone() }</td>
                                <td class="article-list-column">{"50"}</td>
                                <td class="article-list-column">{ article.create_time.unwrap().clone() }</td>
                                <td class="article-list-column">
                                    <button class="article-list-column-button article-list-column-button-update"
                                        onclick={ctx.link().callback(move |_| Msg::ModifyArticle(id.clone()))}>
                                        {"修改"}
                                    </button>
                                    <button class="article-list-column-button article-list-column-button-delete"
                                        onclick={ctx.link().callback(move |_| Msg::DeleteArticle(id.clone()))}>
                                        {"删除"}
                                    </button>
                                </td>
                            </tr>
                        }
                    })
                }
            }
        } else {
            html! {

            }
        }
    }
}

pub enum Msg {
    FetchArticleListHttp(Vec<ArticleListItemHttp>),
    ModifyArticle(i64),
    DeleteArticle(i64),
}

impl Component for DashboardArticleManage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        {
            let link = ctx.link().clone();
            spawn_local(async move {
                if let Ok(articles) = list_article_http().await {
                    link.send_message(Msg::FetchArticleListHttp(articles));
                }
            })
        }

        DashboardArticleManage {
            article_list: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchArticleListHttp(articles) => {
                self.article_list = Some(articles);

                true
            },

            Msg::ModifyArticle(id) => {
                log!(format!("modify: {:?}", id));
                true
            },

            Msg::DeleteArticle(id) => {
                log!(format!("delete: {:?}", id));
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_MAIN_COMMON).unwrap();
        let manage_css = Style::new(DASHBOARD_ARTICLE_MANAGE_CSS).unwrap();

        html! {
            <>
                <div class={ vec![manage_css, dashboard_css] } >
                    <div class="main-title">
                        <div class="main-name article-list-name">{"文章列表"}</div>
                        <div class="article-list-title-fill"></div>
                        <div class="article-list-title-search">
                            <div>
                                <label>
                                    <input class="article-list-search-input" type="text"/>
                                </label>
                                <button class="article-list-search-button">
                                    <Search class="article-list-search-button-icon" />
                                </button>
                            </div>
                            <button class="article-list-create-button">
                                <Link<DashboardArticleRoute> to={ DashboardArticleRoute::Create }>
                                    {"创建"}
                                </Link<DashboardArticleRoute>>
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
                            { self.render_article_list(ctx) }
                        </tbody>
                    </table>
                </div>
            </>
        }
    }
}