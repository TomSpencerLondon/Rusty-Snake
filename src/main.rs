use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time;

fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game", width + 1, height + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

fn main() {
    let canvas_width = 720_u32;
    let canvas_height = 720_u32;
    let (canvas, mut events) = init(canvas_width, canvas_height);

    thread::spawn(move || {
        'game: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit {
                        ..
                    } | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'game,
                    _ => continue 'game,
                }
            }
        }
        thread::sleep(time::Duration::from_millis(800));
    });

}
