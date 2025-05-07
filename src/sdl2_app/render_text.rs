use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn draw_text(
    canvas: &mut WindowCanvas,
    string: &String,
    position: &Rect,
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
    canvas.copy(&text_texture, None, Some(*position))?;
    Ok(())
}
