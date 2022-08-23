use yew::function_component;
use yew::prelude::*;
use crate::dashboard::article::showdown_interop::*;

#[function_component(ForMarkdownConventer)]
pub fn for_markdown_converter() -> Html {

    html! {
        <>
            <ResourceProvider>
                <MarkdownConverter />
            </ResourceProvider>
        </>
    }
}

#[function_component(MarkdownConverter)]
pub fn markdown_converter() -> Html {
    let load_done = use_showdown();

    {
        let load_done = load_done.clone();
        use_effect(move || {
            if load_done {
                // let converter = showdown.Converter();
                // let html = converter.makeHtml("hello world".to_string());
                // log!(format!("{:?}", html));
            }

            || ()
        });
    }

    html! {

    }
}