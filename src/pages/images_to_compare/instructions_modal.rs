use yew::{
    classes,
    function_component,
    html,
    AttrValue,
    Callback,
    Html,
    Properties,
};

use crate::shared_components::Modal;

static ABOUT_THE_PROJECT_EN: &str =
    include_str!("../../markdown/about_the_project-EN.md");

#[derive(Properties, PartialEq)]
pub(super) struct InstructionsModalProps {
    pub(super) onclose: Callback<()>,
}

#[function_component(InstructionsModal)]
pub(super) fn instructions_modal(
    props: &InstructionsModalProps,
) -> Html {
    let about_the_project = markdown::to_html(ABOUT_THE_PROJECT_EN);
    let about_the_project = AttrValue::from(about_the_project);
    let about_the_project =
        Html::from_html_unchecked(about_the_project);

    html! {
        <Modal
            id={"instructions_modal"}
            onclose={props.onclose.clone()}
        >
            <div
                id={"about_the_project"}
                class={classes!["text-5xl"]}
            >
                { about_the_project }
            </div>
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

    use super::InstructionsModal;
    use crate::{
        dom::DOM,
        helpers_for_tests::wasm_sleep_in_ms,
        markdown_to_decoded_html,
        render_yew_component,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[function_component(TestInstructionsModal)]
    fn test_istructions_modal() -> Html {
        html! {
            <div>
                <InstructionsModal onclose={|_| ()} />
            </div>
        }
    }

    #[wasm_bindgen_test]
    async fn about_the_project_text_is_visible() {
        render_yew_component!(TestInstructionsModal);
        wasm_sleep_in_ms(150).await;

        let expected =
            include_str!("../../markdown/about_the_project-EN.md");
        let expected = markdown_to_decoded_html(expected);

        let text = DOM::get_element_by_id("about_the_project")
            .expect("Element #about_the_project to exist");

        assert_eq!(text.inner_html(), expected);
    }

    #[wasm_bindgen_test]
    fn how_to_participate_text_is_visible() {
        // assert!(false);
    }

    #[wasm_bindgen_test]
    fn disclaimer_text_is_visible() {
        // assert!(false);
    }
}
