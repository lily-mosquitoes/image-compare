pub(crate) mod images;
pub(crate) mod user;

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
    pub(crate) data: Option<T>,
    pub(crate) error: Option<E>,
}

impl<T, E> Response<T, E> {
    pub(crate) fn as_result(self) -> Result<T, E> {
        match (self.data, self.error) {
            (Some(data), _) => Ok(data),
            (_, Some(error)) => Err(error),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::Response;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn response_struct_is_deserializable_with_data_field() {
        let value = serde_json::json!({
            "data": 0,
        });

        assert!(serde_json::from_value::<Response<usize, ()>>(value).is_ok());
    }

    #[wasm_bindgen_test]
    fn response_struct_is_deserializable_with_error_field() {
        let value = serde_json::json!({
            "error": 0,
        });

        assert!(serde_json::from_value::<Response<(), usize>>(value).is_ok());
    }

    #[wasm_bindgen_test]
    fn response_struct_with_data_field_can_be_cast_as_result() {
        let value = serde_json::json!({
            "data": 0,
        });

        let response = serde_json::from_value::<Response<usize, ()>>(value)
            .expect("to be deserializable");

        assert!(response.as_result().is_ok());
    }

    #[wasm_bindgen_test]
    fn response_struct_with_error_field_can_be_cast_as_result() {
        let value = serde_json::json!({
            "error": 0,
        });

        let response = serde_json::from_value::<Response<(), usize>>(value)
            .expect("to be deserializable");

        assert!(response.as_result().is_err());
    }
}
