use yew::{
    classes,
    function_component,
    html,
    use_context,
    Callback,
    Html,
    Properties,
    UseReducerHandle,
};

use crate::{
    pages::markdown_to_yew_html,
    shared_components::Modal,
    Language,
};

#[derive(Properties, PartialEq)]
pub(crate) struct FatalErrorModalProps {
    pub(crate) onclose: Callback<()>,
}

#[function_component(FatalErrorModal)]
pub(crate) fn fatal_error_modal(props: &FatalErrorModalProps) -> Html {
    let language = match use_context::<UseReducerHandle<Language>>() {
        Some(ctx) => (*ctx).clone(),
        None => Language::default(),
    };

    let fatal_error_message = language.load_file("fatal_error_message.md");
    let fatal_error_message =
        markdown_to_yew_html(fatal_error_message.unwrap_or(""));

    html! {
        <Modal
            id={"fatal_error_modal"}
            onclose={props.onclose.clone()}
        >
            <section
                id={"fatal_error_message"}
                class={classes![
                    "py-8",
                    "flex",
                    "flex-col",
                    "gap-4",
                    "text-xl",
                    "text-center",
                ]}
            >
                { fatal_error_message }
            </section>
        </Modal>
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };
    use yew::{
        function_component,
        html,
        Html,
    };

    use super::FatalErrorModal;
    use crate::{
        dom::DOM,
        markdown_to_decoded_html,
        render_yew_component,
        wasm_sleep_in_ms,
        Language,
        AVAILABLE_LANGUAGES,
        DEFAULT_LANGUAGE,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[function_component(TestFatalErrorModal)]
    fn test_fatal_error_modal() -> Html {
        html! {
            <div>
                <FatalErrorModal onclose={|_| ()} />
            </div>
        }
    }

    #[wasm_bindgen_test]
    fn fatal_error_message_markdown_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            let language = Language {
                index: language_index,
            };
            let file = language.load_file("fatal_error_message.md");

            assert!(file.is_some())
        }
    }

    #[wasm_bindgen_test]
    async fn fatal_error_message_is_visible() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(TestFatalErrorModal);
            wasm_sleep_in_ms(50).await;

            let language = Language::default();
            let expected = language.load_file("fatal_error_message.md");
            let expected = markdown_to_decoded_html(expected.unwrap_or(""));

            let text = DOM::get_element_by_id("fatal_error_message")
                .expect("Element #fatal_error_message to exist");

            assert_eq!(text.inner_html(), expected);
        }
    }
}
