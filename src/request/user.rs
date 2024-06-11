#[cfg(test)]
use std::sync::atomic::{
    AtomicBool,
    AtomicUsize,
    Ordering,
};

use serde::Deserialize;

use crate::dom::DOM;

#[derive(Clone, PartialEq, Default, Deserialize)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) votes: usize,
    pub(crate) average_chosen_lambda: Option<f64>,
}

pub(crate) static MOCK_VOTES: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

pub(crate) async fn get_user() -> Result<User, ()> {
    #[cfg(test)]
    if !GET_USER_RETURNS_OK.load(Ordering::SeqCst) {
        return Err(());
    }

    let user_id = DOM::local_storage()
        .and_then(|storage| storage.get_item("user_id").unwrap_or(None));

    match user_id {
        Some(id) => get_user_by_id(&id).await,
        None => {
            let user = generate_user().await?;
            DOM::local_storage()
                .ok_or(())?
                .set_item("user_id", &user.id)
                .or(Err(()))?;
            Ok(user)
        },
    }
}

pub(crate) async fn get_user_by_id(id: &str) -> Result<User, ()> {
    #[cfg(test)]
    if cfg!(test) {
        return Ok(User {
            id: id.to_string(),
            votes: VOTES_TO_DISPLAY.load(Ordering::SeqCst),
            average_chosen_lambda: Some(0.65),
        });
    }
    yew::platform::time::sleep(std::time::Duration::from_millis(500)).await;
    Ok(User {
        id: id.to_string(),
        votes: MOCK_VOTES.load(std::sync::atomic::Ordering::SeqCst),
        average_chosen_lambda: Some(0.65),
    })
}

pub(crate) async fn generate_user() -> Result<User, ()> {
    #[cfg(test)]
    if cfg!(test) {
        return Ok(User::new());
    }
    yew::platform::time::sleep(std::time::Duration::from_millis(500)).await;
    Ok(User::default())
}

#[cfg(test)]
pub(crate) static GET_USER_RETURNS_OK: AtomicBool = AtomicBool::new(true);

#[cfg(test)]
pub(crate) static VOTES_TO_DISPLAY: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
impl User {
    fn new() -> Self {
        VOTES_TO_DISPLAY.store(0, Ordering::SeqCst);
        let id: String = (0..16)
            .map(|_| format!("{:x}", rand::random::<u8>()))
            .collect();
        Self {
            id,
            votes: 0,
            average_chosen_lambda: None,
        }
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
            "id": "55555555555555555555555555555555",
            "votes": 4,
            "average_chosen_lambda": 0.65
        });

        assert!(serde_json::from_value::<User>(value).is_ok());
    }
}
