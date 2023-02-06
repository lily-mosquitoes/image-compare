use yew::{
    classes,
    function_component,
    html,
    Children,
    Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub(crate) struct FooterProps {
    pub(crate) children: Children,
}

#[function_component(Footer)]
pub(crate) fn footer(props: &FooterProps) -> Html {
    html! {
        <footer
            id="footer"
            class={classes![
                "bg-stone-800",
                "p-12",
                "shrink-0",
                "flex",
                "flex-row",
                "justify-between",
            ]}
        >
            {for props.children.iter()}
        </footer>
    }
}
