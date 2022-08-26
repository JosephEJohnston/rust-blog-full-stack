use yew::prelude::*;
use yew_router::prelude::*;
use share::article::article_base::ArticleListItemHttp;
use crate::index::IndexRoute;

pub struct ArticleListItem {

}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ArticleListItemProps {
    // 没有 content
    pub article: ArticleListItemHttp,
}

impl ArticleListItem {

}

impl Component for ArticleListItem {
    type Message = ();
    type Properties = ArticleListItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ArticleListItem {

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <>
                <div class="for-article">
                    <img class="article-img" src="/static/resource/img/article-img.jpg" alt="" />
                    <div class="article-text">
                        <button class="article-title">{ ctx.props().article.title.clone() }</button>
                        <p class="article-brief">{ ctx.props().article.outline.clone() }</p>
                        <hr class="article-border-line" />
                        <div class="article-tag">
                            {
                                for ctx.props().article.tag_list.as_ref().unwrap().iter().map(|tag| {

                                    html! {
                                        <button class="for-article-tag">{ tag.name.clone() }</button>
                                    }
                                })
                            }
                        </div>
                        <hr class="article-border-line" />
                        <div class="article-info">
                            <div class="for-article-info">{ ctx.props().article.user.as_ref().unwrap().name.clone() }</div>
                            <div class="for-article-info">{ ctx.props().article.create_time.unwrap().date() }</div>
                            // <div class="for-article-info">{ "三年前" }</div>
                            <div class="for-article-info">{ ctx.props().article.statistics.as_ref().unwrap().read_count }</div>
                        </div>
                        <button class="article-detail-button">
                            <Link<IndexRoute> to={ IndexRoute::Article }>
                                { "Read More >" }
                            </Link<IndexRoute>>
                        </button>
                    </div>
                </div>
                <hr class="article-bottom-line" />
            </>
        }
    }
}