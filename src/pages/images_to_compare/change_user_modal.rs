use yew::{
    classes,
    function_component,
    html,
    Callback,
    Html,
    Properties,
};
use yew_router::hooks::use_navigator;

use crate::{
    assets::ExclamationTriangle,
    pages::markdown_to_yew_html,
    routes::Route,
    shared_components::{
        Button,
        Modal,
    },
};

static CHANGE_USER_CONTENT_EN: &str =
    include_str!("../../markdown/change_user_content-EN.md");

static CONFIRM_RESET_USER_BUTTON_EN: &str =
    include_str!("../../markdown/confirm_reset_user_button-EN.md");

static CANCEL_ACTION_BUTTON_EN: &str =
    include_str!("../../markdown/cancel_action_button-EN.md");

#[derive(Properties, PartialEq)]
pub(super) struct ChangeUserModalProps {
    pub(super) onclose: Callback<()>,
}

#[function_component(ChangeUserModal)]
pub(super) fn change_user_modal(
    props: &ChangeUserModalProps,
) -> Html {
    let navigator = use_navigator().expect("Navitor to be present");

    let change_user_content =
        markdown_to_yew_html(CHANGE_USER_CONTENT_EN);

    let cancel_action_button =
        markdown_to_yew_html(CANCEL_ACTION_BUTTON_EN);

    let confirm_reset_user_button =
        markdown_to_yew_html(CONFIRM_RESET_USER_BUTTON_EN);

    let reset_user = Callback::from(move |_| {
        // TODO: remove cookies
        navigator.push(&Route::Root)
    });

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
                        "h-32",
                        "stroke-rose-600",
                    ]}
                />
                <span class={classes!["sr-only"]}>
                    { "Warning" }
                </span>
            </section>
            <section
                id={"change_user_warning_content"}
                class={classes![
                    "text-5xl",
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
                    "gap-8",
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
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[function_component(TestChangeUserModal)]
    fn test_change_user_modal() -> Html {
        html! {
            <div>
                <ChangeUserModal onclose={|_| ()} />
            </div>
        }
    }

    #[wasm_bindgen_test]
    async fn change_user_content_text_is_visible() {
        render_yew_component!(TestChangeUserModal);
        wasm_sleep_in_ms(50).await;

        let expected =
            include_str!("../../markdown/change_user_content-EN.md");
        let expected = markdown_to_decoded_html(expected);

        let text =
            DOM::get_element_by_id("change_user_warning_content")
                .expect(
                    "Element #change_user_warning_content to exist",
                );

        assert_eq!(text.inner_html(), expected);
    }

    #[wasm_bindgen_test]
    async fn cancel_action_button_exists() {
        render_yew_component!(TestChangeUserModal);
        wasm_sleep_in_ms(50).await;

        let expected =
            include_str!("../../markdown/cancel_action_button-EN.md");
        let expected = markdown_to_decoded_html(expected);

        assert!(DOM::has_button_with_inner_html(&expected));
    }

    #[wasm_bindgen_test]
    async fn confirm_reset_user_button_exists() {
        render_yew_component!(TestChangeUserModal);
        wasm_sleep_in_ms(50).await;

        let expected = include_str!(
            "../../markdown/confirm_reset_user_button-EN.md"
        );
        let expected = markdown_to_decoded_html(expected);

        assert!(DOM::has_button_with_inner_html(&expected));
    }
}
