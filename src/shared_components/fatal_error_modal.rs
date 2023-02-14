use std::path::PathBuf;

use yew::{
    classes,
    function_component,
    html,
    use_context,
    Callback,
    Html,
    Properties,
};

use crate::{
    load_file_from_language,
    pages::markdown_to_yew_html,
    shared_components::Modal,
    Language,
    LanguageContext,
};

#[derive(Properties, PartialEq)]
pub(crate) struct FatalErrorModalProps {
    pub(crate) onclose: Callback<()>,
}

#[function_component(FatalErrorModal)]
pub(crate) fn fatal_error_modal(
    props: &FatalErrorModalProps,
) -> Html {
    let language = match use_context::<LanguageContext>() {
        Some(ctx) => (*ctx).clone(),
        None => Language::default(),
    };

    let fatal_error_message = load_file_from_language(
        PathBuf::from("fatal_error_message.md"),
        language.index,
    );
    let fatal_error_message =
        markdown_to_yew_html(fatal_error_message.unwrap_or(""));

    html! {
        <Modal
            id={"fatal_error_modal"}
            onclose={props.onclose.clone()}
        >
            <section
                id={"fatal_error_message"}
                class={classes!["text-5xl"]}
            >
                { fatal_error_message }
            </section>
        </Modal>
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::PathBuf,
        sync::atomic::Ordering,
    };

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
        load_file_from_language,
        markdown_to_decoded_html,
        render_yew_component,
        wasm_sleep_in_ms,
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
            let file = load_file_from_language(
                PathBuf::from("fatal_error_message.md"),
                language_index,
            );

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

            let expected = load_file_from_language(
                PathBuf::from("fatal_error_message.md"),
                language_index,
            );
            let expected =
                markdown_to_decoded_html(expected.unwrap_or(""));

            let text = DOM::get_element_by_id("fatal_error_message")
                .expect("Element #fatal_error_message to exist");

            assert_eq!(text.inner_html(), expected);
        }
    }
}
