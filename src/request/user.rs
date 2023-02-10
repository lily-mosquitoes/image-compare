use serde::Deserialize;

#[derive(Clone, PartialEq, Default, Deserialize)]
pub(crate) struct User {
    pub(crate) votes: usize,
    pub(crate) average_chosen_lambda: Option<f64>,
}

#[cfg(not(test))]
pub(crate) async fn get_user() -> Result<User, ()> {
    yew::platform::time::sleep(std::time::Duration::from_millis(500))
        .await;

    let mut user = User::default();
    user.votes = 1;

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
