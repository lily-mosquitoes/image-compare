use yew::prelude::*;
use yew_router::prelude::*;
use web_sys;
use crate::Route;

#[function_component(Welcome)]
pub(crate) fn welcome() -> Html {
    let current_card = use_state(|| 1);
 
    let cards_section = move || -> web_sys::Element {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("information")
            .unwrap()
    };
    
    let total_cards = move || -> i32 {
        cards_section().child_element_count() as i32
    };
    use_effect(move || {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("total_cards")
            .unwrap()
            .set_inner_html(&format!("{}", total_cards()));
    });

    let cards_section_width = move || -> i32 {
        cards_section().scroll_width()
    };
    
    let cards_section_left_position = move || -> i32 {
        cards_section().scroll_left()
    };
    
    let card_length = move || -> i32 {
        cards_section_width() / total_cards()
    };
    
    let card_index = move || -> i32 {
        (cards_section_left_position() + (card_length() / 2))
            / card_length()
    };

    let onscroll = {
        let current_card = current_card.clone();
        Callback::from(move |_| {
            current_card.set(card_index() + 1);
        })
    };

    let scroll_by = |x: i32| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("information")
            .unwrap()
            .set_scroll_left(x);
    };

    let scroll_left = {
        let scroll_by = scroll_by.clone();
        Callback::from(move |_| {
            scroll_by(cards_section_left_position()-card_length());
        })
    };

    let scroll_right = {
        let scroll_by = scroll_by.clone();
        Callback::from(move |_| {
            scroll_by(cards_section_left_position()+card_length());
        })
    };

    html! {
        <section
            id="welcome"
            class="mx-auto my-10 lg:my-3 w-full lg:w-1/2 \
            flex flex-col items-center justify-center gap-10 lg:gap-3"
        >
            <h1 class="font-bold text-8xl lg:text-3xl text-violet-800">
                { "Image Compare" }
            </h1>
            <section
                id="information"
                onscroll={onscroll}
                class="flex items-stretch gap-10 px-3 w-4/5 overflow-scroll scroll-smooth scrollbar-hide snap-x snap-mandatory snap-always"
            >
                <InstructionCard
                    title="Welcome to Image Compare!"
                    text="This is a citizen sciene project where you will be able to..."
                    has_button={false}
                />
                <InstructionCard
                    title="How it works"
                    text="You'll be shown two pictures and..."
                    has_button={false}
                />
                <InstructionCard
                    title="What information we collect"
                    text="We keep information on your votes using cookies to determine the user... If you accept this please click the button below to start!"
                    has_button={true}
                />
            </section>
            <section id="slide_buttons" class="w-1/2 flex justify-between text-6xl lg:text-base">
                <button id="slide_left" onclick={scroll_left} class="rounded-full bg-violet-800 px-10 lg:px-3 py-5 lg:py-1.5 symbol-6xl lg:symbol-base text-white">
                    { "arrow_back" }
                </button>
                <section id="slide_counter" class="flex flex-row gap-1 rounded-full bg-violet-200 px-10 lg:px-3 py-5 lg:py-1.5 text-violet-800">
                    <p>{ format!("{}", *current_card) }</p>
                    <p>{ "/" }</p>
                    <p id="total_cards">{ "0" }</p>
                </section>
                <button id="slide_right" onclick={scroll_right} class="rounded-full bg-violet-800 px-10 lg:px-3 py-5 lg:py-1.5 symbol-6xl lg:symbol-base text-white">
                    { "arrow_forward" }
                </button>
            </section>
            <GetFingerprintButton button_type={ButtonType::Link} />
        </section>
    }
}

#[derive(PartialEq)]
enum ButtonType {
    Full,
    Link,
}

#[derive(Properties, PartialEq)]
struct GetFingerprintButtonProps {
    button_type: ButtonType,
}

#[function_component(GetFingerprintButton)]
fn get_fingerprint_button(props: &GetFingerprintButtonProps) -> Html {
    let navigator = use_navigator()
        .expect("navigator to be avaliable");

    let get_fingerprint = Callback::from(move |_| {
        use wasm_bindgen::JsCast;
        fn htmldocument() -> web_sys::HtmlDocument {
            web_sys::window()
                .expect("window to be present")
                .document()
                .expect("document to be present") 
                .dyn_into::<web_sys::HtmlDocument>()
                .expect("Document to be castable to HtmlDocument")
        }

        htmldocument()
            .set_cookie("fingerprint=testvalue; path=/")
            .unwrap();

        navigator.push(&Route::Welcome)
    });

    let (classes, text): (&str, &str) = match props.button_type {
        ButtonType::Full => {(
            "px-10 lg:px-3 py-5 lg:py-1.5 w-fit rounded-full font-bold bg-violet-800 text-white hover:bg-violet-700 text-5xl lg:text-base",
            "Accept and start!"
        )},
        ButtonType::Link => {(
            "underline text-violet-400 hover:text-violet-800 text-5xl lg:text-base",
            "I already know this, let me start!"
        )},
    };

    html! {
        <button
            id="get_fingerprint"
            class={classes}
            onclick={get_fingerprint}
        >
            { text }
        </button>
    }
}

#[derive(Properties, PartialEq)]
struct InstructionCardProps {
    title: String,
    text: String,
    has_button: bool,
}

#[function_component(InstructionCard)]
fn instruction_card(props: &InstructionCardProps) -> Html {
    html! {
        <div
            class="w-full shrink-0 snap-center \
            flex flex-col items-center gap-10 lg:gap-3 \
            bg-violet-200 border-8 lg:border-2 border-violet-800 \
            rounded-3xl lg:rounded-xl \
            py-3 md:py-10 px-5"
        >
            <h2 class="font-bold text-5xl lg:text-xl">
                { &props.title }
            </h2>
            <p class="px-5 text-5xl lg:text-base">
                { &props.text }
            </p>
            if props.has_button {
                <GetFingerprintButton button_type={ButtonType::Full} />
            }
        </div>
    }
}
