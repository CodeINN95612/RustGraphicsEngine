use winit::{
    event::MouseButton,
    keyboard::{KeyCode, SmolStr},
};

#[derive(Debug)]
pub enum EngineEvent {
    Unknown,
    NotImplemented,
    Update,
    Render,
    WindowClosedRequested,
    WindowResized {
        width: u32,
        height: u32,
        state: WindowResizedState,
    },
    WindowMoved {
        x: i32,
        y: i32,
    },
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    MouseMoved {
        x: f64,
        y: f64,
    },
    MouseWheel(f32),
    KeyboardKeyPressed(KeyCode),
    KeyboardKeyReleased(KeyCode),
    KeyboardStr {
        text: SmolStr,
        repeat: bool,
    },
}

#[derive(Debug)]
pub enum WindowResizedState {
    None,
    Maximized,
    Minimized,
}
