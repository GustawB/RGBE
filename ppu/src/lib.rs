use std::{sync::Arc, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub struct Ppu {
    context: Sdl,
    canvas: Canvas<Window>,
    //vram: []
}

impl Ppu {
    pub fn new() -> Ppu {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("gameboy demo", 800, 720)
        .position_centered()
        .build()
        .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        Ppu {
            context: sdl_context,
            canvas: canvas,
        }
    }

    pub fn execute(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
        let mut event_pump = self.context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            //draw_frame(&mut canvas, tile_data, tile_map);
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn get_color(low: u8, high: u8, idx: i32) -> u8 {
        let high_bit: u8 = (high & (1 << 7) >> idx) >> 7 - idx;
        let low_bit: u8 = (low & (1 << 7) >> idx) >> 7 - idx;
        (high_bit << 1) | low_bit
    }

    fn draw_tile(&mut self, color_palette: &[u8; 4], tile: &[u8; 16], tile_x: usize, tile_y: usize) {
        for i in 0..8i32 {
            for j in 0..8i32 {
                let color: u8 = Ppu::get_color(tile[(2*i) as usize], tile[(2*i+1) as usize], j);
                let pixel: Rect = Rect::new((tile_x*40) as i32+j*5, (tile_y*40) as i32+i*5, 5, 5);

                let rgb_val: u8 = 255 - color_palette[color as usize] * 85;
                self.canvas.set_draw_color(Color::RGB(rgb_val, rgb_val, rgb_val));
                self.canvas.draw_rect(pixel).unwrap();
                self.canvas.fill_rect(pixel).unwrap();
            }
        }
    }

    fn draw_frame(&mut self, tile_data: [[u8; 16]; 256], tile_map: [u8; 360]) {
        for i in 0..18usize {
            for j in 0..20usize {
                let tile: &[u8; 16] = &tile_data[tile_map[i*20 + j] as usize];
                self.draw_tile(&[0, 1, 2, 3], tile, j, i);
            }
        }
    }
}