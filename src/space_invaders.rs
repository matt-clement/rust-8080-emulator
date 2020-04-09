use super::state_8080::State8080;
use super::emulator;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture};

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
            paused: false,
        }
    }
}

pub fn start(state: State8080) {
    let mut machine = SpaceInvadersMachine::new(state);
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
                Event::KeyDown { keycode: Some(key), .. } => machine_key_down(&mut machine, &key),
                Event::KeyUp { keycode: Some(key), .. } => machine_key_up(&mut machine, &key),
                _ => {}
            }
        }
        draw(&machine.state, &mut canvas);
        canvas.present();
        // Display is 60Hz, clock is 2MHz, this is close enough for now I guess
        let mut cycle_count = 0;
        while cycle_count < CYCLES_PER_FRAME {
            let program_counter = machine.state.program_counter() as usize;
            let current_opcode = machine.state.memory[program_counter];
            // Special handling for interrupts. Eventually it would be nice to
            // have a way to do this without basically implementing instruction
            // handlerss outside of the main CPU emulator
            match current_opcode {
                // Special handling for IN
                0xdb => {
                    let port_number = machine.state.memory[program_counter + 1];
                    handle_in(&mut machine, port_number);
                    machine.state.increment_program_counter(2);
                },
                // Special handling for OUT
                0xd3 => {
                    let port_number = machine.state.memory[program_counter + 1];
                    let value = machine.state.a;
                    handle_out(&mut machine, port_number, value);
                    machine.state.increment_program_counter(2);
                },
                _ => {
                    cycle_count += emulator::emulate_8080_op(&mut machine.state);
                    let current_time = Instant::now();
                    let time_since_last_interrupt = current_time.saturating_duration_since(machine.last_timer);
                    if time_since_last_interrupt.as_secs_f64() > 1.0/60.0 {
                        if machine.state.interrupt_enabled() {
                            machine.state.generate_interrupt(2);
                            machine.last_timer = current_time;
                        }
                    }
                }
            }

        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn handle_in(machine: &mut SpaceInvadersMachine, port: u8) {
    match port {
        3 => {
            let value: u16 = ((machine.shift_high as u16) << 8) | machine.shift_low as u16;
            let masked_value: u8 = ((value >> (8 - machine.shift_offset)) & 0xff) as u8;
            machine.state.a = masked_value;
        },
        _ => {},
    }
}

fn handle_out(machine: &mut SpaceInvadersMachine, port: u8, value: u8) {
    match port {
        2 => {
            machine.shift_offset = value & 0x7;
        },
        4 => {
            machine.shift_low = machine.shift_high;
            machine.shift_high = value;
        }
        _ => {},
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

fn machine_key_down(machine: &mut SpaceInvadersMachine, key: &sdl2::keyboard::Keycode) {
    match key {
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
        Keycode::Left => {
            machine.in_port1 &= 0x20;
        },
        Keycode::Right => {
            machine.in_port1 &= 0x40;
        },
        Keycode::Z => {
            machine.in_port1 &= 0x10;
        },
        Keycode::T => {
            machine.in_port1 &= 0x04;
        },
        _ => {},
    }
}
