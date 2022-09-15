use std::collections::{HashMap};
use itertools::Itertools;
use yew::{Component, Context, Html, html};
use yew_router::prelude::Link;
use yew_feather::search::Search;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use share::article::article_base::ArticleListItemHttp;
use share::article::article_status::ArticleStatusHttp;
use share::article::http::ListArticleOptions;
use share::utils::page::{PageRequest, Pagination};
use share::utils::status::StatusOptions;
use crate::css::{DASHBOARD_ARTICLE_MANAGE_CSS, DASHBOARD_MAIN_COMMON};
use crate::dashboard::article::DashboardArticleRoute;
use crate::dashboard::article::http::update_article_status;
use crate::index::http::list_article_http;

pub struct DashboardArticleManage {
    page: Option<Pagination<HashMap<i64, ArticleListItemHttp>>>,
}

impl DashboardArticleManage {
    fn render_article_list(&self, ctx: &Context<Self>) -> Html {
        if self.page.is_none() {
            html! {

            }
        } else {
            let article_list: Vec<&ArticleListItemHttp> = self.page.as_ref().unwrap().data.iter()
                .sorted_by_key(|(id, _article)| -1i64 * id.clone())
                .map(|(_id, article)| article)
                .collect();

            html! {
                {
                    for article_list.into_iter().map(|article| -> Html {
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

    fn render_page_transfer_bar(&self, ctx: &Context<Self>) -> Html {
        if self.page.is_none() {
            html! {

            }
        } else {
            let page = self.page.as_ref().unwrap();
            let total_page = page.total_page;
            let cur_page = page.page;

            html! {
                <div id="page-transform">
                    <button class="button-transform-left"
                        onclick={ ctx.link().callback(move |_| Msg::TransferPage(cur_page - 1)) }>
                        { "<" }
                    </button>
                    {
                        for (1..total_page + 1).into_iter().map(|i| {
                            html! {
                                <button class="button-transform-middle"
                                    onclick={ ctx.link().callback(move |_| Msg::TransferPage(i)) }>
                                    { i }
                                </button>
                            }
                        })
                    }
                    <button class="button-transform-right" onclick={ ctx.link().callback(move |_| Msg::TransferPage(cur_page + 1)) }>
                        { ">" }
                    </button>
                </div>
            }
        }
    }

    fn fetch_article_list_http(&self, ctx: &Context<Self>, page_request: PageRequest) {
        {
            let link = ctx.link().clone();
            spawn_local(async move {
                if let Ok(articles) = list_article_http(ListArticleOptions {
                    user_id: 1,
                    status: StatusOptions {
                        is_all: true,
                        status: None,
                    },
                    page: page_request,
                }).await {
                    link.send_message(Msg::FetchArticleListHttp(articles));
                }
            })
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
    FetchArticleListHttp(Pagination<Vec<ArticleListItemHttp>>),
    TransferPage(i64),
    DeleteArticle(i64),
    RecoverArticle(i64),
    UpdateArticleStatus((i64, i8)),
}

impl Component for DashboardArticleManage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let manage = DashboardArticleManage {
            page: None
        };

        let page_request = PageRequest::init(10);
        manage.fetch_article_list_http(ctx, page_request);

        manage
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchArticleListHttp(article_page) => {
                let func =
                    |articles: Vec<ArticleListItemHttp>| {
                        articles.into_iter().fold(HashMap::new(), |mut map, article| {
                            map.insert(article.id.unwrap(), article);

                            map
                        })
                    };

                let map = article_page.to_page(func);

                self.page = Some(map);

                true
            },

            Msg::TransferPage(page) => {
                let total_page = self.page.as_ref().unwrap().total_page;
                if page == 0 || page > total_page {
                    return false;
                }

                let page_request = PageRequest {
                    page,
                    page_size: 10,
                };

                self.fetch_article_list_http(ctx, page_request);

                true
            }

            Msg::DeleteArticle(id) => {
                self.update_article_status_http(ctx, id, 0);

                false
            }

            Msg::RecoverArticle(id) => {
                self.update_article_status_http(ctx, id, 1);

                false
            }

            Msg::UpdateArticleStatus((id, status)) => {
                if let Some(page) = self.page.as_mut() {
                    let article_map = &mut page.data;

                    if let Some(article) = article_map.get_mut(&id) {
                        article.status = status;
                    }
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
                    <hr />
                    { self.render_page_transfer_bar(ctx) }
                </div>
            </>
        }
    }
}