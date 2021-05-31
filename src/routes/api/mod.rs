use actix_web::web;

pub mod register;
pub mod oauth_client;

pub fn routes_import(cfg: &mut web::ServiceConfig) {
    // Add services here:
    cfg.service(register::register);
}