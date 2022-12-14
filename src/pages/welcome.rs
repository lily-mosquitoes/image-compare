use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {   
    html! {
        <section id="welcome">
            <h1>{ "No fingerprint" }</h1>
            <button id="get_fingerprint">
                { "Get fingerprint" }
            </button>
        </section>
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests_setup::*;

    setup_environment!();

    #[wasm_bindgen_test]
    async fn test_get_fingerprint() {
        WasmWindow::document()
            .set_raw_cookies(EMPTY_FINGERPRINT);

        render_app!(Welcome);

        let fingerprint_button = WasmWindow::document()
            .get_element_by_id("get_fingerprint")
            .as_html();

        fingerprint_button.click();

        let cookies = WasmWindow::document()
            .get_raw_cookies()
            .expect("cookies to not be null");

        assert!(cookies.contains("fingerprint"));
    }
}
