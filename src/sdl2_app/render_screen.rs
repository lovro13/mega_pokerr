use crate::logic::constants::{BIG_BLIND, SMALL_BLIND};
use crate::logic::game::Game;
use crate::logic::player::{self, Id, Player};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::sdl2_app::positions::{CARD_HEIGHT, CARD_WIDTH};
use crate::sdl2_app::render_text::draw_text;
use super::constants::PLAYER_INFO_FONT_SIZE;

const CARD2_POS: i32 = 30; // relative to first card, so same height but 30 pixels right, x

const BALANCE_POS: i32 = 50; // relative to player center, y
const BALANCE_WIDTH: u32 = 150;
const BALANCE_HEIGHT: u32 = 75;


const PLAYER_NAME_POS: (i32, i32) = (25, 85);
const PLAYER_NAME_WIDTH: u32 = 150;
const PLAYER_NAME_HEIGHT: u32 = 75;

pub fn get_screen_center(canvas: &WindowCanvas) -> Point {
    let (width, height) = canvas.output_size().unwrap();
    let screen_center = Point::new(width as i32 / 2, (height as i32) / 2 - 40);
    return screen_center;
}

pub fn render_player_info(
    canvas: &mut WindowCanvas,
    player: &player::Player,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
    color: Color,
) -> Result<(), String> {
    // nariše karte, ime, balance, dealer žeton, če je treba
    let player_center = player.id.get_player_screen_center(canvas);
    // tukaj je center v player_position z normalnim kartezičnim
    let card2_pos = player_center + Point::new(CARD2_POS, 0);
    if player.playing {
        player
            .hand_cards
            .0
            .draw_card(canvas, player_center, player.opened_cards)?;
        player
            .hand_cards
            .1
            .draw_card(canvas, card2_pos, player.opened_cards)?;
    }

    // write player name near the player cards
    let name_text = player::Player::player_id_to_str(player);

    let player_name_position = player_center + Point::from(PLAYER_NAME_POS);
    let text_target =
        Rect::from_center(player_name_position, PLAYER_NAME_WIDTH, PLAYER_NAME_HEIGHT);

    draw_text(
        canvas,
        &name_text,
        text_target,
        ttf_context,
        PLAYER_INFO_FONT_SIZE,
        color,
        None,
        false,
    )?;

    let balance_color = Color::RGB(0, 0, 10);
    let balance_text = format!("Chips: {}", player.chips);

    let balance_screen_position = player_name_position + Point::new(0, BALANCE_POS);
    let balance_text_target =
        Rect::from_center(balance_screen_position, BALANCE_WIDTH, BALANCE_HEIGHT);
    draw_text(
        canvas,
        &balance_text,
        balance_text_target,
        ttf_context,
        PLAYER_INFO_FONT_SIZE,
        balance_color,
        None,
        false,
    )?;

    let texture_creator = canvas.texture_creator();
    let texture_red = texture_creator.load_texture("assets/pokerchip_red.png")?;
    let texture_yellow = texture_creator.load_texture("assets/pokerchip_yellow.png")?;
    let texture_green = texture_creator.load_texture("assets/pokerchip_green.png")?;
    let texture_blue = texture_creator.load_texture("assets/pokerchip_blue.png")?;
    let mut balance_with_chips_pos = player_name_position + Point::new(-(CARD_WIDTH as i32), 0);
    let mut copy_balance = player.chips.clone() as i32;
    while copy_balance > 0 {
        let target = Rect::from_center(balance_with_chips_pos, 30, 30);
        if copy_balance >= 500 {
            canvas.copy(&texture_green, None, target)?;
            copy_balance -= 500;
        } else if copy_balance >= 100 {
            canvas.copy(&texture_yellow, None, target)?;
            copy_balance -= 100;
        } else if copy_balance >= BIG_BLIND as i32 {
            canvas.copy(&texture_red, None, target)?;
            copy_balance -= BIG_BLIND as i32;
        } else if copy_balance >= SMALL_BLIND as i32 {
            canvas.copy(&texture_blue, None, target)?;
            copy_balance -= SMALL_BLIND as i32;
        } else {
            break;
        }
        balance_with_chips_pos += Point::new(0, -10);
    }
    if !player.playing {
        let folded_color = Color::RGB(128, 128, 128);
        let folded_text = String::from("Folded");
        let folded_text_position = player_name_position + Point::new(0, -100);
        let folded_text_target = Rect::from_center(folded_text_position, 150 as u32, 50 as u32);
        draw_text(
            canvas,
            &folded_text,
            folded_text_target,
            ttf_context,
            PLAYER_INFO_FONT_SIZE,
            folded_color,
            None,
            false,
        )?;
    }
    let mut copy_curr_bet: i32 = player.current_bet.clone() as i32;
    let mut x_pos = -30;
    while copy_curr_bet > 0 {
        let curr_bet_pos = if vec![Id::Player1, Id::Player2, Id::Player8].contains(&player.id) {
            player_center + Point::new(x_pos, -(CARD_HEIGHT as i32) / 2 - 30)
        } else if vec![Id::Player4, Id::Player5, Id::Player6].contains(&player.id) {
            player_center - Point::new(x_pos, -(CARD_HEIGHT as i32) - 60)
        } else if player.id == Id::Player7 {
            player_center + Point::new(x_pos - 75, 70)
        } else {
            player_center + Point::new(x_pos + 160, 70)
        };
        let curr_bet_target = Rect::from_center(curr_bet_pos, 30, 30);
        if copy_curr_bet as u32 >= 100 {
            canvas.copy(&texture_yellow, None, curr_bet_target)?;
            copy_curr_bet -= 100 as i32;
        } else if copy_curr_bet as u32 >= BIG_BLIND {
            canvas.copy(&texture_red, None, curr_bet_target)?;
            copy_curr_bet -= BIG_BLIND as i32;
        } else if copy_curr_bet as u32 >= SMALL_BLIND {
            canvas.copy(&texture_blue, None, curr_bet_target)?;
            copy_curr_bet -= SMALL_BLIND as i32;
        } else {
            break;
        }
        x_pos += 10;
    }

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
    let player_center = player.id.get_player_screen_center(canvas);
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
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
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
            let player_name_position =
                player.id.get_player_screen_center(canvas) + Point::new(25, 85);
            let text_target = Rect::from_center(player_name_position, 150 as u32, 50 as u32);
            canvas.set_draw_color(background);
            canvas.fill_rect(text_target)?;
        }
        let _ = render_player_info(canvas, player, &ttf_context, color);
        // nariše karte, imena, balance
    }

    let screen_center = get_screen_center(canvas);
    let mut card_position = screen_center - Point::new((2.5 * CARD_WIDTH as f32) as i32, 0);
    for card in game.table_cards.iter() {
        card.draw_card(canvas, card_position, true)?;
        card_position.x += (CARD_WIDTH + 10) as i32;
    }
    Ok(())
}
