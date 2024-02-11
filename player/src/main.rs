use std::{
    array,
    time::{Duration, Instant},
};

use anyhow::Result;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 360;

const BUFFER_SIZE: usize = (WINDOW_WIDTH * WINDOW_HEIGHT * 4) as usize;
const BLACK_COLOR: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];
const COLOR_SIZE: usize = 4;

const TARGET_FPS: f64 = 60.0;
const TARGET_FRAME_TIME: f64 = 1.0 / TARGET_FPS;

struct GamePlayer {
    buffer: [u8; BUFFER_SIZE],
    last_color: [u8; 4],
    should_exit: bool,
    is_paused: bool,
    previous_instant: Instant,
    current_instant: Instant,
}

impl GamePlayer {
    fn new() -> Self {
        let color = [0x00, 0x00, 0x00, 0xFF];
        Self {
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

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(COLOR_SIZE).enumerate() {
            let color = &self.buffer[i * COLOR_SIZE..(i + 1) * COLOR_SIZE];
            pixel.copy_from_slice(color);
        }
    }
}

fn handle_input(player: &mut GamePlayer, event: &KeyEvent) {
    if event.state == ElementState::Pressed && !event.repeat {
        match event.logical_key {
            Key::Named(NamedKey::Escape) => player.should_exit = true,
            Key::Named(NamedKey::Space) => player.is_paused = !player.is_paused,
            _ => {}
        }
    }
}

fn handle_update(player: &mut GamePlayer) {
    if !player.is_paused {
        player.update();
    }
}

fn handle_drawing(player: &mut GamePlayer, pixels: &mut Pixels) {
    player.draw(pixels.frame_mut());
    if let Err(err) = pixels.render() {
        error!("{err:?}");
        player.should_exit = true;
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("SN-50 Player")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)?
    };

    let mut player = GamePlayer::new();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, elwt| {
        if player.should_exit {
            elwt.exit();
            return;
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                player.should_exit = true;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event: key_event, ..
                    },
                ..
            } => {
                handle_input(&mut player, &key_event);
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                player.current_instant = Instant::now();
                let elapsed = player
                    .current_instant
                    .duration_since(player.previous_instant)
                    .as_secs_f64();
                player.previous_instant = player.current_instant;
                info!("Elapsed: {}", elapsed);

                handle_update(&mut player);
                handle_drawing(&mut player, &mut pixels);

                let delay = TARGET_FRAME_TIME - elapsed;
                info!("Delay: {}", delay);
                if delay > 0.0 {
                    std::thread::sleep(Duration::from_secs_f64(delay));
                }

                info!("FPS: {}", 1.0 / (elapsed + delay));

                window.request_redraw();
            }
            _ => (),
        }
    })?;

    Ok(())
}
