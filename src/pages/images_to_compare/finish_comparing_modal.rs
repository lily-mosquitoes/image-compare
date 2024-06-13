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
    assets::CheckBadge,
    pages::markdown_to_yew_html,
    request::User,
    shared_components::Modal,
    Language,
};

#[derive(Properties, PartialEq)]
pub(super) struct FinishComparingModalProps {
    pub(super) user: User,
    pub(super) onclose: Callback<()>,
}

#[function_component(FinishComparingModal)]
pub(super) fn finish_comparing_modal(
    props: &FinishComparingModalProps,
) -> Html {
    let language = match use_context::<UseReducerHandle<Language>>() {
        Some(ctx) => (*ctx).clone(),
        None => Language::default(),
    };

    let thanks_for_comparing = language.load_file("thanks_for_comparing.md");
    let thanks_for_comparing = thanks_for_comparing
        .unwrap_or("")
        .replace("{lambda}", &props.user.average_lambda.to_string());
    let thanks_for_comparing = markdown_to_yew_html(&thanks_for_comparing);

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
                        "h-16",
                        "stroke-emerald-600",
                    ]}
                />
            </section>
            <section
                id={"thanks_for_comparing"}
                class={classes![
                    "flex",
                    "flex-col",
                    "gap-4",
                    "text-xl",
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

    use super::FinishComparingModal;
    use crate::{
        dom::DOM,
        markdown_to_decoded_html,
        render_yew_component,
        request::User,
        wasm_sleep_in_ms,
        Language,
        AVAILABLE_LANGUAGES,
        DEFAULT_LANGUAGE,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    fn test_user() -> User {
        User {
            id: "".to_string(),
            votes: 0,
            average_lambda: 0.8932,
        }
    }

    #[function_component(TestFinishComparingModal)]
    fn test_change_user_modal() -> Html {
        html! {
            <div>
                <FinishComparingModal
                    user={test_user()}
                    onclose={|_| ()}
                />
            </div>
        }
    }

    #[wasm_bindgen_test]
    fn thanks_for_comparing_markdown_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            let language = Language {
                index: language_index,
            };
            let file = language.load_file("thanks_for_comparing.md");

            assert!(file.is_some())
        }
    }

    #[wasm_bindgen_test]
    async fn thanks_for_comparing_text_is_visible_and_shows_lambda() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(TestFinishComparingModal);
            wasm_sleep_in_ms(50).await;

            let language = Language::default();
            let expected = language.load_file("thanks_for_comparing.md");
            let expected = expected
                .unwrap_or("")
                .replace("{lambda}", &test_user().average_lambda.to_string());
            let expected = markdown_to_decoded_html(&expected);

            let text = DOM::get_element_by_id("thanks_for_comparing")
                .expect("Element #thanks_for_comparing to exist");

            assert_eq!(text.inner_html(), expected);
        }
    }
}
