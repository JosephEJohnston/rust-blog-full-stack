use yew::{Component, Context, Html, html};

pub struct Article {

}

impl Component for Article {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Article {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div id="content-banner" class="article-banner">
                    <div class="banner-title">{ "Ubuntu 16.04 环境安装部署" }</div>
                    <p class="banner-brief">{ "项目部署文档" }</p>
                    <div>
                        <button class="banner-tag">{ "入门" }</button>
                        <button class="banner-tag">{ "Linux" }</button>
                        <button class="banner-tag">{ "Laravel" }</button>
                        <button class="banner-tag">{ "Ubuntu" }</button>
                        <button class="banner-tag">{ "PHP7" }</button>
                    </div>
                </div>
                <article>
                    <div id="user-article">
                        <article class="markdown-body">
                            { "我是文章内容" }
                        </article>
                    </div>
                </article>
            </>
        }
    }
}