use winit::event_loop::EventLoop;
use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent};
use crate::{HEIGHT, WIDTH};

pub fn run_event_loop(mut f: impl FnMut(&mut [u8])) {
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    event_loop.run(move |event, _| {
        let Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } = event else {
            return;
        };
        f(pixels.frame_mut());
        pixels.render().unwrap();
        window.request_redraw();
    }).unwrap();

}
