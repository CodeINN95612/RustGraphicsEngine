use crate::{app::timestep::Timestep, events::engine_event::EngineEvent};

pub trait Layer {
    fn on_init(&mut self);
    fn on_update(&mut self, timestep: Timestep);
    fn on_render(&mut self);
    fn on_terminate(&mut self);
    fn on_event(&mut self, event: &EngineEvent);
}
