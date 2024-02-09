use std::{array, time::Instant};

use anyhow::Result;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 360;

const BUFFER_SIZE: usize = (WINDOW_WIDTH * WINDOW_HEIGHT * 4) as usize;
const COLOR_SIZE: usize = 4;

struct GamePlayer {
    buffer: [u8; BUFFER_SIZE],
}

impl GamePlayer {
    fn new() -> Self {
        let color = [0x00, 0x00, 0x00, 0xFF];
        Self {
            buffer: array::from_fn(|i| color[i % color.len()]),
        }
    }

    fn update(&mut self) {
        for (i, v) in self.buffer.iter_mut().enumerate() {
            let x = i % WINDOW_WIDTH as usize;
            let y = i / WINDOW_WIDTH as usize;

            *v = if (x / 24) % 2 == 0 && (y / 24) % 2 == 0 {
                0xFF
            } else {
                0x00
            };
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(COLOR_SIZE).enumerate() {
            let color = &self.buffer[i * COLOR_SIZE..(i + 1) * COLOR_SIZE];
            pixel.copy_from_slice(color);
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
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
    let mut last_frame = Instant::now();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, elwt| {
        info!("{event:?}");

        if input.update(&event) {
            info!("Last Frame: {:?}", last_frame.elapsed());
            last_frame = Instant::now();

            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            player.update();

            player.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                error!("{err:?}");
                elwt.exit();
                return;
            }
        }
    })?;

    Ok(())
}
