pub(crate) mod images;
pub(crate) mod user;

use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

pub(crate) use self::{
    images::{
        get_comparison_for_user,
        post_vote,
        Comparison,
        Vote,
    },
    user::{
        get_user,
        User,
    },
};

#[derive(Deserialize)]
pub(crate) struct Response<T, E> {
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) data: Option<T>,
    pub(crate) error: Option<E>,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::Response;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn response_struct_is_deserializable_with_data_field() {
        let value = serde_json::json!({
            "timestamp": Utc::now(),
            "data": 0,
        });

        assert!(serde_json::from_value::<Response<usize, ()>>(value).is_ok());
    }

    #[wasm_bindgen_test]
    fn response_struct_is_deserializable_with_error_field() {
        let value = serde_json::json!({
            "timestamp": Utc::now(),
            "error": 0,
        });

        assert!(serde_json::from_value::<Response<(), usize>>(value).is_ok());
    }
}
