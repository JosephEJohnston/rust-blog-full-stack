use yew::function_component;
use yew::prelude::*;
use crate::dashboard::article::create_interop::*;

#[function_component(ForEditor)]
pub fn for_editor() -> Html {

    html! {
        <>
            <ResourceProvider>
                <Test/>
            </ResourceProvider>
        </>
    }
}

#[function_component(Test)]
pub fn test() -> Html {
    let try_to_get = use_simplemde();

    html! {
        if try_to_get {
            <div>{ "success" }</div>
        } else {
            <div>{ "failure" }</div>
        }
    }
}