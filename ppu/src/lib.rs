use std::{collections::VecDeque, sync::{Arc, Mutex, MutexGuard}, time::Duration};

use constants::{IO_REGS_BASE, LCDC, LY, OAM_SIZE, SCX, SCY, VRAM_BASE, WX, WY};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub struct Ppu {
    context: Sdl,
    canvas: Canvas<Window>,
    vram: Arc<Mutex<[u8; 0x2000]>>,
    oam: Option<Arc<Mutex<[u8; 0x100]>>>,
    io_regs: Arc<Mutex<[u8; 0x80]>>,

    sprite_buffer: Vec<Sprite>,
    fifo: VecDeque<u8>,
    x_counter: u8,
}

struct Sprite {
    y_pos: u8,
    x_pos: u8,
    tile_idx: u8,
    attrs: u8,
}

impl Ppu {
    pub fn new(vram: Arc<Mutex<[u8; 0x2000]>>, oam: Arc<Mutex<[u8; 0x100]>>, io_regs: Arc<Mutex<[u8; 0x80]>>) -> Ppu {
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
            vram: vram,
            oam: Some(oam),
            io_regs: io_regs,
            sprite_buffer: Vec::new(),
            fifo: VecDeque::new(),
            x_counter: 0,
        }
    }

    fn get_color(low: u8, high: u8, idx: i32) -> u8 {
        let high_bit: u8 = (high & (1 << 7) >> idx) >> 7 - idx;
        let low_bit: u8 = (low & (1 << 7) >> idx) >> 7 - idx;
        (high_bit << 1) | low_bit
    }
    
    fn draw_pixel(canvas: &mut Canvas<Window>, io_regs_guard: &MutexGuard<'_, [u8; 0x80]>, idx: u8, color: u8) {
        let ly: u8 = io_regs_guard[LY - IO_REGS_BASE];
        let px: Rect = Rect::new((idx as u16 * 5).into(), ly.into(),5, 5);

        let color_palette: [u8; 4] = [0, 1, 2, 3];
        let rgb_val: u8 = 255 - color_palette[color as usize] * 85;

        canvas.set_draw_color(Color::RGB(rgb_val, rgb_val, rgb_val));
        canvas.draw_rect(px).unwrap();
        canvas.fill_rect(px).unwrap();
    }

    fn mode2(&mut self, oam_guard: &MutexGuard<'_, [u8; 0x100]>) {
        let ly: u8 = self.io_regs.lock().unwrap()[LY - IO_REGS_BASE];

        let mut counter: usize = 0;
        while counter < OAM_SIZE {
            let y_pos: u8 = oam_guard[counter];
            let x_pos: u8 = oam_guard[counter + 1];
            let tile_idx: u8 = oam_guard[counter + 2];
            let attrs: u8 = oam_guard[counter + 3];
            let sprite_heights = if self.io_regs.lock().unwrap()[LCDC - IO_REGS_BASE] & (1 << 2) == 0 { 0 } else { 1 };

            if ly + 16 >= y_pos && ly + 16 < y_pos + sprite_heights && self.sprite_buffer.len() < 10 {
                self.sprite_buffer.push(Sprite {
                    y_pos: y_pos,
                    x_pos: x_pos,
                    tile_idx: tile_idx,
                    attrs: attrs,
                });
            }

            counter += 4;
        }
    }

    fn fetch_tile(_oam_guard: &MutexGuard<'_,[u8; 0x100]>,
                            vram_guard: &MutexGuard<'_, [u8; 0x2000]>,
                            io_regs_guard: &MutexGuard<'_, [u8; 0x80]>,
                            pixel_idx: u8, fifo: &mut VecDeque<u8>) {
        let tile_base_addr: u16 = 0x9800;
        let lcdc: u8 = io_regs_guard[LCDC - IO_REGS_BASE];
        let scx: u8 = io_regs_guard[SCX - IO_REGS_BASE];
        let scy: u8 = io_regs_guard[SCY - IO_REGS_BASE];

        let ly: u8 = io_regs_guard[LY - IO_REGS_BASE];

        // Skipping Window logic for now
        let curr_tile: u16 = (((pixel_idx + (scx / 8)) & 0x1F).wrapping_add(32u8.wrapping_mul(((ly.wrapping_add(scy)) & 0xFF) / 8))).into();
        let tile_idx: u8 = vram_guard[(tile_base_addr + curr_tile) as usize - VRAM_BASE];

        let tdl: u8 = vram_guard[tile_idx as usize];
        let tdh: u8 = vram_guard[tile_idx as usize];

        for i in 0..8 {
            let px: u8 = Ppu::get_color(tdl, tdh, i);
            fifo.push_back(px); 
        }
    }

    fn mode3(&mut self, oam_guard: &MutexGuard<'_, [u8; 0x100]>) {
        let ly: u8 = oam_guard[LY - IO_REGS_BASE];
        let vram_guard = self.vram.lock().unwrap();
        let io_regs_guard = self.io_regs.lock().unwrap();

        let mut drawn_pixels: u8 = 0;
        let mut fetched_tiles: u8 = 0;
        while drawn_pixels < 160 {
            if self.fifo.len() > 8 {
                let px: u8 = self.fifo.pop_front().unwrap();
                Ppu::draw_pixel(&mut self.canvas, &io_regs_guard, drawn_pixels, px);
                drawn_pixels += 1;
            } else {
                // fetch new tile and update queue
                Ppu::fetch_tile(&oam_guard, &vram_guard, &io_regs_guard, fetched_tiles, &mut self.fifo);
                fetched_tiles += 1;
            }
        }
    }

    fn mode0(&mut self) {
        self.sprite_buffer.clear();
        self.fifo.clear();
        //::std::thread::sleep(Duration::from_millis(500));
    }

    fn mode1() {
        ::std::thread::sleep(Duration::from_millis(1));
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

            self.draw_frame();
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn draw_tile(&mut self, color_palette: &[u8; 4], tile: &[u8; 16], tile_x: usize, tile_y: usize) {
        for i in 0..8i32 {
            for j in 0..8i32 {
                let color: u8 = Ppu::get_color(tile[(2*i) as usize], tile[(2*i+1) as usize], j);
                let pixel: Rect = Rect::new((tile_x*40) as i32+j*5, (tile_y*40) as i32+i*5, 5, 5);

                let rgb_val: u8 = 255 - color_palette[color as usize] * 85;
                self.canvas.set_draw_color(Color::RGB(1, 0, 0));
                self.canvas.draw_rect(pixel).unwrap();
                self.canvas.set_draw_color(Color::RGB(rgb_val, rgb_val, rgb_val));
                self.canvas.fill_rect(pixel).unwrap();
            }
        }
    }

    fn draw_frame(&mut self) {
        for i in 0..144 {
            self.io_regs.lock().unwrap()[LY - IO_REGS_BASE] = i;
            let oam = self.oam.take();
            let oam_mutex = oam.unwrap();
            {
                let oam_guard = oam_mutex.lock().unwrap();
                self.mode2(&oam_guard);
                self.mode3(&oam_guard);
            }
            self.oam = Some(oam_mutex);
            self.mode0();
        }
        for i in 0..10 {
            self.io_regs.lock().unwrap()[LY - IO_REGS_BASE] = 144 + i;
            Ppu::mode1();
        }
    }

    /*fn draw_frame(&mut self, tile_data: [[u8; 16]; 256], tile_map: [u8; 360]) {
        for i in 0..18usize {
            for j in 0..20usize {
                let tile: &[u8; 16] = &tile_data[tile_map[i*20 + j] as usize];
                self.draw_tile(&[0, 1, 2, 3], tile, j, i);
            }
        }
    }*/
}