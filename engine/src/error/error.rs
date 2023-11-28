use crate::error::app_error::AppError;

use super::renderer_error::RendererError;

#[derive(Debug)]
pub enum Error {
    App(AppError),
    Renderer(RendererError),
}
