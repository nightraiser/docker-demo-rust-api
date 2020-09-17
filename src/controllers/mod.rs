use rocket::Rocket;

pub mod order;

pub fn attach_routes(app: Rocket) -> Rocket {
    return app.mount("/order", routes![order::get_all]);
}