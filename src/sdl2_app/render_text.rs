use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use super::constants::INFO_B_COLOR;
use super::render_screen::get_screen_center;

pub fn draw_text(
    canvas: &mut WindowCanvas,
    text: &str,
    position: Rect, // Ta Rect definira območje (x, y, width, height) kjer naj bo besedilo centrirano
    font: &sdl2::ttf::Font,
    text_color: Color,
    background: Option<Color>,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    
    // Ustvari površino s tekstom
    let text_surface = font
        .render(text)
        .blended(text_color)
        .map_err(|e| e.to_string())?;

    // Dobi dimenzije besedila
    let (text_width, text_height) = (text_surface.width(), text_surface.height());

    // Izračunaj pozicijo za centriranje
    let x = position.x() + (position.width() as i32 - text_width as i32) / 2;
    let y = position.y() + (position.height() as i32 - text_height as i32) / 2;
    let dest_rect = Rect::new(x, y, text_width, text_height);

    // Nariši ozadje (če je podano)
    if let Some(bg_color) = background {
        canvas.set_draw_color(bg_color);
        canvas.fill_rect(dest_rect)?; // Celoten podani pravokotnik
    }

    // Ustvari teksturo in nariši centrirano besedilo
    let text_texture = texture_creator
        .create_texture_from_surface(&text_surface)
        .map_err(|e| e.to_string())?;
    
    canvas.copy(&text_texture, None, dest_rect)?;
    Ok(())
}

pub fn write_info(
    canvas: &mut WindowCanvas,
    string: &String,
    font: &sdl2::ttf::Font,
    size: u32,
) -> Result<(), String> {
    let center = get_screen_center(&canvas);
    let pos = Point::new(center.x, center.y - 100);
    let rect = Rect::from_center(pos, size, 60);
    canvas.set_draw_color(Color::RGB(255, 102, 102));
    canvas.fill_rect(rect)?;
    draw_text(
        canvas,
        &string,
        rect,
        font,
        Color::RGB(0, 0, 0),
        Some(Color::RGB(INFO_B_COLOR.0, INFO_B_COLOR.1, INFO_B_COLOR.2)),
    )?;
    Ok(())
}
