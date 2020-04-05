use sdl2::render::Canvas;
use super::state_8080::State8080;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

enum PixelState {
    ON,
    OFF,
}

// Width: 224, Height: 256

pub fn draw(state: &State8080, canvas: &mut Canvas<sdl2::video::Window>) {
    let texture_creator = canvas.texture_creator();
    let mut tex1 = texture_creator.create_texture_target(None, 224, 256).unwrap();
    // let mut pixel_on = (&mut tex1, TextureColor::WHITE);
    let mut tex2 = texture_creator.create_texture_target(None, 224, 256).unwrap();
    // let mut pixel_off = (&mut tex2, TextureColor::BLACK);
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
    canvas.copy(&tex1, None, None).unwrap();
    canvas.copy(&tex2, None, None).unwrap();
}