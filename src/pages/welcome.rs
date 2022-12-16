use yew::prelude::*;
use yew_router::prelude::*;
use web_sys;
use crate::Route;

#[function_component(Welcome)]
pub(crate) fn welcome() -> Html {
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
            class="m-auto w-1/2 flex flex-col items-center justify-center gap-3"
        >
            <h1>{ "Image Compare" }</h1>
            <section
                id="information"
                onscroll={onscroll}
                class="flex items-stretch w-96 overflow-scroll scroll-smooth scrollbar-hide snap-x snap-mandatory snap-always"
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
                    title="What we get?"
                    text="We keep information on your votes using cookies to determine the user... If you accept this please click the button below to start!"
                    has_button={true}
                />
            </section>
            <section id="slide_buttons" class="w-1/2 flex justify-between">
                <button id="slide_left" onclick={scroll_left}>
                    { "<<" }
                </button>
                <section id="slide_counter" class="flex flex-row">
                    <p>{ format!("{}", *current_card) }</p>
                    <p>{ "/" }</p>
                    <p id="total_cards">{ "0" }</p>
                </section>
                <button id="slide_right" onclick={scroll_right}>
                    { ">>" }
                </button>
            </section>
            <button
                id="get_fingerprint"
                class="underline"
                onclick={get_fingerprint}
            >
                { "I already know this, let me start!" }
            </button>
        </section>
    }
}

#[function_component(GetFingerprintButton)]
fn get_fingerprint_button() -> Html {
    let navigator = use_navigator()
        .expect("navigator to be avaliable");

    let get_fingerprint = Callback::from(move |_| {
        use web_sys;
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

    html! {
        <button
            id="get_fingerprint"
            class="px-3 py-1.5 w-fit bg-violet-800 rounded-full"
            onclick={get_fingerprint}
        >
            { "Accept and start!" }
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
            class="w-96 shrink-0 snap-center flex flex-col items-center gap-3 border-2"
        >
            <h2>
                { &props.title }
            </h2>
            <p class="basis-full">{ &props.text }</p>
            if props.has_button {
                <GetFingerprintButton />
            }
        </div>
    }
}
