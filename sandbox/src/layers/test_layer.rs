use engine::{app::layer::Layer, events::engine_event::EngineEvent, log::info};

pub struct TestLayer {
    test: u32,
}

impl TestLayer {
    pub fn new() -> Self {
        Self { test: 0 }
    }
}

impl Layer for TestLayer {
    fn on_init(&mut self) {}

    fn on_update(&mut self, _: engine::app::timestep::Timestep) {}

    fn on_terminate(&mut self) {}

    fn on_render(&mut self) {}

    fn on_event(&mut self, event: &engine::events::engine_event::EngineEvent) {
        match event {
            EngineEvent::MouseButtonPressed(_) => {
                self.test += 1;
                info!("Pressed {} times!", self.test);
            }
            _ => (),
        }
    }
}
