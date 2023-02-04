use yew::{
    classes,
    function_component,
    html,
    Callback,
    Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub(crate) struct ButtonProps {
    pub(crate) text: String,
    pub(crate) onclick: Callback<()>,
}

#[function_component(Button)]
pub(crate) fn button(props: &ButtonProps) -> Html {
    let onclick = {
        let event = props.onclick.clone();
        Callback::from(move |_| event.emit(()))
    };

    html! {
        <button
            class={classes!["text-5xl", "drop-shadow-2xl"]}
            onclick={onclick}
        >
            { &props.text }
        </button>
    }
}
