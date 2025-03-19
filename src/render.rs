use crate::card;
use crate::player;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub const CARD_HEIGHT: u32 = 120;
pub const CARD_WIDTH: u32 = 95;

pub fn render_text(
    canvas: &mut WindowCanvas,
    string: String,
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

struct Button {
    rect: Rect,
    text: String,
    is_clicked: bool
}

impl Button {
    // TODO
}

pub fn render_player_info(
    canvas: &mut WindowCanvas,
    player: &player::Player,
    font: &sdl2::ttf::Font,
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let screen_center = Point::new(width as i32 / 2, (height as i32) / 2 - 100); 
    // torej screen_center na nekak obrnjen kartezicni sistem, torej x normalen, y pa je obrnjen
    let card_position = player.card_position;
    let player_center = // is on the middle of the first card
        Point::new(card_position.0, -card_position.1) + screen_center;
        // tukaj je center v player_position z normalnim kartezičnim
    let card_target1 = Rect::from_center(player_center, card::CARD_WIDTH, card::CARD_HEIGHT);
    let screen_position2 = player_center + Point::new(30, 0);
    let card_target2 = Rect::from_center(screen_position2, card::CARD_WIDTH, card::CARD_HEIGHT);
    let texture_creator = canvas.texture_creator();
    let filename1 = card::Card::card_to_file(&player.cards.0);
    let filename2 = card::Card::card_to_file(&player.cards.1);
    let texture1 = texture_creator.load_texture(filename1)?;
    let texture2 = texture_creator.load_texture(filename2)?;
    canvas.copy(&texture1, None, card_target1)?;
    canvas.copy(&texture2, None, card_target2)?;

    // write player name near the player cards
    let text_color = Color::RGB(0, 0, 0);
    let name_text = player::Player::get_player_name(player);

    let player_name_position =
        player_center + Point::new(25, 85);
    let text_target = Rect::from_center(player_name_position, 150 as u32, 75 as u32);


    let _ = render_text(canvas, name_text, &text_target, font, text_color);

    let balance_color = Color::RGB(0, 0, 10);
    let balance_text = format!("Balance: {}", player.money);

    let balance_screen_position = player_name_position + Point::new(0, 50);
    let balance_text_target = Rect::from_center(balance_screen_position, 150 as u32, 75 as u32);
    let _ = render_text(canvas, balance_text, &balance_text_target, font, balance_color);

    if player.position == player::PlayerPosition::Dealer {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("assets/dealer_token.png")?;
        let screen_position4 = player_name_position + Point::new(95, -60);
        let screen_rect_dealer = Rect::from_center(screen_position4, 70, 70);
        canvas.copy(&texture, None, screen_rect_dealer)?;
    }
    Ok(())
}

