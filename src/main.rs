use smithay::reexports::wayland_server::Display;
use winit::platform::pump_events::PumpStatus;
use tracing::info;

mod backend;
mod state;

use state::Imperium;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("imperium starting");

    let mut display: Display<Imperium> = Display::new()?;
    let display_handle = display.handle();

    let mut state = Imperium::new(display_handle);

    let (mut backend, mut winit) = backend::winit::init()?;

    info!("imperium entering event loop");

    loop {
        if !state.running {
            break;
        }

        let status = winit.dispatch_new_events(|event| {
            backend::winit::handle_event(&mut backend, event, &mut state);
        });

        match status {
            PumpStatus::Continue => {}
            PumpStatus::Exit(_) => {
                break;
            }
        }

        display.flush_clients()?;
    }

    Ok(())
}
