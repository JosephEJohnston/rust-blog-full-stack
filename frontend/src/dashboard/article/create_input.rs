use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

pub struct ValidateMaintain {
    pub input: NodeRef,
    pub validate_result: Rc<RefCell<bool>>,
    pub validate_msg: Rc<RefCell<String>>,
}

impl ValidateMaintain {
    pub fn new() -> ValidateMaintain {
        ValidateMaintain {
            input: NodeRef::default(),
            validate_result: Rc::new(RefCell::new(true)),
            validate_msg: Rc::new(RefCell::new("".to_string())),
        }
    }

    pub fn set_wrong(&mut self, msg: String) {
        let mut ref_result = self.validate_result.borrow_mut();
        *ref_result = false;

        let mut ref_msg = self.validate_msg.borrow_mut();
        *ref_msg = msg;
    }

    pub fn set_right(&mut self) {
        let mut ref_mut = self.validate_result.borrow_mut();
        *ref_mut = true;
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateInputProps {
    pub input_name: String,
    pub validate_result: Rc<RefCell<bool>>,
    pub validate_msg: Rc<RefCell<String>>,
    pub children: Children,
}

pub struct CreateInput {
    validate_input: NodeRef,
    validate_container: NodeRef,
}

impl CreateInput {
    fn render_validate(&self, ctx: &Context<Self>) -> Html {
        match *ctx.props().validate_result.borrow() {
            true => {
                html! {

                }
            },

            false => {
                html! {
                    <div class="input-validate-notice"
                        ref={self.validate_input.clone()}>
                        {(*ctx.props().validate_msg.borrow()).clone()}
                    </div>
                }
            }
        }
    }

    fn render_container_class(&self, ctx: &Context<Self>) {
        match *ctx.props().validate_result.borrow() {
            true => {
                self.set_input_property("display", "block");

                self.validate_container.cast::<Element>()
                    .map(|ele| ele.class_list().add_1("for-each-input-container-wrong"))
                    .unwrap()
                    .expect("");
            },

            false => {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="for-each-input-container"
                    ref={self.validate_container.clone()}>
                    { self.render_validate(ctx) }
                    <div class="input-name-container">
                        <div class="input-name">{ctx.props().input_name.clone()}</div>
                    </div>
                    { ctx.props().children.clone() }
                </div>
            </>
        }
    }
}