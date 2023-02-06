mod change_user_modal;
mod header;
mod image_list;
mod instructions_modal;
mod question_mark;

use yew::{
    classes,
    function_component,
    html,
    use_effect,
    use_state,
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
    dom::DOM,
    request::{
        get_images,
        get_user,
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
    let show_instructions_modal = use_state_eq(|| true);
    let loading = use_state_eq(|| true);
    let image_list =
        use_state_eq(|| ImagesResponse::default().to_vec());
    let user_info = use_state_eq(|| User::default());
    let selected_image = use_state(|| None);

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
        let show_instructions_modal = show_instructions_modal.clone();
        let show_fatal_error_modal = show_fatal_error_modal.clone();
        let loading = loading.clone();
        let image_list = image_list.clone();
        let user_info = user_info.clone();

        use_effect(move || {
            if *loading && !*show_fatal_error_modal {
                let t = wasm_bindgen::JsValue::from("changing...");
                web_sys::console::log_1(&t);

                wasm_bindgen_futures::spawn_local(async move {
                    let images_response = get_images().await;
                    let user_response = get_user().await;
                    match (images_response, user_response) {
                        (Ok(images), Ok(user)) => {
                            image_list.set(images);
                            if user.votes > 0 {
                                show_instructions_modal.set(false);
                            }
                            user_info.set(user);
                            loading.set(false);
                            let debug_string =
                                wasm_bindgen::JsValue::from(
                                    "changed",
                                );
                            web_sys::console::log_1(&debug_string);
                        },
                        (_, _) => show_fatal_error_modal.set(true),
                    }
                });
            }
        });
    }

    let on_image_select = {
        let loading = loading.clone();
        let selected_image = selected_image.clone();

        Callback::from(move |image: Image| {
            loading.set(true);
            let debug_string = wasm_bindgen::JsValue::from(format!(
                "image chosen: {},  user agent: {:?}, language: {:?}",
                &image.id.to_string(),
                DOM::user_agent(),
                DOM::language(),
            ));
            web_sys::console::log_1(&debug_string);
            selected_image.set(Some(image));
        })
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
                        images={(*image_list).clone()}
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
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::ImagesToCompare;
    use crate::{
        dom::DOM,
        render_yew_component,
        request::get_user,
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
        render_yew_component!(ImagesToCompare);

        let user =
            get_user().await.expect("Request to return Ok response");
        let expected_text =
            format!("I'm done with {} votes!", user.votes);
        let button = DOM::get_button_by_id("finish_comparing_button")
            .expect("finish_comparing_button to be present");

        assert_eq!(button.inner_html(), expected_text);
    }

    #[wasm_bindgen_test]
    async fn two_images_to_compare_exist() {
        render_yew_component!(ImagesToCompare);

        assert_eq!(DOM::get_images().unwrap_or(vec![]).len(), 2);
    }

    #[wasm_bindgen_test]
    async fn choosing_first_image_loads_new_images() {
        render_yew_component!(ImagesToCompare);

        let images = DOM::get_images().expect("Images to be present");

        let image = images[0]
            .clone()
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement");

        image.click();
        wasm_sleep!(50);
        assert_eq!(DOM::get_images().unwrap_or(vec![]).len(), 0);
        wasm_sleep!(100);
        assert_eq!(DOM::get_images().unwrap_or(vec![]).len(), 2);
    }

    #[wasm_bindgen_test]
    async fn choosing_second_image_loads_new_images() {
        render_yew_component!(ImagesToCompare);

        let images = DOM::get_images().expect("Images to be present");

        let image = images[1]
            .clone()
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement");

        image.click();
        wasm_sleep!(50);
        assert_eq!(DOM::get_images().unwrap_or(vec![]).len(), 0);
        wasm_sleep!(100);
        assert_eq!(DOM::get_images().unwrap_or(vec![]).len(), 2);
    }

    #[wasm_bindgen_test]
    async fn if_user_has_0_votes_show_instructions_modal() {
        render_yew_component!(ImagesToCompare);

        let user =
            get_user().await.expect("Request to return Ok response");

        match DOM::get_element_by_id("instructions_modal") {
            Some(_) => assert_eq!(user.votes, 0),
            None => assert_ne!(user.votes, 0),
        };
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
        wasm_sleep!(50);
        assert!(
            DOM::get_element_by_id("instructions_modal").is_some()
        );
    }
}
