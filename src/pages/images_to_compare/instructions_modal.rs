use yew::{
    classes,
    function_component,
    html,
    Callback,
    Html,
    Properties,
};

use crate::shared_components::Modal;

#[derive(Properties, PartialEq)]
pub(super) struct InstructionsModalProps {
    pub(super) onclose: Callback<()>,
}

#[function_component(InstructionsModal)]
pub(super) fn instructions_modal(
    props: &InstructionsModalProps,
) -> Html {
    html! {
        <Modal
            id={"instructions_modal"}
            onclose={props.onclose.clone()}
        >
            <p class={classes!["text-5xl"]}>{ "content" }</p>
        </Modal>
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::InstructionsModal;
    use crate::{
        dom::DOM,
        render_yew_component,
        wasm_sleep,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn todo() {
        // assert!(false);
    }
}
