use super::{app::App, app_properties::AppProperties};

pub struct AppBuilder {
    app_properties: AppProperties,
}

impl AppBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            app_properties: AppProperties {
                name,
                logging_enabled: true,
            },
        }
    }

    pub fn build(self) -> Option<App> {
        App::new(self.app_properties)
    }

    pub fn disable_logger(mut self) -> Self {
        self.app_properties.logging_enabled = false;
        return self;
    }
}
