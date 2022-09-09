use web_sys::{Element, HtmlElement};
use yew::prelude::*;
use crate::utils::input_ref::InputRef;

pub struct ValidateMaintain {
    pub input: InputRef,
    pub validate_result: bool,
    pub validate_msg: String,
}

impl ValidateMaintain {
    pub fn new() -> ValidateMaintain {
        ValidateMaintain {
            input: InputRef::default(),
            validate_result: true,
            validate_msg: "".to_string(),
        }
    }

    pub fn set_wrong(&mut self, msg: String) {
        self.validate_result = false;
        self.validate_msg = msg;
    }

    pub fn set_right(&mut self) {
        self.validate_result = true;
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateInputProps {
    pub input_name: String,
    pub validate_result: bool,
    pub validate_msg: String,
    pub children: Children,
}

pub struct CreateInput {
    validate_input: NodeRef,
    validate_container: NodeRef,
}

impl CreateInput {
    fn render_validate_css(&self, ctx: &Context<Self>) {
        match ctx.props().validate_result {
            false => {
                self.set_input_property("display", "block");

                self.validate_container.cast::<Element>()
                    .map(|ele| ele.class_list().add_1("for-each-input-container-wrong"))
                    .unwrap()
                    .expect("");
            },

            true => {
                self.set_input_property("display", "none");

                self.validate_container.cast::<Element>()
                    .map(|ele| ele.class_list().remove_1("for-each-input-container-wrong"))
                    .unwrap()
                    .expect("");
            }
        }
    }

    fn set_input_property(&self, property: &str, value: &str) {
        self.validate_input.cast::<HtmlElement>()
            .map(|ele| ele.style().set_property(property, value))
            .unwrap()
            .expect("");
    }
}

impl Component for CreateInput {
    type Message = ();
    type Properties = CreateInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        CreateInput {
            validate_input: NodeRef::default(),
            validate_container: NodeRef::default()
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.render_validate_css(ctx);

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="for-each-input-container"
                    ref={self.validate_container.clone()}>
                    <div class="input-validate-notice"
                        ref={self.validate_input.clone()}>
                        {ctx.props().validate_msg.clone()}
                    </div>
                    <div class="input-name-container">
                        <div class="input-name">{ctx.props().input_name.clone()}</div>
                    </div>
                    { ctx.props().children.clone() }
                </div>
            </>
        }
    }
}