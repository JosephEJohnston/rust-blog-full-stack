use yew::{Component, Context, Html, html};

pub struct IndexFooter {

}

impl Component for IndexFooter {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        IndexFooter {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <footer id="footer">
                    <nav>
                        <button class="footer-button"></button>
                        <button class="footer-button"></button>
                        <button class="footer-button"></button>
                    </nav>
                </footer>
            </>
        }
    }
}