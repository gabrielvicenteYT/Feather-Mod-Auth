pub mod register;
pub mod home;
pub use register::display_register_page;
use actix_web::web;

pub fn routes_import(cfg: &mut web::ServiceConfig) {
    // Add services here:
    cfg.service(register::display_register_page);
    cfg.service(home::display_home_page);
}