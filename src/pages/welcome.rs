use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {   
    html! {
        <section id="welcome">
            <h1>{ "No cookies" }</h1>
        </section>
    }
}

