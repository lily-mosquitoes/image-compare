use yew::{
    classes,
    function_component,
    html,
    Callback,
    Html,
    Properties,
};

use crate::{
    request::Image,
    shared_components::{
        Button,
        Loading,
    },
};

#[derive(Properties, PartialEq)]
pub(super) struct ImageListProps {
    pub(super) loading: bool,
    pub(super) images: Vec<Image>,
    pub(super) onclick: Callback<Image>,
}

#[function_component(ImageList)]
pub(super) fn image_list(props: &ImageListProps) -> Html {
    let onclick = props.onclick.clone();

    props
        .images
        .iter()
        .map(|image| {
            let on_image_select = {
                let onclick = onclick.clone();
                let image = image.clone();
                Callback::from(move |_| onclick.emit(image.clone()))
            };

            if props.loading {
                html! {
                    <Button
                        id={"loading_status_button".to_string()}
                        class={classes![
                            "m-10",
                        ]}
                        disabled=true
                    >
                        <Loading />
                    </Button>
                }
            } else {
                html! {
                    <Button
                        id={"image_to_compare_button"}
                        class={classes![
                            "m-10",
                        ]}
                        onclick={on_image_select}
                    >
                        <img
                            id={"image_to_compare"}
                            class={classes!["w-full", "object-contain"]}
                            src={image.src.clone()}
                            alt=""
                        />
                    </Button>
                }
            }
        })
        .collect::<Html>()
}
