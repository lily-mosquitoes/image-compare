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
    assets::CheckBadge,
    load_file_from_language,
    pages::markdown_to_yew_html,
    shared_components::Modal,
    Language,
    LanguageContext,
};

#[derive(Properties, PartialEq)]
pub(super) struct FinishComparingModalProps {
    pub(super) onclose: Callback<()>,
}

#[function_component(FinishComparingModal)]
pub(super) fn finish_comparing_modal(
    props: &FinishComparingModalProps,
) -> Html {
    let language = match use_context::<LanguageContext>() {
        Some(ctx) => (*ctx).clone(),
        None => Language::default(),
    };

    let thanks_for_comparing = load_file_from_language(
        PathBuf::from("thanks_for_comparing.md"),
        language.index,
    );
    let thanks_for_comparing =
        markdown_to_yew_html(thanks_for_comparing.unwrap_or(""));

    html! {
        <Modal
            id={"finish_comparing_modal"}
            onclose={props.onclose.clone()}
        >
            <section
                id={"thanks_for_comparing_title"}
                class={classes!["self-center"]}
            >
                <CheckBadge
                    class={classes![
                        "h-32",
                        "stroke-emerald-600",
                    ]}
                />
            </section>
            <section
                id={"thanks_for_comparing"}
                class={classes![
                    "flex",
                    "flex-col",
                    "gap-8",
                    "text-5xl",
                    "leading-normal",
                    "text-center",
                    "my-8",
                ]}
            >
                { thanks_for_comparing }
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

    use super::FinishComparingModal;
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

    #[function_component(TestFinishComparingModal)]
    fn test_change_user_modal() -> Html {
        html! {
            <div>
                <FinishComparingModal onclose={|_| ()} />
            </div>
        }
    }

    #[wasm_bindgen_test]
    fn thanks_for_comparing_markdown_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            let file = load_file_from_language(
                PathBuf::from("thanks_for_comparing.md"),
                language_index,
            );

            assert!(file.is_some())
        }
    }

    #[wasm_bindgen_test]
    async fn thanks_for_comparing_text_is_visible() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(TestFinishComparingModal);
            wasm_sleep_in_ms(50).await;

            let expected = load_file_from_language(
                PathBuf::from("thanks_for_comparing.md"),
                language_index,
            );
            let expected =
                markdown_to_decoded_html(expected.unwrap_or(""));

            let text = DOM::get_element_by_id("thanks_for_comparing")
                .expect("Element #thanks_for_comparing to exist");

            assert_eq!(text.inner_html(), expected);
        }
    }
}
