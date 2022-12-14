use yew::prelude::*;
use yew_feather::users::Users;
use yew_feather::eye::Eye;
use yew_feather::book::Book;
use yew_feather::message_circle::MessageCircle;

pub struct DashboardIndex {
}

#[derive(PartialEq, Debug, Clone)]
pub enum DashboardIndexMsg {
    RenderJS,
}

impl Component for DashboardIndex {
    type Message = DashboardIndexMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardIndex {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardIndexMsg::RenderJS => {
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="statistics-info">
                    <div class="for-each-statistics">
                        <div class="each-statistics-title">
                            <div class="each-title-detail">{"用户数"}</div>
                            <div class="each-tag-all">{"全部"}</div>
                        </div>
                        <div class="each-statistics-data">
                            <Users class="each-icon" />
                            <div class="each-data">{"12"}</div>
                        </div>
                    </div>
                    <div class="for-each-statistics">
                        <div class="each-statistics-title">
                            <div class="each-title-detail">{"访问者"}</div>
                            <div class="each-tag-all">{"全部"}</div>
                        </div>
                        <div class="each-statistics-data">
                            <Eye class="each-icon" />
                            <div class="each-data">{"4775"}</div>
                        </div>
                    </div>
                    <div class="for-each-statistics">
                        <div class="each-statistics-title">
                            <div class="each-title-detail">{"文章数"}</div>
                            <div class="each-tag-all">{"全部"}</div>
                        </div>
                        <div class="each-statistics-data">
                            <Book class="each-icon" />
                            <div class="each-data">{"20"}</div>
                        </div>
                    </div>
                    <div class="for-each-statistics">
                        <div class="each-statistics-title">
                            <div class="each-title-detail">{"评论数"}</div>
                            <div class="each-tag-all">{"全部"}</div>
                        </div>
                        <div class="each-statistics-data">
                            <MessageCircle class="each-icon" />
                            <div class="each-data">{"50"}</div>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(DashboardIndexMsg::RenderJS);
        }
    }
}