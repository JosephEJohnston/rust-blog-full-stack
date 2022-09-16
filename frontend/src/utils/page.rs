#[allow(dead_code)]

use yew::prelude::*;
use share::utils::page::{PageBar, PageRequest};

pub struct Page {

}

impl Page {
    fn render_page_transfer_bar(&self, ctx: &Context<Self>) -> Html {
        let page = ctx.props().page_bar;

        if page.is_none() {
            html! {

            }
        } else {
            let page = page.unwrap();
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
}

#[derive(PartialEq, Properties)]
pub struct PageProps {
    #[prop_or_default]
    page_bar: Option<PageBar>,
    callback: Callback<PageRequest>,
}

pub enum Msg {
    TransferPage(i64),
}

impl Component for Page {
    type Message = Msg;
    type Properties = PageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TransferPage(page) => {
                let page_bar = ctx.props().page_bar.as_ref().unwrap();

                let total_page = page_bar.total_page;
                if page == 0 || page > total_page {
                    return false;
                }

                let page_request = PageRequest {
                    page,
                    page_size: page_bar.page_size,
                };

                ctx.props().callback.emit(page_request);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            { self.render_page_transfer_bar(ctx) }
        }
    }
}

