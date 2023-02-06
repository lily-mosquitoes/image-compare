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
                "drop-shadow-2xl",
                "p-10",
                "flex",
                "flex-row",
                "justify-between",
            ]}
        >
            <Button
                id={"finish_comparing_button"}
                onclick={Callback::from(move |_| ())}
            >
                { votes_count_text.clone() }
            </Button>
            <Button
                id={"chage_user_button"}
                onclick={open_change_user_modal}
            >
                { "Reset user" }
            </Button>
            if *show_change_user_modal {
                <ChangeUserModal
                    onclose={close_change_user_modal}
                />
            }
        </section>
    }
}
