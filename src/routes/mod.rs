use actix_web::dev::{Body, MessageBody};
use actix_web::App;

pub mod home;
pub mod web;
pub mod oauth2;
pub mod api;

pub fn routes_import(cfg: &mut actix_web::web::ServiceConfig) {
    api::routes_import(cfg);
    web::routes_import(cfg);
}