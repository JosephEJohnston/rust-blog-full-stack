use yew::prelude::*;
use yew_router::prelude::*;
use share::article::ArticleHttp;
use crate::index::IndexRoute;

pub struct ArticleListItem {

}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ArticleListItemProps {
    pub article: ArticleHttp,
}

pub enum ArticleListItemMsg {
}

impl Component for ArticleListItem {
    type Message = ArticleListItemMsg;
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
                            <button class="for-article-tag">{ "入门" }</button>
                            <button class="for-article-tag">{ "Linux" }</button>
                            <button class="for-article-tag">{ "Laravel" }</button>
                            <button class="for-article-tag">{ "Ubuntu" }</button>
                            <button class="for-article-tag">{ "PHP7" }</button>
                        </div>
                        <hr class="article-border-line" />
                        <div class="article-info">
                            <div class="for-article-info">{ "Jiajian" }</div>
                            <div class="for-article-info">{ "3年前" }</div>
                            <div class="for-article-info">{ "15820" }</div>
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