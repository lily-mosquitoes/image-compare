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
    assets::ExclamationTriangle,
    dom::{
        console_error,
        DOM,
    },
    pages::markdown_to_yew_html,
    shared_components::{
        Button,
        Modal,
    },
    Language,
};

#[derive(Properties, PartialEq)]
pub(super) struct ChangeUserModalProps {
    pub(super) onclose: Callback<()>,
    pub(super) onconfirm: Callback<()>,
}

#[function_component(ChangeUserModal)]
pub(super) fn change_user_modal(props: &ChangeUserModalProps) -> Html {
    let language = match use_context::<UseReducerHandle<Language>>() {
        Some(ctx) => (*ctx).clone(),
        None => Language::default(),
    };

    let change_user_content = language.load_file("change_user_content.md");
    let change_user_content =
        markdown_to_yew_html(change_user_content.unwrap_or(""));

    let cancel_action_button = language.load_file("cancel_action_button.md");
    let cancel_action_button =
        markdown_to_yew_html(cancel_action_button.unwrap_or(""));

    let confirm_reset_user_button =
        language.load_file("confirm_reset_user_button.md");
    let confirm_reset_user_button =
        markdown_to_yew_html(confirm_reset_user_button.unwrap_or(""));

    let reset_user = {
        let close_event = props.onclose.clone();
        let confirmation_event = props.onconfirm.clone();
        Callback::from(move |_| {
            let deleted = DOM::local_storage()
                .ok_or("Unable to fetch localstorage")
                .and_then(|storage| {
                    storage.delete("user_id").map_err(|_| {
                        "Unable to delete `user_id` from localstorage"
                    })
                });
            if let Err(error) = deleted {
                console_error!(error);
            };

            close_event.emit(());
            confirmation_event.emit(());
        })
    };

    html! {
        <Modal
            id={"change_user_modal"}
            onclose={props.onclose.clone()}
        >
            <section
                id={"change_user_warning_title"}
                class={classes!["self-center"]}
            >
                <ExclamationTriangle
                    class={classes![
                        "h-16",
                        "stroke-rose-600",
                    ]}
                />
            </section>
            <section
                id={"change_user_warning_content"}
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
                { change_user_content }
            </section>
            <section
                id={"change_user_warning_buttons"}
                class={classes![
                    "pb-8",
                    "self-center",
                    "flex",
                    "flex-row",
                    "gap-2",
                    "md:gap-4",
                ]}
            >
                <Button
                    id={"change_user_cancel_button"}
                    class={classes![
                        "text-gray-600",
                        "border-2",
                        "border-gray-600",
                    ]}
                    onclick={props.onclose.clone()}
                >
                    { cancel_action_button }
                </Button>
                <Button
                    id={"change_user_confirm_button"}
                    class={classes![
                        "text-rose-600",
                        "border-2",
                        "border-rose-600",
                    ]}
                    onclick={reset_user}
                >
                    { confirm_reset_user_button }
                </Button>
            </section>
        </Modal>
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };
    use yew::{
        function_component,
        html,
        Html,
    };

    use super::ChangeUserModal;
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

    #[function_component(TestChangeUserModal)]
    fn test_change_user_modal() -> Html {
        html! {
            <div>
                <ChangeUserModal
                    onclose={|_| ()}
                    onconfirm={|_| ()}
                />
            </div>
        }
    }

    #[wasm_bindgen_test]
    fn change_user_content_markdown_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            let language = Language {
                index: language_index,
            };
            let file = language.load_file("change_user_content.md");

            assert!(file.is_some())
        }
    }

    #[wasm_bindgen_test]
    async fn change_user_content_text_is_visible() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(TestChangeUserModal);
            wasm_sleep_in_ms(50).await;

            let language = Language::default();
            let expected = language.load_file("change_user_content.md");
            let expected = markdown_to_decoded_html(expected.unwrap_or(""));

            let text = DOM::get_element_by_id("change_user_warning_content")
                .expect("Element #change_user_warning_content to exist");

            assert_eq!(text.inner_html(), expected);
        }
    }

    #[wasm_bindgen_test]
    fn cancel_action_button_markdown_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            let language = Language {
                index: language_index,
            };
            let file = language.load_file("cancel_action_button.md");

            assert!(file.is_some())
        }
    }

    #[wasm_bindgen_test]
    async fn cancel_action_button_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(TestChangeUserModal);
            wasm_sleep_in_ms(50).await;

            let language = Language::default();
            let expected = language.load_file("cancel_action_button.md");
            let expected = markdown_to_decoded_html(expected.unwrap_or(""));

            assert!(DOM::has_button_with_inner_html(&expected));
        }
    }

    #[wasm_bindgen_test]
    fn confirm_reset_user_button_markdown_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            let language = Language {
                index: language_index,
            };
            let file = language.load_file("confirm_reset_user_button.md");

            assert!(file.is_some())
        }
    }

    #[wasm_bindgen_test]
    async fn confirm_reset_user_button_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(TestChangeUserModal);
            wasm_sleep_in_ms(50).await;

            let language = Language::default();
            let expected = language.load_file("confirm_reset_user_button.md");
            let expected = markdown_to_decoded_html(expected.unwrap_or(""));

            assert!(DOM::has_button_with_inner_html(&expected));
        }
    }

    #[wasm_bindgen_test]
    async fn confirm_reset_user_button_removes_user_id_from_localstorage() {
        render_yew_component!(TestChangeUserModal);
        wasm_sleep_in_ms(50).await;

        DOM::local_storage()
            .expect("Localstorage to exist")
            .set_item("user_id", "123456")
            .expect("Localstorage to be settable");

        let button = DOM::get_button_by_id("change_user_confirm_button")
            .expect("Element #change_user_confirm_button to be present")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement");

        button.click();
        wasm_sleep_in_ms(50).await; // allow page to re-render

        let user_id = DOM::local_storage()
            .expect("Localstorage to exist")
            .get_item("user_id")
            .expect("Localstorage to be gettable");
        assert_eq!(user_id, None);
    }
}
