use crate::logic::constants::*;
use crate::logic::game::Game;
use crate::logic::player::{self, Id, Player};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use super::constants::*;

use crate::sdl2_app::render_text::draw_text;

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
            .draw_card(canvas, player_center, player.opened_cards, 0., 1.)?;
        player
            .hand_cards
            .1
            .draw_card(canvas, card2_pos, player.opened_cards, 0., 1.)?;
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

    let balance_text = format!("{}{}", CHIPS_TEXT_PREFIX, player.chips);

    let balance_screen_position = player_name_position + Point::new(0, BALANCE_POS);
    let balance_text_target =
        Rect::from_center(balance_screen_position, BALANCE_WIDTH, BALANCE_HEIGHT);
    draw_text(
        canvas,
        &balance_text,
        balance_text_target,
        ttf_context,
        PLAYER_INFO_FONT_SIZE,
        BALANCE_COLOR,
        None,
        false,
    )?;

    let texture_creator = canvas.texture_creator();
    let texture_red = texture_creator.load_texture(PATH_TO_POKERCHIP_RED)?;
    let texture_yellow = texture_creator.load_texture(PATH_TO_POKERCHIP_YELLOW)?;
    let texture_green = texture_creator.load_texture(PATH_TO_POKERCHIP_GREEN)?;
    let texture_blue = texture_creator.load_texture(PATH_TO_POKERCHIP_BLUE)?;
    let mut balance_with_chips_pos = player_name_position + Point::new(-(CARD_WIDTH as i32), 0);
    let mut copy_balance = player.chips.clone() as i32;
    while copy_balance > 0 {
        let target = Rect::from_center(balance_with_chips_pos, 30, 30);
        if copy_balance >= CHIP_DENOMINATION_LARGE as i32 {
            canvas.copy(&texture_green, None, target)?;
            copy_balance -= CHIP_DENOMINATION_LARGE as i32;
        } else if copy_balance >= CHIP_DENOMINATION_SMALL as i32 {
            canvas.copy(&texture_yellow, None, target)?;
            copy_balance -= CHIP_DENOMINATION_SMALL as i32;
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
        let folded_text = String::from(FOLDED_TEXT);
        let folded_text_position = player_name_position + Point::new(0, FOLDED_TEXT_Y_OFFSET);
        let folded_text_target = Rect::from_center(folded_text_position, 150 as u32, 50 as u32);
        draw_text(
            canvas,
            &folded_text,
            folded_text_target,
            ttf_context,
            PLAYER_INFO_FONT_SIZE,
            FOLDED_COLOR,
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
            player_center + Point::new(x_pos + BET_DISPLAY_X_OFFSET, BET_DISPLAY_Y_OFFSET)
        };
        let curr_bet_target = Rect::from_center(curr_bet_pos, 30, 30);
        if copy_curr_bet as u32 >= CHIP_DENOMINATION_SMALL {
            canvas.copy(&texture_yellow, None, curr_bet_target)?;
            copy_curr_bet -= CHIP_DENOMINATION_SMALL as i32;
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
        let texture = texture_creator.load_texture(PATH_TO_DEALER_TOKEN)?;
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
    canvas.set_draw_color(RED_COLOR);
    canvas.fill_rect(target)?;
    Ok(())
}

pub fn render_screen(
    canvas: &mut WindowCanvas,
    game: &Game, // tega tudi mogoče dobi iz player lista
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) -> Result<(), String> {
    render_background(canvas);
    let players_list = &game.players;
    for player in players_list {
        //naprinta ime in karte igralca
        // let _ = player::Player::render_player_info(canvas, player, font);
        let color = Color::RGB(0, 0, 0);
        if player.position == game.position_on_turn {
            let background = LIGHT_RED;
            let player_name_position =
                player.id.get_player_screen_center(canvas) + Point::new(25, 85);
            let text_target = Rect::from_center(player_name_position, PLAYER_NAME_WIDTH, PLAYER_NAME_HEIGHT);
            canvas.set_draw_color(background);
            canvas.fill_rect(text_target)?;
        }
        let _ = render_player_info(canvas, player, &ttf_context, color);
        // nariše karte, imena, balance
    }

    let screen_center = get_screen_center(canvas);
    let mut card_position = screen_center - Point::new((2.5 * CARD_WIDTH as f32) as i32, 0);
    for card in game.table_cards.iter() {
        card.draw_card(canvas, card_position, true, 0., 1.)?;
        card_position.x += (CARD_WIDTH + 10) as i32;
    }
    Ok(())
}

pub fn render_background(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
    let (width, height) = canvas.output_size().unwrap();
    let center = Point::new(width as i32 / 2, height as i32 / 2);
    let table_width = (width as f32 * 0.85) as i32;
    let table_height = (height as f32 * 0.45) as i32;

    // Draw table felt (ellipse)
    let mut pixels = vec![];
    let rx = table_width / 2;
    let ry = table_height / 2;
    for y in -ry..=ry {
        for x in -rx..=rx {
            if ((x * x * ry * ry) + (y * y * rx * rx)) <= (rx * rx * ry * ry) {
                pixels.push((center.x + x, center.y + y));
            }
        }
    }
    canvas.set_draw_color(Color::RGB(34, 139, 34)); // green felt
    for (x, y) in pixels {
        let _ = canvas.draw_point(Point::new(x, y));
    }

    // Draw table border (ellipse outline)
    canvas.set_draw_color(Color::RGB(139, 69, 19)); // brown border
    let border_thickness = 12;
    for t in 0..border_thickness {
        let rxo = rx + t;
        let ryo = ry + t;
        for deg in 0..360 {
            let rad = (deg as f32).to_radians();
            let x = (rxo as f32 * rad.cos()) as i32;
            let y = (ryo as f32 * rad.sin()) as i32;
            let _ = canvas.draw_point(center + Point::new(x, y));
        }
    }

    // Optional: Draw a subtle highlight
    canvas.set_draw_color(Color::RGBA(255, 255, 255, 30));
    for t in 0..4 {
        let rxo = rx - t;
        let ryo = (ry as f32 * 0.7) as i32 - t;
        for deg in 30..150 {
            let rad = (deg as f32).to_radians();
            let x = (rxo as f32 * rad.cos()) as i32;
            let y = (ryo as f32 * rad.sin()) as i32 - ry / 3;
            let _ = canvas.draw_point(center + Point::new(x, y));
        }
    }
}
