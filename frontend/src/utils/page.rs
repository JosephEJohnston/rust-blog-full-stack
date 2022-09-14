use yew::prelude::*;
use share::utils::page::Pagination;

pub struct Page {
    // page: Pagination,
}

#[derive(Copy, Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
    page_size: i32,
}

impl Component for Page {
    type Message = ();
    type Properties = PageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            // page: Pagination::init(ctx.props().page_size),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="page-transform">
                <button class="button-transform-left">{"<"}</button>
                <button class="button-transform-middle">{"1"}</button>
                <button class="button-transform-middle">{"2"}</button>
                <button class="button-transform-middle">{"3"}</button>
                <button class="button-transform-right">{">"}</button>
            </div>
        }
    }
}

