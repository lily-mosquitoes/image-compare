use yew::{
    function_component,
    html,
    Html,
    Properties,
};

use crate::shared_components::Modal;

#[derive(Properties, PartialEq)]
pub(super) struct ChangeUserModalProps {
    pub(super) id: String,
}

#[function_component(ChangeUserModal)]
pub(super) fn change_user_modal(
    props: &ChangeUserModalProps,
) -> Html {
    html! {
        <Modal id={props.id.clone()}>
            <p class="text-5xl">{ "content" }</p>
        </Modal>
    }
}
