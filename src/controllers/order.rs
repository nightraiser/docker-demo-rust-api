
use rocket_contrib::json::Json;
use crate::services;
use crate::dtos::order::Order;

#[get("/")]
pub fn get_all() -> Json<Vec<Order>> {
    Json(services::order_service::get_all_orders())
}