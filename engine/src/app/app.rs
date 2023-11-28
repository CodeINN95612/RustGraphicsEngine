use std::{cell::RefCell, rc::Rc};

use log::{error, info};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    error::app_run_error::AppRunError,
    events::{engine_event::EngineEvent, event_processor::EventProcessor},
    utils::logger::Logger,
};

use super::{app_properties::AppProperties, layer::Layer};

pub struct App {
    event_loop: EventLoop<()>,
    layers: Rc<RefCell<Vec<Box<dyn Layer>>>>,
    window: Window,
}

impl App {
    pub fn new(app_properties: AppProperties) -> Option<Self> {
        if app_properties.logging_enabled {
            let Ok(_) = Logger::init() else {
                return None;
            };
        }

        let Ok(event_loop) = EventLoop::new() else {
            error!("Unable to create event loop");
            return None;
        };

        let Ok(window) = WindowBuilder::new()
            .with_title(app_properties.name)
            .build(&event_loop)
        else {
            error!("Unable to create window");
            return None;
        };

        info!("Starting App {}", app_properties.name);
        Some(Self {
            event_loop,
            window,
            layers: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn run(self) -> Result<(), AppRunError> {
        let layers = Rc::clone(&self.layers);
        layers.borrow_mut().iter_mut().for_each(|layer| {
            layer.on_init();
        });

        self.event_loop.set_control_flow(ControlFlow::Poll);
        let event_processor = EventProcessor::new(&self.window);
        let event_loop_result = self.event_loop.run(move |event, window_target| {
            let event = event_processor.process(event, window_target);

            let layers = Rc::clone(&self.layers);

            if let EngineEvent::Update = event {
                layers.borrow_mut().iter_mut().for_each(|layer| {
                    layer.on_update(1);
                });
            } else if let EngineEvent::Render = event {
                layers.borrow_mut().iter_mut().for_each(|layer| {
                    layer.on_render();
                });
            }

            layers.borrow_mut().iter_mut().for_each(|layer| {
                layer.on_event(&event);
            });
        });

        layers.borrow_mut().iter_mut().for_each(|layer| {
            layer.on_terminate();
        });

        match event_loop_result {
            Ok(_) => Ok(()),
            Err(_) => {
                error!("Unable to execute event loop");
                return Err(AppRunError);
            }
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.borrow_mut().push(layer);
    }
}
