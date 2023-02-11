use yew::{
    classes,
    function_component,
    html,
    use_effect,
    use_state_eq,
    AttrValue,
    Callback,
    Html,
    Properties,
};

use super::{
    dot_button::DotButton,
    instructions_card::InstructionsCard,
};
use crate::{
    dom::{
        console_error,
        DOM,
    },
    shared_components::Modal,
};

static ABOUT_THE_PROJECT_EN: &str =
    include_str!("../../markdown/about_the_project-EN.md");

static HOW_TO_PARTICIPATE_EN: &str =
    include_str!("../../markdown/how_to_participate-EN.md");

static DISCLAIMER_EN: &str =
    include_str!("../../markdown/disclaimer-EN.md");

fn markdown_to_yew_html(text: &str) -> Html {
    let html_string = markdown::to_html(text);
    Html::from_html_unchecked(AttrValue::from(html_string))
}

#[derive(Properties, PartialEq)]
pub(super) struct InstructionsModalProps {
    pub(super) onclose: Callback<()>,
}

#[function_component(InstructionsModal)]
pub(super) fn instructions_modal(
    props: &InstructionsModalProps,
) -> Html {
    let number_of_cards = use_state_eq(|| 0);
    let currently_visible_card = use_state_eq(|| 0);

    let get_number_of_cards = || -> u32 {
        match DOM::get_element_by_id("instructions_cards") {
            Some(element) => element.child_element_count(),
            None => 0,
        }
    };

    {
        let number_of_cards = number_of_cards.clone();
        let get_number_of_cards = get_number_of_cards.clone();

        use_effect(move || {
            let n = get_number_of_cards();
            number_of_cards.set(n);
        });
    }

    let onscroll = {
        let currently_visible_card = currently_visible_card.clone();
        let number_of_cards = *number_of_cards;

        Callback::from(move |_| {
            match DOM::get_element_by_id("instructions_cards") {
                Some(element) => {
                    let card_length = element.scroll_width()
                        / number_of_cards as i32;
                    let index = (element.scroll_left()
                        + (card_length / 2))
                        / card_length;
                    currently_visible_card.set(index as u32);
                },
                None => (),
            }
        })
    };

    let scroll_to = |index: u32| {
        let currently_visible_card = *currently_visible_card as i32;
        let number_of_cards = *number_of_cards as i32;

        Callback::from(move |_| {
            match DOM::get_element_by_id("instructions_cards") {
                Some(element) => {
                    let card_length =
                        element.scroll_width() / number_of_cards;
                    let modifier =
                        index as i32 - currently_visible_card;
                    let scroll_amount = element.scroll_left()
                        + (card_length * modifier);
                    element.set_scroll_left(scroll_amount);
                },
                None => (),
            }
        })
    };

    let about_the_project =
        markdown_to_yew_html(ABOUT_THE_PROJECT_EN);

    let how_to_participate =
        markdown_to_yew_html(HOW_TO_PARTICIPATE_EN);

    let disclaimer = markdown_to_yew_html(DISCLAIMER_EN);

    html! {
        <Modal
            id={"instructions_modal"}
            onclose={props.onclose.clone()}
        >
            <section
                id={"instructions_cards"}
                class={classes![
                    "flex",
                    "flex-row",
                    "items-stretch",
                    "overflow-scroll",
                    "snap-x",
                    "snap-mandatory",
                    "snap-always",
                    "scrollbar-hide",
                    "scroll-smooth",
                ]}
                onscroll={onscroll}
            >
                <InstructionsCard id={"about_the_project"}>
                    { about_the_project }
                </InstructionsCard>
                <InstructionsCard id={"how_to_participate"}>
                    { how_to_participate }
                </InstructionsCard>
                <InstructionsCard id={"disclaimer"}>
                    { disclaimer }
                </InstructionsCard>
            </section>
            <section
                id={"instructions_cards_buttons"}
                class={classes![
                    "mt-8",
                    "p-8",
                    "flex",
                    "flex-row",
                    "gap-10",
                    "justify-center",
                ]}
            >
                {
                    (0..*number_of_cards).map(|index| {
                        let selected = index == *currently_visible_card;
                        html! {
                            <DotButton
                                index={index}
                                selected={selected}
                                onclick={scroll_to(index)}
                            />
                        }
                    }).collect::<Html>()
                }
            </section>
        </Modal>
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };
    use yew::{
        function_component,
        html,
        Html,
    };

    use super::InstructionsModal;
    use crate::{
        dom::DOM,
        helpers_for_tests::wasm_sleep_in_ms,
        markdown_to_decoded_html,
        render_yew_component,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[function_component(TestInstructionsModal)]
    fn test_istructions_modal() -> Html {
        html! {
            <div>
                <InstructionsModal onclose={|_| ()} />
            </div>
        }
    }

    #[wasm_bindgen_test]
    async fn about_the_project_text_is_visible() {
        render_yew_component!(TestInstructionsModal);
        wasm_sleep_in_ms(50).await;

        let expected =
            include_str!("../../markdown/about_the_project-EN.md");
        let expected = markdown_to_decoded_html(expected);

        let text = DOM::get_element_by_id("about_the_project")
            .expect("Element #about_the_project to exist");

        assert_eq!(text.inner_html(), expected);
    }

    #[wasm_bindgen_test]
    async fn how_to_participate_text_is_visible() {
        render_yew_component!(TestInstructionsModal);
        wasm_sleep_in_ms(50).await;

        let expected =
            include_str!("../../markdown/how_to_participate-EN.md");
        let expected = markdown_to_decoded_html(expected);

        let text = DOM::get_element_by_id("how_to_participate")
            .expect("Element #how_to_participate to exist");

        assert_eq!(text.inner_html(), expected);
    }

    #[wasm_bindgen_test]
    async fn disclaimer_text_is_visible() {
        render_yew_component!(TestInstructionsModal);
        wasm_sleep_in_ms(50).await;

        let expected =
            include_str!("../../markdown/disclaimer-EN.md");
        let expected = markdown_to_decoded_html(expected);

        let text = DOM::get_element_by_id("disclaimer")
            .expect("Element #disclaimer to exist");

        assert_eq!(text.inner_html(), expected);
    }
}
