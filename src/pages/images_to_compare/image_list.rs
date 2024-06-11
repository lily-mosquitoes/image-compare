use yew::{
    classes,
    function_component,
    html,
    Callback,
    Html,
    Properties,
};

use crate::shared_components::{
    Button,
    Loading,
};

#[derive(Properties, PartialEq)]
pub(super) struct ImageListProps {
    pub(super) loading: bool,
    pub(super) images: Vec<String>,
    pub(super) onclick: Callback<String>,
}

#[function_component(ImageList)]
pub(super) fn image_list(props: &ImageListProps) -> Html {
    let onclick = props.onclick.clone();

    props
        .images
        .iter()
        .enumerate()
        .map(|(index, image)| {
            let on_image_select = {
                let onclick = onclick.clone();
                let image = image.clone();
                Callback::from(move |_| onclick.emit(image.clone()))
            };

            if props.loading {
                html! {
                    <Button
                        id={format!("loading_status_button_{index}")}
                        class={classes![
                            "h-1/2",
                            "md:h-5/6",
                            "aspect-square",
                            "w-fit",
                        ]}
                        disabled=true
                    >
                        <Loading />
                    </Button>
                }
            } else {
                html! {
                    <Button
                        id={format!("image_to_compare_button_{index}")}
                        class={classes![
                            "h-1/2",
                            "md:h-5/6",
                            "aspect-square",
                            "w-fit",
                        ]}
                        onclick={on_image_select}
                    >
                        <img
                            id={format!("image_to_compare_{index}")}
                            class={classes!["h-full"]}
                            src={image.clone()}
                            alt=""
                        />
                    </Button>
                }
            }
        })
        .collect::<Html>()
}
