extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

pub mod game_window;
use game_window::GameWindow;
pub mod renderer;
use renderer::Renderer;
pub mod shader;

use gl::types::GLint;
use std::time::Instant;

//Game Settings
const TARGET_FPS: u32 = 60;
const GAME_TICKS_PER_SECOND: u32 = 1;
const SECONDS_PER_GAMETICK: f32 = 1.0 / GAME_TICKS_PER_SECOND as f32;

fn handle_events(
    event_pump: &mut sdl2::EventPump, 
    game_window: &mut GameWindow,
    renderer : &Renderer ) -> bool {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    for event in event_pump.poll_iter() {

        game_window
            .imgui_sdl2
            .handle_event(&mut game_window.imgui, &event);

        if game_window.imgui_sdl2.ignore_event(&event) {
            continue;
        }
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::O),
                ..
            } => {
                println!("o pressed!");
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                let is_fullscreen: bool = game_window.is_fullscreen();
                game_window.set_fullscreen(!is_fullscreen);

                let (width, height) = game_window.get_width_and_height();
                println!("width: {0} height: {1}", width, height);
                renderer.set_viewport(width, height);
            }
            _ => {}
        }
    }
    return true;
}

fn main() {
    let game_name: String = "Fighting Game".to_string();
    let mut game_window = game_window::create_game_window(&game_name, 720, 480);

    let renderer = renderer::create_renderer();

    let mut event_pump = game_window.sdl_context.event_pump().unwrap();
    let mut last_frame = Instant::now();
    'running: loop {

        let ok : bool = handle_events(&mut event_pump, &mut game_window, &renderer);
        if !ok { break 'running; }

        game_window.imgui_sdl2.prepare_frame(
            game_window.imgui.io_mut(),
            &game_window.sdl_window,
            &event_pump.mouse_state(),
        );

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        game_window.imgui.io_mut().delta_time = delta_s;

        let ui = game_window.imgui.frame();
        ui.show_demo_window(&mut true);

        renderer.render();

        game_window
            .imgui_sdl2
            .prepare_render(&ui, &game_window.sdl_window);
        game_window.renderer.render(ui);
        game_window.sdl_window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}
