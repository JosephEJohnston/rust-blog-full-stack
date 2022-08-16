use stylist::Style;
use yew::prelude::*;
use crate::css::DASHBOARD_CSS;
use crate::dashboard::side_bar::DashboardSideBar;

pub struct DashboardIndex {
    js_code: String,
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
            js_code: "feather.replace();".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardIndexMsg::RenderJS => {
                self.js_code = "feather.replace();".to_string();

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_CSS).unwrap();

        html! {
            <>
                <div id="page" class={ dashboard_css }>
                    <DashboardSideBar />
                    <div id="content" class="page-column-right">
                        <div class="page-right-header">
                            <i class="header-icon" data-feather="align-justify"></i>
                        </div>
                        <article>
                            <div class="statistics-info">
                                <div class="for-each-statistics">
                                    <div class="each-statistics-title">
                                        <div class="each-title-detail">{"用户数"}</div>
                                        <div class="each-tag-all">{"全部"}</div>
                                    </div>
                                    <div class="each-statistics-data">
                                        <i class="each-icon" data-feather="users"></i>
                                        <div class="each-data">{"12"}</div>
                                    </div>
                                </div>
                                <div class="for-each-statistics">
                                    <div class="each-statistics-title">
                                        <div class="each-title-detail">{"访问者"}</div>
                                        <div class="each-tag-all">{"全部"}</div>
                                    </div>
                                    <div class="each-statistics-data">
                                        <i class="each-icon" data-feather="eye"></i>
                                        <div class="each-data">{"4775"}</div>
                                    </div>
                                </div>
                                <div class="for-each-statistics">
                                    <div class="each-statistics-title">
                                        <div class="each-title-detail">{"文章数"}</div>
                                        <div class="each-tag-all">{"全部"}</div>
                                    </div>
                                    <div class="each-statistics-data">
                                        <i class="each-icon" data-feather="book"></i>
                                        <div class="each-data">{"20"}</div>
                                    </div>
                                </div>
                                <div class="for-each-statistics">
                                    <div class="each-statistics-title">
                                        <div class="each-title-detail">{"评论数"}</div>
                                        <div class="each-tag-all">{"全部"}</div>
                                    </div>
                                    <div class="each-statistics-data">
                                        <i class="each-icon" data-feather="message-circle"></i>
                                        <div class="each-data">{"50"}</div>
                                    </div>
                                </div>
                            </div>
                        </article>
                    </div>
                    <footer>

                    </footer>
                </div>
                <script src="feather/feather.min.js">
                </script>
                <script>
                    { "setTimeout(function() { feather.replace(); }, 100)" }
                </script>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(DashboardIndexMsg::RenderJS);
        }
    }
}