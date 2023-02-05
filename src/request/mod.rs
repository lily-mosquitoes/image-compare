pub(crate) mod images;
pub(crate) mod user;

use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

pub(crate) use self::{
    images::{
        get_images,
        Image,
        ImagesResponse,
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
