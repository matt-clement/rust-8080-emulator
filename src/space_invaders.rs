use super::state_8080::State8080;
use super::emulator;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::Canvas;

// Display is 60Hz, clock is 2MHz
const CYCLES_PER_FRAME: u32 = 2_000_000 / 60;

struct SpaceInvadersMachine {
    state: State8080,

    last_timer: Instant,
    next_interrupt: f64,
    which_interrupt: i32,

    // emulator_timer: ???,

    shift_low: u8,
    shift_high: u8,
    shift_offset: u8,

    in_port1: u8,
    in_port2: u8,
    out_port3: u8,
    out_port5: u8,
    paused: bool,
}

impl SpaceInvadersMachine {
    fn new(state: State8080) -> SpaceInvadersMachine {
        SpaceInvadersMachine {
            state: state,
            last_timer: Instant::now(), // Should this be an Option<Instant>?
            next_interrupt: 0.0,
            which_interrupt: 0,
            // timer?
            shift_low: 0,
            shift_high: 0,
            shift_offset: 0,
            in_port1: 0,
            in_port2: 0,
            out_port3: 0,
            out_port5: 0,
            paused: false,
        }
    }
}

pub fn start(state: State8080) {
    let mut machine = SpaceInvadersMachine::new(state);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("space-invaders", 224 * 2, 256 * 2)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(10, 10, 10));
    canvas.clear();
    canvas.present();
    let mut color_scheme = ColorScheme::CLASSIC;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Q), ..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    color_scheme = match color_scheme {
                        ColorScheme::MONOCHROME => ColorScheme::CLASSIC,
                        ColorScheme::CLASSIC => ColorScheme::MONOCHROME,
                    };
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    machine.paused = !machine.paused;
                },
                Event::KeyDown { keycode: Some(key), .. } => machine_key_down(&mut machine, &key),
                Event::KeyUp { keycode: Some(key), .. } => machine_key_up(&mut machine, &key),
                _ => {}
            }
        }
        draw(&machine.state, &mut canvas, color_scheme);
        canvas.present();
        // Display is 60Hz, clock is 2MHz, this is close enough for now I guess
        let mut cycle_count = 0;
        while !machine.paused && (cycle_count < CYCLES_PER_FRAME) {
            let program_counter = machine.state.program_counter() as usize;
            let current_opcode = machine.state.memory[program_counter];
            // Special handling for interrupts. Eventually it would be nice to
            // have a way to do this without basically implementing instruction
            // handlerss outside of the main CPU emulator
            match current_opcode {
                // Special handling for IN
                0xdb => {
                    let port_number = machine.state.memory[program_counter + 1];
                    machine.state.a = handle_in(&mut machine, port_number);
                    machine.state.increment_program_counter(2);
                    cycle_count += 3;
                },
                // Special handling for OUT
                0xd3 => {
                    let port_number = machine.state.memory[program_counter + 1];
                    let value = machine.state.a;
                    handle_out(&mut machine, port_number, value);
                    machine.state.increment_program_counter(2);
                    cycle_count += 3;
                },
                _ => {
                    cycle_count += emulator::emulate_8080_op(&mut machine.state);
                }
            }
            let current_time = Instant::now();
            let time_since_last_interrupt = current_time.saturating_duration_since(machine.last_timer);
            if time_since_last_interrupt.as_secs_f64() > 1.0/60.0 {
                if machine.state.interrupt_enabled() {
                    machine.state.generate_interrupt(2);
                    machine.last_timer = current_time;
                }
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn handle_in(machine: &mut SpaceInvadersMachine, port: u8) -> u8 {
    match port {
        0 => { 0xf },
        1 => machine.in_port1,
        2 => { 0 }, // Player 2 controls and some other random stuff
        3 => {
            let value: u16 = ((machine.shift_high as u16) << 8) | machine.shift_low as u16;
            let masked_value: u8 = ((value >> (8 - machine.shift_offset)) & 0xff) as u8;
            masked_value
        },
        _ => { unreachable!() },
    }
}

fn handle_out(machine: &mut SpaceInvadersMachine, port: u8, value: u8) {
    match port {
        2 => {
            machine.shift_offset = value & 0x7;
        },
        3 => {
            machine.out_port3 = value;
        }
        4 => {
            machine.shift_low = machine.shift_high;
            machine.shift_high = value;
        }
        5 => {
            machine.out_port5 = value;
        }
        _ => {},
    }
}

#[derive(Clone,Copy)]
enum ColorScheme {
    CLASSIC,
    MONOCHROME,
}

impl ColorScheme {
    fn get_pixel_color(&self, x: u8, y: u8, state: u8) -> PixelColor {
        if state == 0 { return PixelColor::BLACK }
        match self {
            ColorScheme::MONOCHROME => PixelColor::WHITE,
            ColorScheme::CLASSIC => {
                if y >= 32 && y < 64 {
                    PixelColor::RED
                } else if y < 184 {
                    PixelColor::WHITE
                } else if y < 240 {
                    PixelColor::GREEN
                } else if x < 16 || x > 134 {
                    PixelColor::WHITE
                } else {
                    PixelColor::GREEN
                }
            }
        }
    }
}

enum PixelColor {
    BLACK = 0b00000000,
    RED = 0b11100000,
    GREEN = 0b00011100,
    WHITE = 0b11111111,
}

fn draw(state: &State8080, canvas: &mut Canvas<sdl2::video::Window>, color_scheme: ColorScheme) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_target(PixelFormatEnum::RGB332, 224, 256).unwrap();
    let vram: &[u8] = &state.memory[0x2400..0x4000];
    let pixels: &mut [u8] = &mut [0; 224 * 256];
    for i in 0..(224 * 256 / 8) {
        let memory_x = i % 32;
        let memory_y = i / 32;
        let current_byte = vram[i];
        let screen_x = memory_y;
        let screen_y_base = 255 - memory_x * 8;
        (0..0x8).into_iter().for_each(|shift| {
            let screen_y = screen_y_base - shift;
            let bit = (current_byte & (1 << shift)) >> shift;
            let expanded_pixel_data = color_scheme.get_pixel_color(screen_x as u8, screen_y as u8, bit);
            let pixel_index = (screen_x + screen_y * 224) as usize;
            pixels[pixel_index] = expanded_pixel_data as u8;
        })
    }

    texture.update(None, &pixels, 224).unwrap();

    canvas.copy(&texture, None, None).unwrap();
}

fn machine_key_down(machine: &mut SpaceInvadersMachine, key: &sdl2::keyboard::Keycode) {
    match key {
        Keycode::C => {
            machine.in_port1 |= 0x01;
        },
        Keycode::Left => {
            machine.in_port1 |= 0x20;
        },
        Keycode::Right => {
            machine.in_port1 |= 0x40;
        },
        Keycode::Z => {
            machine.in_port1 |= 0x10;
        },
        Keycode::T => {
            machine.in_port1 |= 0x04;
        },
        _ => {},
    }
}

fn machine_key_up(machine: &mut SpaceInvadersMachine, key: &sdl2::keyboard::Keycode) {
    match key {
        Keycode::C => {
            machine.in_port1 &= !0x01;
        },
        Keycode::Left => {
            machine.in_port1 &= !0x20;
        },
        Keycode::Right => {
            machine.in_port1 &= !0x40;
        },
        Keycode::Z => {
            machine.in_port1 &= !0x10;
        },
        Keycode::T => {
            machine.in_port1 &= !0x04;
        },
        _ => {},
    }
}
