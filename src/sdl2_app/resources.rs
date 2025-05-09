use sdl2::{video::Window, render::Canvas, Sdl};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::image::Sdl2ImageContext;
use super::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

// Struktura, ki združi vse potrebne komponente


pub struct AppContext {
    pub sdl_context: Sdl,
    pub ttf_context: Sdl2TtfContext,
    pub image_context: Sdl2ImageContext,
    pub canvas: Canvas<Window>
}

pub fn init_app_context() -> Result<AppContext, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let image_context = sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG)?;

    let window = video_subsystem
        .window("POKEEEER", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    Ok(AppContext {
        sdl_context: sdl_context,
        ttf_context: ttf_context,
        image_context: image_context,
        canvas
    })
}
