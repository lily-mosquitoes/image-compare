use yew::{
    function_component,
    html,
    Classes,
    Html,
    Properties,
};

#[derive(Clone, PartialEq, Properties)]
pub(crate) struct XMarkProps {
    #[prop_or_default]
    pub(crate) class: Classes,
}

#[function_component(XMark)]
pub(crate) fn x_mark(props: &XMarkProps) -> Html {
    html! {
        <svg
            aria-hidden="true"
            class={props.class.clone()}
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M6 18L18 6M6 6l12 12"
            />
        </svg>
    }
}
