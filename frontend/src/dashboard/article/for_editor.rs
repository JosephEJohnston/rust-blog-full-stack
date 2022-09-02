use yew::function_component;
use yew::prelude::*;
use crate::dashboard::article::simplemde_interop::*;

static mut INIT: bool = false;

#[derive(Properties, Clone, PartialEq)]
pub struct ForEditorProps {
    pub editor_callback: Callback<SimpleMDE>,
}

#[function_component(ForEditor)]
pub fn for_editor(props: &ForEditorProps) -> Html {

    html! {
        <>
            <ResourceProvider>
                <Editor editor_callback={props.editor_callback.clone()}/>
            </ResourceProvider>
        </>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct EditorProps {
    pub editor_callback: Callback<SimpleMDE>,
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {

    let load_done = unsafe {
        !INIT && use_simplemde()
    };

    {
        let callback = props.editor_callback.clone();
        let load_done = load_done.clone();
        use_effect(move || {
            if load_done {
                create_editor();
                unsafe {
                    INIT = true;
                }
                // callback.emit(create_editor());
            }

            || ()
        });
    }

    html! {
        if load_done {
            <div class="editor-inner-container">
                <textarea id="editor"></textarea>
            </div>
        } else {
            <label>
                <input class="each-input" type="text" placeholder=""/>
            </label>
        }
    }
}
