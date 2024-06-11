#[cfg(test)]
use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::dom::DOM;

#[derive(Clone, PartialEq, Deserialize)]
pub(crate) struct Comparison {
    pub(crate) id: String,
    pub(crate) images: Vec<String>,
}

pub(crate) async fn get_comparison_for_user(
    _user_id: String,
) -> Result<Comparison, ()> {
    #[cfg(test)]
    if cfg!(test) {
        // sleep a bit to allow test to see the loading status
        crate::wasm_sleep_in_ms(50).await;

        return match GET_IMAGES_RETURNS_OK.load(Ordering::SeqCst) {
            true => Ok(Comparison::default()),
            false => Err(()),
        };
    }

    yew::platform::time::sleep(std::time::Duration::from_millis(500)).await;

    let comparison = Comparison {
        id: "".to_string(),
        images: vec![
            "https://i.imgur.com/3ByU8xj.png".to_string(),
            "https://i.imgur.com/KN2lyRT.png".to_string(),
        ],
    };

    Ok(comparison)
}

impl Default for Comparison {
    fn default() -> Self {
        Self {
            id: String::default(),
            images: vec![String::default(), String::default()],
        }
    }
}

#[cfg(test)]
pub(crate) static GET_IMAGES_RETURNS_OK: AtomicBool = AtomicBool::new(true);

#[derive(Serialize)]
pub(crate) struct Vote {
    comparison_id: String,
    #[serde(skip_serializing)]
    _comparison_images: Vec<String>,
    user_id: String,
    image: String,
    user_agent: Option<String>,
    language: Option<String>,
}

pub(crate) async fn post_vote(vote: Vote) -> Result<(), ()> {
    #[cfg(test)]
    if cfg!(test) {
        return match vote._comparison_images.contains(&vote.image) {
            true => Ok(()),
            false => Err(()),
        };
    }

    let debug_string = wasm_bindgen::JsValue::from(format!(
        "image was chosen: {}",
        vote.image
    ));
    web_sys::console::log_1(&debug_string);

    super::user::MOCK_VOTES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

impl Vote {
    pub(crate) fn build(comparison: Comparison) -> Self {
        Self {
            comparison_id: comparison.id,
            _comparison_images: comparison.images,
            user_id: String::default(),
            image: String::default(),
            user_agent: DOM::user_agent(),
            language: DOM::language(),
        }
    }

    pub(crate) fn user(mut self, user_id: String) -> Self {
        self.user_id = user_id;
        self
    }

    pub(crate) fn vote(mut self, image: String) -> Self {
        self.image = image;
        self
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::{
        Comparison,
        Vote,
    };
    use crate::dom::DOM;
    wasm_bindgen_test_configure!(run_in_browser);

    fn test_comparison() -> Comparison {
        Comparison {
            id: "55555555555555555555555555555555".to_string(),
            images: vec![
                "/image/path/0.png".to_string(),
                "/image/path/1.png".to_string(),
            ],
        }
    }

    #[wasm_bindgen_test]
    fn vote_contains_user_agent() {
        let vote: Vote = Vote::build(test_comparison())
            .user("44444444444444444444444444444444".to_string())
            .vote("/image/path/0.png".to_string());

        assert_eq!(vote.user_agent, DOM::user_agent());
    }

    #[wasm_bindgen_test]
    fn vote_contains_language() {
        let vote: Vote = Vote::build(test_comparison())
            .user("44444444444444444444444444444444".to_string())
            .vote("/image/path/0.png".to_string());

        assert_eq!(vote.language, DOM::language());
    }

    #[wasm_bindgen_test]
    fn vote_contains_image() {
        let vote: Vote = Vote::build(test_comparison())
            .user("44444444444444444444444444444444".to_string())
            .vote("/image/path/0.png".to_string());

        assert_eq!(vote.image, "/image/path/0.png");
    }

    #[wasm_bindgen_test]
    fn vote_contains_user_id() {
        let vote: Vote = Vote::build(test_comparison())
            .user("44444444444444444444444444444444".to_string())
            .vote("/image/path/0.png".to_string());

        assert_eq!(vote.user_id, "44444444444444444444444444444444");
    }

    #[wasm_bindgen_test]
    fn vote_contains_comparison_id() {
        let vote: Vote = Vote::build(test_comparison())
            .user("44444444444444444444444444444444".to_string())
            .vote("/image/path/0.png".to_string());

        assert_eq!(vote.comparison_id, "55555555555555555555555555555555");
    }

    #[wasm_bindgen_test]
    fn vote_is_serializable() {
        let vote: Vote = Vote::build(test_comparison())
            .user("44444444444444444444444444444444".to_string())
            .vote("/image/path/0.png".to_string());

        assert!(serde_json::to_value(vote).is_ok())
    }

    #[wasm_bindgen_test]
    fn comparison_is_deserializable() {
        let value = serde_json::json!({
            "id": "55555555555555555555555555555555",
            "images": ["/image/path/0.png", "/image/path/1.png"]
        });

        assert!(serde_json::from_value::<Comparison>(value).is_ok());
    }
}
