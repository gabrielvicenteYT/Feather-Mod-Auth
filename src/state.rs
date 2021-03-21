use crate::utils::handlebars_utils::HandlebarsUtils;
use handlebars::Handlebars;
use std::sync::{Arc, RwLock};
use std::borrow::Borrow;

pub struct State {
    pub handlebars: Arc<RwLock<HandlebarsUtils<'static>>>,
}
impl Default for State {
    fn default() -> Self {
        State {
            handlebars: Arc::new(RwLock::new(HandlebarsUtils::default())),
        }
    }
}
impl State {
    pub fn register(self) -> Self {
        self.handlebars.write().unwrap().register_handlebars();
        self
    }
}
