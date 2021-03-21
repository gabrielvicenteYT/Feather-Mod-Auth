use crate::utils::error::WebsiteErrors;
use anyhow::Result;
use handlebars::{Handlebars, TemplateError};
use log::{error, info};
use serde::Serialize;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use std::process::exit;

pub struct HandlebarsUtils<'a> {
    handlebars: Handlebars<'a>,
    base_path: PathBuf,
}

impl<'a> HandlebarsUtils<'a> {
    pub fn register_handlebars(&mut self) -> bool {
        info!("Registering handlebars");
        match self
            .handlebars
            .register_templates_directory(".handlebars", &self.base_path)
        {
            Ok(_) => {
                info!(
                    "Registered {} templates!",
                    self.handlebars.get_templates().len()
                );
                true
            }
            Err(e) => {
                error!("There was an error while registering templates: {:#?}", e);
                exit(-1)
            }
        }
    }

    pub fn render_handlebar<T>(&self, name: &str, data: &T) -> Result<String, WebsiteErrors>
    where
        T: Serialize,
    {
        Ok(self.handlebars.render(name, data)?)
    }
}

impl<'a> Default for HandlebarsUtils<'a> {
    fn default() -> Self {
        HandlebarsUtils {
            handlebars: Handlebars::new(),
            base_path: dotenv::var("TEMPLATE_PATH")
                .expect("The 'TEMPLATE_PATH' environment variable is missing.")
                .parse()
                .expect("The 'TEMPLATE_PATH' variable is not valid, please check your input."),
        }
    }
}
