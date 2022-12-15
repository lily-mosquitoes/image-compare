use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Welcome)]
pub(crate) fn welcome() -> Html {
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
        <section id="welcome">
            <h1>{ "No fingerprint" }</h1>
            <button id="get_fingerprint" onclick={get_fingerprint}>
                { "Get fingerprint" }
            </button>
        </section>
    }
}

