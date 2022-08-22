use yew::function_component;
use yew::prelude::*;
use crate::dashboard::article::simplemde_interop::*;

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

#[derive(Properties, Clone, PartialEq)]
pub struct EditorProps {

}

#[function_component(Editor)]
pub fn editor() -> Html {
    let load_done = use_simplemde();

    {
        let load_done = load_done.clone();
        use_effect(move || {
            if load_done {
                create_editor();
            }

            || ()
        });
    }

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
