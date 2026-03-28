use std::error::Error;

use smithay::{
    backend::{
        renderer::{Frame, Renderer, gles::GlesRenderer},
        winit::{self, WinitEvent, WinitGraphicsBackend},
    },
    utils::{Rectangle, Transform},
};

use crate::state::Imperium;

pub fn init() -> Result<(WinitGraphicsBackend<GlesRenderer>, winit::WinitEventLoop), Box<dyn Error>> {
    let (mut backend, mut winit_loop) = winit::init::<GlesRenderer>()?;

    backend.window().set_title("Imperium");
    backend.window().request_redraw();

    Ok((backend, winit_loop))
}

pub fn handle_event(
    backend: &mut WinitGraphicsBackend<GlesRenderer>,
    event: WinitEvent,
    state: &mut Imperium,
) {
    match event {
        WinitEvent::Resized { .. } => {
            backend.window().request_redraw();
        }
        WinitEvent::Redraw => {
            if let Err(err) = redraw(backend, state) {
                eprintln!("redraw failed: {err}");
                state.stop();
            }
        }
        WinitEvent::CloseRequested => {
            state.stop();
        }
        _ => {}
    }
}

fn redraw(
    backend: &mut WinitGraphicsBackend<GlesRenderer>,
    state: &mut Imperium,
) -> Result<(), Box<dyn Error>> {
    let size = backend.window_size();
    let damage = [Rectangle::new((0, 0).into(), size)];

    {
        let (renderer, mut framebuffer) = backend.bind()?;
        let mut frame = renderer.render(&mut framebuffer, size, Transform::Normal)?;

        frame.clear(state.clear_color, &damage)?;
        let _ = frame.finish()?;
    }
    
    backend.submit(Some(&damage))?;
    Ok(())
}
