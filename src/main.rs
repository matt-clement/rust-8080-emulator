use std::io::prelude::*;
use std::fs::File;

mod disassembler;
mod emulator;
mod space_invaders_display;
mod state_8080;

use state_8080::State8080;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let file_name = std::env::args().nth(1).expect("Pass file name as first argument");
    let mut file = File::open(&file_name).expect(&format!("Unable to open file '{}'", file_name));
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    while buffer.len() < 0x10000 {
        buffer.push(0);
    }
    let mut state = State8080::empty_state();
    state.memory = buffer;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 224, 256)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(10, 10, 10));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Q), ..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        space_invaders_display::draw(&state, &mut canvas);
        canvas.present();
        emulator::emulate_8080_op(&mut state);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn dothething() {
    let file_name = std::env::args().nth(1).expect("Pass file name as first argument");
    let mut file = File::open(&file_name).expect(&format!("Unable to open file '{}'", file_name));
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    while buffer.len() < 0x10000 {
        buffer.push(0);
    }
    let mut state = State8080::empty_state();
    state.memory = buffer;
    emulator::run(&mut state);
}
