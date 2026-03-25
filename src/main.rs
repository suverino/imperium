use calloop::EventLoop;
use smithay::reexports::wayland_server::{Display, DisplayHandle};
use std::time::Duration;
use tracing::info;

struct Imperium {
    _display_handle: DisplayHandle,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("imperium starting");

    let mut event_loop: EventLoop<Imperium> = EventLoop::try_new()?;
    let mut display: Display<Imperium> = Display::new()?;
    let display_handle = display.handle();

    let mut state = Imperium {
        _display_handle: display_handle,
    };

    info!("imperium entering event loop");

    loop {
        event_loop.dispatch(Duration::from_millis(1000), &mut state)?;
        display.flush_clients()?;
    }
}
