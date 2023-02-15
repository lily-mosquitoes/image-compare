use serde::Deserialize;

#[derive(Clone, PartialEq, Default, Deserialize)]
pub(crate) struct User {
    pub(crate) votes: usize,
    pub(crate) average_chosen_lambda: Option<f64>,
}

pub(crate) static MOCK_VOTES: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[cfg(not(test))]
pub(crate) async fn get_user() -> Result<User, ()> {
    yew::platform::time::sleep(std::time::Duration::from_millis(500))
        .await;

    let user = User {
        votes: MOCK_VOTES
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        average_chosen_lambda: Some(0.65),
    };

    Ok(user)
}

#[cfg(test)]
use std::sync::atomic::{
    AtomicBool,
    AtomicUsize,
    Ordering,
};

#[cfg(test)]
pub(crate) static GET_USER_RETURNS_OK: AtomicBool =
    AtomicBool::new(true);

#[cfg(test)]
pub(crate) static VOTES_TO_DISPLAY: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) async fn get_user() -> Result<User, ()> {
    if GET_USER_RETURNS_OK.load(Ordering::SeqCst) {
        let mut user = User::default();
        user.votes = VOTES_TO_DISPLAY.load(Ordering::SeqCst);
        Ok(user)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::User;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn user_struct_is_deserializable() {
        let value = serde_json::json!({
            "votes": 4,
            "average_chosen_lambda": 0.65
        });

        assert!(serde_json::from_value::<User>(value).is_ok());
    }
}
