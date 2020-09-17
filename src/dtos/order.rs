use serde::Serialize;

#[derive(Serialize)]
pub struct Order {
    pub id: i32,
    pub title: String
}