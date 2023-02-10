mod change_user_modal;
mod header;
mod image_list;
mod instructions_modal;
mod question_mark;

use yew::{
    classes,
    function_component,
    html,
    use_effect_with_deps,
    use_state_eq,
    Callback,
    Html,
};

use self::{
    header::Header,
    image_list::ImageList,
    instructions_modal::InstructionsModal,
    question_mark::QuestionMark,
};
use crate::{
    request::{
        get_images,
        get_user,
        post_chosen_image,
        ChosenImage,
        Image,
        ImagesResponse,
        User,
    },
    shared_components::{
        Button,
        FatalErrorModal,
        Footer,
    },
};

#[function_component(ImagesToCompare)]
pub(crate) fn images_to_compare() -> Html {
    let show_fatal_error_modal = use_state_eq(|| false);
    let show_instructions_modal = use_state_eq(|| false);
    let loading = use_state_eq(|| true);
    let images_to_compare = use_state_eq(|| None);
    let user_info = use_state_eq(|| User::default());

    let close_fatal_error_modal = {
        let show_fatal_error_modal = show_fatal_error_modal.clone();
        Callback::from(move |_| {
            show_fatal_error_modal.set(false);
        })
    };

    let open_instructions_modal = {
        let show_instructions_modal = show_instructions_modal.clone();
        Callback::from(move |_| {
            show_instructions_modal.set(true);
        })
    };

    let close_instructions_modal = {
        let show_instructions_modal = show_instructions_modal.clone();
        Callback::from(move |_| {
            show_instructions_modal.set(false);
        })
    };

    {
        let show_fatal_error_modal = show_fatal_error_modal.clone();
        let show_instructions_modal = show_instructions_modal.clone();
        let loading = loading.clone();
        let images_to_compare = images_to_compare.clone();
        let images_to_compare_as_dependency =
            images_to_compare.clone();
        let user_info = user_info.clone();

        use_effect_with_deps(
            move |_| {
                if *loading && !*show_fatal_error_modal {
                    wasm_bindgen_futures::spawn_local(async move {
                        let images_response = get_images().await;
                        let user_response = get_user().await;
                        match (images_response, user_response) {
                            (Ok(images), Ok(user)) => {
                                loading.set(false);
                                if user.votes == 0 {
                                    show_instructions_modal.set(true);
                                }
                                user_info.set(user);
                                images_to_compare.set(Some(images));
                            },
                            (_, _) => {
                                show_fatal_error_modal.set(true);
                            },
                        }
                    });
                }
            },
            images_to_compare_as_dependency,
        );
    }

    let on_image_select = {
        let show_fatal_error_modal = show_fatal_error_modal.clone();
        let loading = loading.clone();
        let images_to_compare = images_to_compare.clone();

        Callback::from(move |image: Image| {
            loading.set(true);
            let show_fatal_error_modal =
                show_fatal_error_modal.clone();
            let images_to_compare = images_to_compare.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response =
                    post_chosen_image(ChosenImage::from(image)).await;
                match response {
                    Ok(_) => images_to_compare.set(None),
                    Err(_) => show_fatal_error_modal.set(true),
                }
            });
        })
    };

    let image_list_to_display = match (*images_to_compare).clone() {
        Some(images) => images.to_vec(),
        None => ImagesResponse::default().to_vec(),
    };

    html! {
        <section id="compare" class={classes!["flex", "flex-col", "h-full"]}>
            <Header user={(*user_info).clone()}/>
            <section id="content" class={classes!["flex-1"]}>
                <section
                    id="images_list"
                    class={classes!["flex", "flex-row"]}
                >
                    <ImageList
                        loading={(*loading).clone()}
                        images={image_list_to_display}
                        onclick={on_image_select}
                    />
                </section>
            </section>
            <Footer>
                <Button
                    id={"open_instructions_modal_button"}
                    onclick={open_instructions_modal}
                >
                  <QuestionMark
                    class={classes![
                        "h-16",
                        "text-gray-100",
                    ]}
                  />
                </Button>
            </Footer>
            if *show_instructions_modal && !*show_fatal_error_modal {
                <InstructionsModal onclose={close_instructions_modal} />
            }
            if *show_fatal_error_modal {
                <FatalErrorModal onclose={close_fatal_error_modal} />
            }
        </section>
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::ImagesToCompare;
    use crate::{
        dom::DOM,
        render_yew_component,
        request::{
            get_user,
            images::GET_IMAGES_RETURNS_OK,
            user::{
                GET_USER_RETURNS_OK,
                VOTES_TO_DISPLAY,
            },
        },
        wasm_sleep,
    };

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn button_to_change_user_exists() {
        render_yew_component!(ImagesToCompare);

        let change_user_button_text = "Reset user";

        assert!(DOM::has_button_with_inner_html(
            change_user_button_text
        ));
    }

    #[wasm_bindgen_test]
    async fn button_to_finish_comparing_exists() {
        render_yew_component!(ImagesToCompare);

        assert!(DOM::get_button_by_id("finish_comparing_button")
            .is_some());
    }

    #[wasm_bindgen_test]
    async fn button_to_finish_comparing_shows_user_votes() {
        GET_IMAGES_RETURNS_OK.store(true, Ordering::SeqCst);
        GET_USER_RETURNS_OK.store(true, Ordering::SeqCst);
        VOTES_TO_DISPLAY.store(rand::random(), Ordering::SeqCst);

        render_yew_component!(ImagesToCompare);

        let user =
            get_user().await.expect("request to return Ok response");
        let expected_text =
            format!("I'm done with {} votes!", user.votes);
        let button = DOM::get_button_by_id("finish_comparing_button")
            .expect("finish_comparing_button to be present");

        assert_eq!(button.inner_html(), expected_text);
    }

    #[wasm_bindgen_test]
    async fn show_fatal_error_modal_when_get_user_returns_error() {
        GET_IMAGES_RETURNS_OK.store(true, Ordering::SeqCst);
        GET_USER_RETURNS_OK.store(false, Ordering::SeqCst);

        render_yew_component!(ImagesToCompare);

        assert!(DOM::get_element_by_id("fatal_error_modal").is_some());
    }

    #[wasm_bindgen_test()]
    async fn show_fatal_error_modal_when_get_images_returns_error() {
        GET_IMAGES_RETURNS_OK.store(false, Ordering::SeqCst);
        GET_USER_RETURNS_OK.store(true, Ordering::SeqCst);

        render_yew_component!(ImagesToCompare);

        assert!(DOM::get_element_by_id("fatal_error_modal").is_some());
    }

    #[wasm_bindgen_test]
    async fn two_images_to_compare_exist() {
        GET_IMAGES_RETURNS_OK.store(true, Ordering::SeqCst);
        GET_USER_RETURNS_OK.store(true, Ordering::SeqCst);

        render_yew_component!(ImagesToCompare);

        assert_eq!(
            DOM::get_images_by_id("image_to_compare")
                .unwrap_or(vec![])
                .len(),
            2
        );
    }

    #[wasm_bindgen_test]
    async fn choosing_first_image_loads_new_images() {
        GET_IMAGES_RETURNS_OK.store(true, Ordering::SeqCst);
        GET_USER_RETURNS_OK.store(true, Ordering::SeqCst);

        render_yew_component!(ImagesToCompare);

        let images = DOM::get_images_by_id("image_to_compare")
            .expect("Images to compare to be present");

        let image = images[0]
            .clone()
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement");

        image.click();
        wasm_sleep!(50); // allow page to re-render
        assert!(DOM::get_images_by_id("image_to_compare").is_none());
        wasm_sleep!(100); // allow images to actually load
        assert_eq!(
            DOM::get_images_by_id("image_to_compare")
                .unwrap_or(vec![])
                .len(),
            2
        );
    }

    #[wasm_bindgen_test]
    async fn choosing_second_image_loads_new_images() {
        GET_IMAGES_RETURNS_OK.store(true, Ordering::SeqCst);
        GET_USER_RETURNS_OK.store(true, Ordering::SeqCst);

        render_yew_component!(ImagesToCompare);

        let images = DOM::get_images_by_id("image_to_compare")
            .expect("Images to compare to be present");

        let image = images[1]
            .clone()
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement");

        image.click();
        wasm_sleep!(50); // allow page to re-render
        assert!(DOM::get_images_by_id("image_to_compare").is_none());
        wasm_sleep!(100); // allow images to actually load
        assert_eq!(
            DOM::get_images_by_id("image_to_compare")
                .unwrap_or(vec![])
                .len(),
            2
        );
    }

    #[wasm_bindgen_test]
    async fn when_user_has_0_votes_show_instructions_modal() {
        GET_IMAGES_RETURNS_OK.store(true, Ordering::SeqCst);
        GET_USER_RETURNS_OK.store(true, Ordering::SeqCst);
        VOTES_TO_DISPLAY.store(0, Ordering::SeqCst);

        render_yew_component!(ImagesToCompare);

        assert!(
            DOM::get_element_by_id("instructions_modal").is_some()
        );
    }

    #[wasm_bindgen_test]
    async fn button_to_show_instructions_modal_exists() {
        render_yew_component!(ImagesToCompare);

        assert!(DOM::get_button_by_id(
            "open_instructions_modal_button"
        )
        .is_some());
    }

    #[wasm_bindgen_test]
    async fn button_to_show_instructions_modal_works() {
        render_yew_component!(ImagesToCompare);

        let button =
            DOM::get_button_by_id("open_instructions_modal_button")
                .expect(
                    "open_instructions_modal_button to be present",
                )
                .dyn_into::<web_sys::HtmlElement>()
                .expect("Element to be castable to HtmlElement");

        button.click();
        wasm_sleep!(50); // allow page to re-render
        assert!(
            DOM::get_element_by_id("instructions_modal").is_some()
        );
    }
}
