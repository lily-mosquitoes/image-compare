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
pub(super) struct ChangeUserModalProps {
    pub(super) onclose: Callback<()>,
}

#[function_component(ChangeUserModal)]
pub(super) fn change_user_modal(
    props: &ChangeUserModalProps,
) -> Html {
    html! {
        <Modal
            id={"change_user_modal"}
            onclose={props.onclose.clone()}
        >
            <p class={classes!["text-5xl"]}>{ "content" }</p>
        </Modal>
    }
}
