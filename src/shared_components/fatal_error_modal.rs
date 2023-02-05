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
pub(crate) struct FatalErrorModalProps {
    pub(crate) onclose: Callback<()>,
}

#[function_component(FatalErrorModal)]
pub(crate) fn fatal_error_modal(
    props: &FatalErrorModalProps,
) -> Html {
    html! {
        <Modal
            id={"fatal_error_modal"}
            onclose={props.onclose.clone()}
        >
            <p class={classes!["text-5xl"]}>
                { "An error has occurred, we apologize for the inconvenience." }
            </p>
            <p class={classes!["text-5xl"]}>
                { "Please try again later." }
            </p>
        </Modal>
    }
}
