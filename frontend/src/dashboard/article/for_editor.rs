use yew::prelude::*;
use crate::dashboard::article::simplemde::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ForEditorProps {
    pub editor_callback: Callback<SimpleMDE>,
}

pub struct ForEditor {

}

pub enum Msg {
    JsOnload,
}

impl Component for ForEditor {
    type Message = Msg;
    type Properties = ForEditorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ForEditor {

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::JsOnload => {
                ctx.props().editor_callback.emit(create_editor());

                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let script_src = "/static/js/simplemde/simplemde.min.js";
        let css_src = "/static/js/simplemde/simplemde.min.css";

        html! {
            <>
                <div class="editor-inner-container">
                    <textarea id="editor"></textarea>
                </div>
                <script src={script_src} type="text/javascript" onload={ ctx.link().callback(|_| Msg::JsOnload) } />
                <link href={css_src} rel="stylesheet" type="text/css"/>
            </>
        }
    }
}