use std::collections::HashMap;
use yew::{Component, Context, Html, html};
use yew_router::prelude::Link;
use yew_feather::search::Search;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use share::article::article_base::ArticleListItemHttp;
use share::article::article_status::ArticleStatusHttp;
use crate::css::{DASHBOARD_ARTICLE_MANAGE_CSS, DASHBOARD_MAIN_COMMON};
use crate::dashboard::article::DashboardArticleRoute;
use crate::dashboard::article::http::update_article_status;
use crate::index::http::list_article_http;

pub struct DashboardArticleManage {
    article_list: Option<Vec<ArticleListItemHttp>>,
    article_map: Option<HashMap<i64, ArticleListItemHttp>>,
}

impl DashboardArticleManage {
    fn render_article_list(&self, ctx: &Context<Self>) -> Html {
        if self.article_map.is_none() {
            html! {

            }
        } else {
            html! {
                {
                    for self.article_map.as_ref().unwrap().iter().map(|(_, article)| -> Html {
                        let id = article.id.unwrap().clone();
                        html! {
                            <tr class="article-list-row">
                                <td class="article-list-column article-id">{ id }</td>
                                <td class="article-list-column">{ article.title.clone() }</td>
                                <td class="article-list-column">{ article.outline.clone() }</td>
                                <td class="article-list-column">{"0"}</td>
                                <td class="article-list-column">{ article.create_time.unwrap().clone() }</td>
                                <td class="article-list-column">
                                    <button class="article-list-column-button article-list-column-button-update">
                                        <Link<DashboardArticleRoute> to={ DashboardArticleRoute::Update { article_id: id } }>
                                            {"修改"}
                                        </Link<DashboardArticleRoute>>
                                    </button>
                                    { self.render_status_button(ctx, article) }
                                </td>
                            </tr>
                        }
                    })
                }
            }
        }

    }

    fn render_status_button(&self, ctx: &Context<Self>, article: &ArticleListItemHttp) -> Html {
        let id = article.id.unwrap().clone();
        if article.status == 0 {
            html! {
                <button class="article-list-column-button article-list-column-button-recover"
                    onclick={ctx.link().callback(move |_| Msg::RecoverArticle(id))}>
                    {"恢复"}
                </button>
            }
        } else {
            html! {
                <button class="article-list-column-button article-list-column-button-delete"
                    onclick={ctx.link().callback(move |_| Msg::DeleteArticle(id))}>
                    {"删除"}
                </button>
            }
        }
    }

    fn update_article_status_http(&mut self, ctx: &Context<Self>, id: i64, status: i8) {
        let link = ctx.link().clone();
        spawn_local(async move {
            if let Ok(_id) = update_article_status(ArticleStatusHttp {
                id,
                status,
            }).await {

                link.send_message(Msg::UpdateArticleStatus((id, status)));
            }
        });
    }
}

pub enum Msg {
    FetchArticleListHttp(Vec<ArticleListItemHttp>),
    DeleteArticle(i64),
    RecoverArticle(i64),
    UpdateArticleStatus((i64, i8)),
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
            article_list: None,
            article_map: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchArticleListHttp(articles) => {
                self.article_map = Some(articles.into_iter().fold(HashMap::new(), |mut map, article| {
                    map.insert(article.id.unwrap(), article);

                    map
                }));

                true
            },

            Msg::DeleteArticle(id) => {
                self.update_article_status_http(ctx, id, 0);

                false
            },

            Msg::RecoverArticle(id) => {
                self.update_article_status_http(ctx, id, 1);

                false
            },

            Msg::UpdateArticleStatus((id, status)) => {
                let article_map = self.article_map.as_mut().unwrap();

                if let Some(article) = article_map.get_mut(&id) {
                    article.status = status;
                }

                true
            }
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