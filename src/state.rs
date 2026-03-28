use smithay::{
    backend::renderer::Color32F,
    reexports::wayland_server::DisplayHandle,
};

pub struct Imperium {
    pub display_handle: DisplayHandle,
    pub running: bool,
    pub clear_color: Color32F,
}

impl Imperium {
    pub fn new(display_handle: DisplayHandle) -> Self {
        Self {
            display_handle,
            running: true,
            clear_color: Color32F::new(0.08, 0.09, 0.11, 1.0),
        }

    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
