use yew::function_component;
use yew::prelude::*;
use crate::dashboard::article::create_interop::*;

#[function_component(ForEditor)]
pub fn for_editor() -> Html {

    html! {
        <>
            <ResourceProvider>
                <Editor />
            </ResourceProvider>
        </>
    }
}

#[function_component(Editor)]
pub fn editor() -> Html {
    let load_done = use_simplemde();

    html! {
        if load_done {
            <div class="editor-inner-container">
                <textarea id="editor" ></textarea>
            </div>
        } else {
            <label>
                <input class="each-input" type="text" placeholder=""/>
            </label>
        }
    }
}
