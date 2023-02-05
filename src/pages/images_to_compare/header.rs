use yew::{
    classes,
    function_component,
    html,
    use_state_eq,
    Callback,
    Html,
    Properties,
};

use super::change_user_modal::ChangeUserModal;
use crate::{
    request::User,
    shared_components::Button,
};

#[derive(Properties, PartialEq)]
pub(super) struct HeaderProps {
    pub(super) user: User,
}

#[function_component(Header)]
pub(super) fn header(props: &HeaderProps) -> Html {
    let change_user_modal_id = "change_user_modal".to_string();
    let show_change_user_modal = use_state_eq(|| false);

    let open_change_user_modal = {
        let show_change_user_modal = show_change_user_modal.clone();
        Callback::from(move |_| show_change_user_modal.set(true))
    };

    let close_change_user_modal = {
        let show_change_user_modal = show_change_user_modal.clone();
        Callback::from(move |_| show_change_user_modal.set(false))
    };

    let votes_count_text =
        format!("I'm done with {} votes!", props.user.votes.clone());

    html! {
        <section
            id="header"
            class={classes![
                "bg-stone-200",
                "h-28",
                "drop-shadow-2xl",
                "px-10",
                "flex",
                "flex-row",
                "justify-between",
            ]}
        >
            <Button
                text={ votes_count_text.clone() }
                onclick={Callback::from(move |_| ())}
            />
            <Button
                text={ "Change user" }
                onclick={open_change_user_modal}
            />
            if *show_change_user_modal {
                <ChangeUserModal
                    id={change_user_modal_id}
                    onclose={close_change_user_modal}
                />
            }
        </section>
    }
}
