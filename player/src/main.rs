use std::{
    array,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 360;

const BUFFER_SIZE: usize = (WINDOW_WIDTH * WINDOW_HEIGHT * 4) as usize;
const BLACK_COLOR: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];
const COLOR_SIZE: usize = 4;

const TARGET_FPS: f64 = 60.0;
const TARGET_FRAME_TIME: f64 = 1.0 / TARGET_FPS;

struct GamePlayer<'win> {
    pixels: Option<Pixels<'win>>,
    window: Option<Arc<Window>>,
    buffer: [u8; BUFFER_SIZE],
    last_color: [u8; 4],
    should_exit: bool,
    is_paused: bool,
    previous_instant: Instant,
    current_instant: Instant,
}

impl GamePlayer<'_> {
    fn new() -> Self {
        let color = [0x00, 0x00, 0x00, 0xFF];
        Self {
            pixels: None,
            window: None,
            buffer: array::from_fn(|i| color[i % color.len()]),
            last_color: color,
            should_exit: false,
            is_paused: false,
            previous_instant: Instant::now(),
            current_instant: Instant::now(),
        }
    }

    fn update(&mut self) {
        let last_value = self.last_color.len() - 1;
        for (i, v) in self.last_color.iter_mut().enumerate() {
            if i != last_value {
                *v = if *v == 0xFF { 0x00 } else { *v + 1 }
            }
        }

        for (i, pixel) in self.buffer.chunks_exact_mut(COLOR_SIZE).enumerate() {
            let x = i % WINDOW_WIDTH as usize;
            let y = i / WINDOW_WIDTH as usize;

            pixel.copy_from_slice(if (x / 16) % 2 == (y / 16) % 2 {
                &self.last_color
            } else {
                &BLACK_COLOR
            });
        }
    }

    fn draw(&mut self) {
        let frame = self.pixels.as_mut().unwrap().frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(COLOR_SIZE).enumerate() {
            let color = &self.buffer[i * COLOR_SIZE..(i + 1) * COLOR_SIZE];
            pixel.copy_from_slice(color);
        }
    }

    fn handle_input(&mut self, event: &KeyEvent) {
        if event.state == ElementState::Pressed && !event.repeat {
            match event.logical_key {
                Key::Named(NamedKey::Escape) => self.should_exit = true,
                Key::Named(NamedKey::Space) => self.is_paused = !self.is_paused,
                _ => {}
            }
        }
    }

    fn handle_update(&mut self) {
        if !self.is_paused {
            self.update();
        }
    }

    fn handle_drawing(&mut self) {
        self.draw();
        if let Err(err) = self.pixels.as_ref().unwrap().render() {
            error!("{err:?}");
            self.should_exit = true;
        }
    }
}

impl ApplicationHandler for GamePlayer<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        let window_attributes = Window::default_attributes()
            .with_title("SN-50 Player")
            .with_inner_size(size)
            .with_resizable(false);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.clone());
        let pixels = Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture).unwrap();

        self.pixels = Some(pixels);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if self.should_exit {
            event_loop.exit();
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.should_exit = true;
            }
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                self.handle_input(&key_event);
            }
            WindowEvent::RedrawRequested => {
                self.current_instant = Instant::now();
                let elapsed = self
                    .current_instant
                    .duration_since(self.previous_instant)
                    .as_secs_f64();
                self.previous_instant = self.current_instant;
                info!("Elapsed: {}", elapsed);

                self.handle_update();
                self.handle_drawing();

                let delay = TARGET_FRAME_TIME - elapsed;
                info!("Delay: {}", delay);
                if delay > 0.0 {
                    std::thread::sleep(Duration::from_secs_f64(delay));
                }

                info!("FPS: {}", 1.0 / (elapsed + delay));

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut player = GamePlayer::new();
    event_loop.run_app(&mut player)?;

    Ok(())
}
