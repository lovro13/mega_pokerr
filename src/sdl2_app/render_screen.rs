use crate::logic::game::Game;
use crate::logic::player::{self, Player};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::sdl2_app::constants::CARD_WIDTH;
use crate::sdl2_app::render_text::draw_text;


pub fn get_screen_center(canvas: &WindowCanvas) -> Point {
    let (width, height) = canvas.output_size().unwrap();
    let screen_center = Point::new(width as i32 / 2, (height as i32) / 2 - 40);
    return screen_center;
}

impl Player {
    pub fn get_player_screen_center(&self, canvas: &WindowCanvas) -> Point {
        // is on the middle of the first card
        let (width, height) = canvas.output_size().unwrap();
        let screen_center = Point::new(width as i32 / 2, (height as i32) / 2 - 100);
        let card_position = self.card_position;
        let player_center = // is on the middle of the first card
        Point::new(card_position.0, -card_position.1) + screen_center;
        return player_center;
    }
}


pub fn render_player_info(
    canvas: &mut WindowCanvas,
    player: &player::Player,
    font: &sdl2::ttf::Font,
    color: Color
) -> Result<(), String> {
    // nariše karte, ime, balance, dealer žeton, če je treba
    let player_center = player.get_player_screen_center(canvas);
    // tukaj je center v player_position z normalnim kartezičnim
    let card2_pos = player_center + Point::new(30, 0);
    if player.playing {
        player.hand_cards.0.draw_card(canvas, player_center)?;
        player.hand_cards.1.draw_card(canvas, card2_pos)?;
    }

    // write player name near the player cards
    let name_text = player::Player::get_player_name(player);

    let player_name_position = player_center + Point::new(25, 85);
    let text_target = Rect::from_center(player_name_position, 150 as u32, 75 as u32);

    draw_text(canvas, &name_text, text_target, font, color)?;

    let balance_color = Color::RGB(0, 0, 10);
    let balance_text = format!("Balance: {}", player.chips);

    let balance_screen_position = player_name_position + Point::new(0, 50);
    let balance_text_target = Rect::from_center(balance_screen_position, 150 as u32, 75 as u32);
    let _ = draw_text(
        canvas,
        &balance_text,
        balance_text_target,
        font,
        balance_color,
    );

    if player.position == player::PlayerPosition::Dealer {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("assets/dealer_token.png")?;
        let screen_position4 = player_name_position + Point::new(95, -60);
        let screen_rect_dealer = Rect::from_center(screen_position4, 70, 70);
        canvas.copy(&texture, None, screen_rect_dealer)?;
    }

    Ok(())
}

pub fn render_turn_indicator(player: &Player, canvas: &mut WindowCanvas) -> Result<(), String> {
    let player_center = player.get_player_screen_center(canvas);
    let player_name_position = player_center + Point::new(25, 85);
    let indincator_target = player_name_position + Point::new(0, 80);
    let target = Rect::from_center(indincator_target, 150, 10);
    canvas.set_draw_color(Color::RGB(200, 0, 0));
    canvas.fill_rect(target)?;
    Ok(())
}

pub fn render_screen(
    canvas: &mut WindowCanvas,
    background_color: Color,
    game: &Game, // tega tudi mogoče dobi iz player lista
    font: &sdl2::ttf::Font,
) -> Result<(), String> {
    canvas.set_draw_color(background_color);
    canvas.clear();
    let players_list = &game.players;
    for player in players_list {
        //naprinta ime in karte igralca
        // let _ = player::Player::render_player_info(canvas, player, font);
        let color = Color::RGB(0, 0, 0);
        if player.position == game.position_on_turn {
            let background = Color::RGB(255, 105, 105);
            let player_name_position = player.get_player_screen_center(canvas) + Point::new(25, 85);
            let text_target = Rect::from_center(player_name_position, 150 as u32, 50 as u32);
            canvas.set_draw_color(background);
            canvas.fill_rect(text_target)?;
        }
        let _ = render_player_info(canvas, player, font, color);
        // nariše karte, imena, balance
    }

    let screen_center = get_screen_center(canvas);
    let mut card_position = screen_center - Point::new((2.5 * CARD_WIDTH as f32) as i32, 0);
    for card in game.board_cards.iter() {
        card.draw_card(canvas, card_position)?;
        card_position.x += (CARD_WIDTH + 10) as i32;        
    }
    Ok(())
}
