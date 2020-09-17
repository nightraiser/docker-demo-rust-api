#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

pub mod services;
pub mod dtos;
mod controllers;


fn main() {
    let config = rocket::config::Config::build(rocket::config::Environment::Staging)
    .address("0.0.0.0")
    .port(8000)
    .unwrap();

    controllers::attach_routes(rocket::custom(config)).launch();
}