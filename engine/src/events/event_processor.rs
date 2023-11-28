use winit::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard,
    window::Window,
};

use super::engine_event::{EngineEvent, WindowResizedState};

pub struct EventProcessor<'a> {
    window: &'a Window,
}

impl<'a> EventProcessor<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    pub fn process(
        &self,
        event: Event<()>,
        window_target: &EventLoopWindowTarget<()>,
    ) -> EngineEvent {
        match event {
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                self.window.request_redraw();
                EngineEvent::Update
            }
            Event::WindowEvent { event, .. } => self.process_window_event(event, window_target),
            Event::DeviceEvent { event, .. } => self.process_device_event(event),
            _ => EngineEvent::NotImplemented,
        }
    }

    fn process_device_event(&self, event: DeviceEvent) -> EngineEvent {
        match event {
            DeviceEvent::Key(raw_key) => {
                let keyboard::PhysicalKey::Code(key) = raw_key.physical_key else {
                    return EngineEvent::Unknown;
                };

                match raw_key.state {
                    winit::event::ElementState::Pressed => EngineEvent::KeyboardKeyPressed(key),
                    winit::event::ElementState::Released => EngineEvent::KeyboardKeyReleased(key),
                }
            }
            _ => EngineEvent::NotImplemented,
        }
    }

    fn process_window_event(
        &self,
        event: WindowEvent,
        window_target: &EventLoopWindowTarget<()>,
    ) -> EngineEvent {
        match event {
            WindowEvent::CloseRequested => {
                window_target.exit();
                EngineEvent::WindowClosedRequested
            }
            WindowEvent::Resized(size) => {
                let width: u32 = size.width;
                let height: u32 = size.height;
                let mut state = WindowResizedState::None;

                if self.window.is_maximized() {
                    state = WindowResizedState::Maximized;
                } else if let Some(minimized) = self.window.is_minimized() {
                    if minimized {
                        state = WindowResizedState::Minimized;
                    }
                }
                EngineEvent::WindowResized {
                    width,
                    height,
                    state,
                }
            }
            WindowEvent::Moved(position) => {
                let x: i32 = position.x;
                let y: i32 = position.y;

                EngineEvent::WindowMoved { x, y }
            }
            WindowEvent::RedrawRequested => EngineEvent::Render,
            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x;
                let y = position.y;
                EngineEvent::MouseMoved { x, y }
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                winit::event::ElementState::Pressed => EngineEvent::MouseButtonPressed(button),
                winit::event::ElementState::Released => EngineEvent::MouseButtonReleased(button),
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        state,
                        repeat,
                        ..
                    },
                ..
            } => {
                if let ElementState::Pressed = state {
                    return match logical_key {
                        keyboard::Key::Character(text) => EngineEvent::KeyboardStr { text, repeat },
                        _ => EngineEvent::NotImplemented,
                    };
                }
                EngineEvent::NotImplemented
            }
            _ => EngineEvent::NotImplemented,
        }
    }
}
