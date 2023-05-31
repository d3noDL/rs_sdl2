use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    let screen_width = 800;
    let screen_height = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("The Bomberboiz!", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let screen_area = Rect::new(0, 0, screen_width, screen_height);
    let screen_color = Color::RGB(64, 128, 255);

    canvas.set_draw_color(screen_color);

    let mut event_queue = sdl_context.event_pump().unwrap();

    'update: loop {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => break 'update,
                _ => (),
            }
        }

        canvas.fill_rect(screen_area)?;
        canvas.present();
    }

    'draw: loop {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => break 'draw,
                _ => (),
            }
        }
    }

    return Ok(());
}
