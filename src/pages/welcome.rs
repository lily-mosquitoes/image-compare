use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {   
    html! {
        <section id="main">
            <h1>{ "No cookies" }</h1>
        </section>
    }
}

