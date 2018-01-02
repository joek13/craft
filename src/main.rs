#[macro_use]
extern crate glium;

mod world;
mod render;
mod vertex;

use glium::{glutin, Surface};
use world::World;
use render::Render;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Craft Demo")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut world = World::new(&display);
    let mut running = true;
    while running {
        events_loop.poll_events(|ev| {
            use glutin::{Event, WindowEvent};

            match ev {
                Event::WindowEvent { event: e, .. } => {
                    match e {
                        WindowEvent::Closed => {
                            running = false;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        world.render(&mut target);

        target.finish();
    }
}
