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