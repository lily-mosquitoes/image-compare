use yew::{
    classes,
    function_component,
    html,
    Callback,
    Children,
    Classes,
    Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub(crate) struct ButtonProps {
    pub(crate) id: String,
    #[prop_or_default]
    pub(crate) class: Classes,
    #[prop_or_default]
    pub(crate) onclick: Callback<()>,
    pub(crate) children: Children,
    #[prop_or_default]
    pub(crate) disabled: bool,
}

#[function_component(Button)]
pub(crate) fn button(props: &ButtonProps) -> Html {
    let onclick = {
        let event = props.onclick.clone();
        Callback::from(move |_| event.emit(()))
    };

    html! {
        <button
            id={props.id.clone()}
            class={classes![
                "text-5xl",
                "drop-shadow-2xl",
                props.class.clone(),
            ]}
            onclick={onclick}
            disabled={props.disabled.clone()}
        >
            { for props.children.iter() }
        </button>
    }
}
