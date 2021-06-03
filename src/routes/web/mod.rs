pub mod home;
pub mod register;
use actix_web::web;
pub use register::display_register_page;

pub fn routes_import(cfg: &mut web::ServiceConfig) {
    // Add services here:
    //cfg.service(register::display_register_page);
    cfg.service(home::display_home_page);
}
