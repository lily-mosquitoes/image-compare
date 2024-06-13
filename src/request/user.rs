#[cfg(test)]
use std::sync::atomic::{
    AtomicBool,
    AtomicUsize,
    Ordering,
};

use serde::Deserialize;

use super::Response;
use crate::dom::{
    console_error,
    DOM,
};

#[derive(Clone, PartialEq, Default, Deserialize)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) votes: usize,
    pub(crate) average_lambda: f64,
}

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
            average_lambda: 0.65,
        });
    }

    let user = gloo_net::http::Request::get(&format!("/api/user/{id}"))
        .send()
        .await
        .map_err(|error| console_error!(error.to_string()))?
        .json::<Response<User, String>>()
        .await
        .map_err(|error| console_error!(error.to_string()))?
        .as_result()
        .map_err(|error| console_error!(error))?;

    Ok(user)
}

pub(crate) async fn generate_user() -> Result<User, ()> {
    #[cfg(test)]
    if cfg!(test) {
        return Ok(User::new());
    }

    let user = gloo_net::http::Request::post("/api/user")
        .send()
        .await
        .map_err(|error| console_error!(error.to_string()))?
        .json::<Response<User, String>>()
        .await
        .map_err(|error| console_error!(error.to_string()))?
        .as_result()
        .map_err(|error| console_error!(error))?;

    Ok(user)
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
            average_lambda: 0.0,
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
            "average_lambda": 0.65
        });

        assert!(serde_json::from_value::<User>(value).is_ok());
    }
}
