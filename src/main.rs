use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::time::Duration;
use sdl2::rect::Rect;
use crate::game::{Particle, World, Sand, ParticleType, Wood};
use std::collections::HashMap;
use sdl2::mouse::MouseButton;

mod game;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Particles Testing", 700, 500)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut world = World::new(700, 500);

    let mut current_particle = ParticleType::SAND;

    let (mut pressing, mut deleting) = (false, false);
    let mut mouse_position = (0, 0);

    let mut pause = false;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Set background to black
        canvas.clear(); // Clear the screen before drawing on it
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running, // Quit the game
                Event::KeyDown { keycode: Some(k), .. } => {
                    match k {
                        Keycode::Escape => break 'running, // Quit the game
                        Keycode::Num1 => current_particle = ParticleType::SAND, // Set sand as current brush
                        Keycode::Num2 => current_particle = ParticleType::WOOD, // Set wood as current brush
                        Keycode::R => world.clean(), // Clean the world
                        Keycode::P => pause = !pause, // Pause/Unpause the game
                        _ => {}
                    }
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        pressing = true;
                    } else if mouse_btn == MouseButton::Right {
                        deleting = true;
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        pressing = false;
                    } else if mouse_btn == MouseButton::Right {
                        deleting = false;
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    mouse_position = (x, y); // Update the mouse position
                }
                _ => {}
            }
        }

        if pressing { // If pressing the left mouse button
            match current_particle {
                ParticleType::WOOD => { // Spawn the wood inside the brush area
                    for i in 0..5 {
                        world.spawn_particle(Box::new(Wood {
                            x: mouse_position.0 + i - 2,
                            y: mouse_position.1 + 4,
                            does_exists: true
                        }));
                    }
                    for i in 0..7 {
                        world.spawn_particle(Box::new(Wood {
                            x: mouse_position.0 + i - 3,
                            y: mouse_position.1 + 3,
                            does_exists: true
                        }));
                    }
                    for k in 0..5 {
                        for i in 0..9 {
                            world.spawn_particle(Box::new(Wood {
                                x: mouse_position.0 + i - 4,
                                y: mouse_position.1 - 2 + k,
                                does_exists: true
                            }));
                        }
                    }
                    for i in 0..7 {
                        world.spawn_particle(Box::new(Wood {
                            x: mouse_position.0 + i - 3,
                            y: mouse_position.1 - 3,
                            does_exists: true
                        }));
                    }
                    for i in 0..5 {
                        world.spawn_particle(Box::new(Wood {
                            x: mouse_position.0 + i - 2,
                            y: mouse_position.1 - 4,
                            does_exists: true
                        }));
                    }
                }
                ParticleType::SAND => { // Spawn the sand inside the brush area
                    for i in 0..5 {
                        world.spawn_particle(Box::new(Sand {
                            x: mouse_position.0 + i - 2,
                            y: mouse_position.1 + 4,
                            does_exists: true
                        }));
                    }
                    for i in 0..7 {
                        world.spawn_particle(Box::new(Sand {
                            x: mouse_position.0 + i - 3,
                            y: mouse_position.1 + 3,
                            does_exists: true
                        }));
                    }
                    for k in 0..5 {
                        for i in 0..9 {
                            world.spawn_particle(Box::new(Sand {
                                x: mouse_position.0 + i - 4,
                                y: mouse_position.1 - 2 + k,
                                does_exists: true
                            }));
                        }
                    }
                    for i in 0..7 {
                        world.spawn_particle(Box::new(Sand {
                            x: mouse_position.0 + i - 3,
                            y: mouse_position.1 - 3,
                            does_exists: true
                        }));
                    }
                    for i in 0..5 {
                        world.spawn_particle(Box::new(Sand {
                            x: mouse_position.0 + i - 2,
                            y: mouse_position.1 - 4,
                            does_exists: true
                        }));
                    }
                }
            }
        } else if deleting { // Delete particles inside the brush area
            for i in 0..3 {
                world.destroy_particle(mouse_position.0+i-1, mouse_position.1+2);
            }
            for k in 0..3 {
                for i in 0..5 {
                    world.destroy_particle(mouse_position.0 + i - 2, mouse_position.1-1+k);
                }
            }
            for i in 0..3 {
                world.destroy_particle(mouse_position.0+i-1, mouse_position.1+2);
            }
        }

        // Update the particles physics if the game is not paused
        if !pause {
            world.update();
        }

        // Render the particles
        world.render(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}