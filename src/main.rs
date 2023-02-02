use image_compare::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
pub(crate) mod tests_setup;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests_setup::*;

    setup_environment!();

    fn unset_fingerprint() {
        WasmWindow::document().set_raw_cookies(EMPTY_FINGERPRINT);
    }

    fn fingerprint_exists() -> bool {
        if let Some(cookies) =
            WasmWindow::document().get_raw_cookies()
        {
            cookies.contains("fingerprint")
        } else {
            false
        }
    }

    #[wasm_bindgen_test]
    async fn test_first_render_and_fingerprint_redirect() {
        unset_fingerprint();
        assert!(!fingerprint_exists());

        render_app!(App);

        // I could not figure out a way to test that the browser
        // gets redirected back to Welcome if the user tries to
        // go directly to Compare without a fingerprint...

        let mut id_of_first_child_from_main = WasmWindow::document()
            .get_element_by_id("main")
            .first_element_child()
            .expect("child to be present")
            .id();
        assert_eq!(id_of_first_child_from_main.as_str(), "welcome");

        let fingerprint_button = WasmWindow::document()
            .get_element_by_id("get_fingerprint")
            .as_html();

        fingerprint_button.click();
        wait_for_render!();
        assert!(fingerprint_exists());

        id_of_first_child_from_main = WasmWindow::document()
            .get_element_by_id("main")
            .first_element_child()
            .expect("child to be present")
            .id();
        assert_eq!(id_of_first_child_from_main.as_str(), "compare");
    }
}
