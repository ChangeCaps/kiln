use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{error::Result, render::Renderer};

pub struct Window {
    pub title: String,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: String::from("kiln window"),
        }
    }
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(self, mut f: impl FnMut(Event<()>, &Renderer) -> bool + 'static) -> Result<()> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(self.title)
            .build(&event_loop)?;

        let mut renderer = unsafe { Renderer::new(&window) };

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(new_size) => {
                        renderer.config.width = new_size.width;
                        renderer.config.height = new_size.height;
                        renderer.configure();
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        renderer.config.width = new_inner_size.width;
                        renderer.config.height = new_inner_size.height;
                        renderer.configure();
                    }
                    _ => {}
                },
                _ => {}
            }

            if f(event, &renderer) {
                println!("redraw");
                window.request_redraw();
            }
        });
    }
}
