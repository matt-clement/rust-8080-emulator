use super::state_8080::State8080;
use super::emulator;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture};

// Display is 60Hz, clock is 2MHz
const CYCLES_PER_FRAME: u32 = 2_000_000 / 60;

pub fn start(state: &mut State8080) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("space-invaders", 224, 256)
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
        draw(&state, &mut canvas);
        canvas.present();
        // Display is 60Hz, clock is 2MHz, this is close enough for now I guess
        let mut cycle_count = 0;
        while cycle_count < CYCLES_PER_FRAME {
            cycle_count += emulator::emulate_8080_op(state);

        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

enum PixelState {
    ON,
    OFF,
}

fn draw(state: &State8080, canvas: &mut Canvas<sdl2::video::Window>) {
    let texture_creator = canvas.texture_creator();
    let mut tex1 = texture_creator.create_texture_target(None, 224, 256).unwrap();
    let mut tex2 = texture_creator.create_texture_target(None, 224, 256).unwrap();
    let textures: Vec<(&mut Texture, PixelState)> = vec![
        (&mut tex1, PixelState::ON),
        (&mut tex2, PixelState::OFF),
    ];
    let vram: &[u8] = &state.memory[0x2400..0x3fff];
    canvas.with_multiple_texture_canvas(textures.iter(), |texture_canvas, pixel_state| {
        match *pixel_state {
            PixelState::ON => {
                texture_canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
                vram.iter().enumerate().for_each(|(index, value)| {
                    let bit_number = index * 8;
                    (0..0x8).into_iter().for_each(|shift| {
                        let pixel_number = bit_number + shift;
                        let bit = (value & (1 << shift)) >> shift;
                        let row = pixel_number / 256;
                        let column = pixel_number % 256;
                        match bit {
                            1 => texture_canvas.draw_point(Point::new(row as i32, column as i32)).unwrap(),
                            _ => {},
                        }
                    });
                });
            },
            PixelState::OFF => {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                vram.iter().enumerate().for_each(|(index, value)| {
                    let bit_number = index * 8;
                    (0..0x8).into_iter().for_each(|shift| {
                        let pixel_number = bit_number + shift;
                        let bit = (value & (1 << shift)) >> shift;
                        let row = pixel_number / 256;
                        let column = pixel_number % 256;
                        match bit {
                            0 => texture_canvas.draw_point(Point::new(row as i32, column as i32)).unwrap(),
                            _ => {},
                        }
                    });
                });
            }
        }
    }).unwrap();
    canvas.copy_ex(&tex1, None, None, 0.0, Point::new(0, 0), false, true).unwrap();
    canvas.copy_ex(&tex2, None, None, 0.0, Point::new(0, 0), false, true).unwrap();
}