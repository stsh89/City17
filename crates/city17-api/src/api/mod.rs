mod error;

pub mod symbols;

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct PagePresenter<T> {
    items: Vec<T>,
    next_page_token: Option<String>,
}
