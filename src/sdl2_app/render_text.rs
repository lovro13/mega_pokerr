use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use super::render_screen::get_screen_center;

pub fn draw_text(
    canvas: &mut WindowCanvas,
    string: &String,
    position: Rect,
    font: &sdl2::ttf::Font,
    text_color: Color,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let name_surface = font
        .render(&string)
        .blended(text_color)
        .map_err(|e| e.to_string())?;

    let text_texture = texture_creator
        .create_texture_from_surface(&name_surface)
        .map_err(|e| e.to_string())?;
    canvas.copy(&text_texture, None, position)?;
    Ok(())
}

pub fn write_info(canvas: &mut WindowCanvas, string: String, font: &sdl2::ttf::Font) -> Result<(), String> {
    let center = get_screen_center(&canvas);
    let pos = Point::new(center.x, center.y - 100);
    let rect = Rect::from_center(pos, 250, 60);
    println!("Rišem write_info");
    canvas.set_draw_color(Color::RGB(255, 102, 102));
    canvas.fill_rect(rect)?;
    draw_text(canvas, &string, rect, font, Color::RGB(0, 0, 0))?;
    Ok(())
}
