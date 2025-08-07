use crate::logic::constants::*;
use crate::logic::game::Game;
use crate::logic::player::{self, Id, Player};
use sdl2::gfx::primitives::DrawRenderer;
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
    player_count: usize,
    is_on_turn: bool,
) -> Result<(), String> {
    // nariše karte, ime, balance, dealer žeton, če je treba
    let player_center = player
        .id
        .get_player_screen_center_for_count(canvas, player_count);

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

    // Use different background color if player is on turn
    let name_background = if is_on_turn {
        Some(Color::RGBA(255, 102, 102, 100)) // Light red with more opacity for turn indicator
    } else {
        Some(Color::RGBA(173, 216, 230, 25)) // Light blue, very transparent background
    };

    draw_text(
        canvas,
        &name_text,
        text_target,
        ttf_context,
        PLAYER_INFO_FONT_SIZE,
        color,
        name_background,
        false,
    )?;

    let balance_text = format!("{}{}", CHIPS_TEXT_PREFIX, player.chips);

    let balance_screen_position = player_name_position + Point::new(0, BALANCE_POS);
    let balance_text_target =
        Rect::from_center(balance_screen_position, BALANCE_WIDTH, BALANCE_HEIGHT);
    
    // Use same background logic for balance text
    let balance_background = if is_on_turn {
        Some(Color::RGBA(255, 102, 102, 100)) // Light red with more opacity for turn indicator
    } else {
        Some(Color::RGBA(173, 216, 230, 25)) // Light blue, very transparent background
    };
    
    draw_text(
        canvas,
        &balance_text,
        balance_text_target,
        ttf_context,
        PLAYER_INFO_FONT_SIZE,
        BALANCE_COLOR,
        balance_background,
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
    player_count: usize,
) -> Result<(), String> {
    render_background(canvas)?;
    let players_list = &game.players;
    for player in players_list {
        //naprinta ime in karte igralca
        // let _ = player::Player::render_player_info(canvas, player, font);
        let color = Color::RGB(0, 0, 0);
        let is_on_turn = player.position == game.position_on_turn;
        let _ = render_player_info(canvas, player, &ttf_context, color, player_count, is_on_turn);
        // nariše karte, imena, balance
    }

    let screen_center = get_screen_center(canvas);
    let mut card_position = screen_center - Point::new((2.5 * CARD_WIDTH as f32) as i32, 0);
    for card in game.table_cards.iter() {
        card.draw_card(canvas, card_position, true, 0., 1.)?;
        card_position.x += (CARD_WIDTH + 10) as i32;
    }

    let pot_text = format!("Pot size : {}", game.pot);
    pub const POT_CENTER: (i32, i32) = (0, -150);
    let pot_text_pos = Rect::from_center(
        Point::from(POT_CENTER) + get_screen_center(canvas),
        200,
        200,
    );
    draw_text(
        canvas,
        &pot_text,
        pot_text_pos,
        ttf_context,
        BUTTON_FONT_SIZE,
        BLACK,
        None,
        false,
    )?;

    // Display all players' debts in top right corner
    let (screen_width, _) = canvas.output_size()?;
    let mut debt_y_offset = 20;
    let debt_panel_width = 180; // Consistent width for all elements
    let debt_panel_x = screen_width as i32 - debt_panel_width - 10; // 10px margin from edge
    
    // Calculate total height needed for the debt panel
    let total_height = 30 + 40 + (game.players.len() * 35); // title + spacing + players
    
    // Draw single background for entire debt panel
    let panel_background = Rect::new(
        debt_panel_x - 5, // Slightly larger for padding
        debt_y_offset - 5,
        (debt_panel_width + 10) as u32,
        total_height as u32
    );
    canvas.set_draw_color(Color::RGBA(255, 255, 255, 180));
    canvas.fill_rect(panel_background)?;
    
    // Draw title "Debts:"
    let title_text = "Debts:";
    let title_pos = Rect::new(
        debt_panel_x,
        debt_y_offset,
        debt_panel_width as u32,
        30
    );
    
    draw_text(
        canvas,
        title_text,
        title_pos,
        ttf_context,
        30, // Slightly larger font for title
        Color::RGB(0, 0, 0), // Black color for title
        None, // No individual background
        true, // Use custom background
    )?;
    
    debt_y_offset += 40; // Move down after title
    
    for player in &game.players {
        let debt_text = format!("{}: {}", player::Player::player_id_to_str(player), player.debt);
        let debt_pos = Rect::new(
            debt_panel_x,
            debt_y_offset,
            debt_panel_width as u32,
            30
        );
        
        let debt_color = if player.debt > 0 {
            Color::RGB(128, 0, 0) // Dark red for players with debt
        } else {
            Color::RGB(0, 128, 0) // Dark green for players with no debt
        };
        
        draw_text(
            canvas,
            &debt_text,
            debt_pos,
            ttf_context,
            25, // Smaller font size for corner display
            debt_color,
            None, // No individual background
            true, // Use custom background
        )?;
        
        debt_y_offset += 35; // Move down for next player
    }

    Ok(())
}

pub fn render_background(canvas: &mut WindowCanvas) -> Result<(), String> {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let center = get_screen_center(canvas);

    let radius = canvas.output_size()?;

    let base_radius_x = (radius.0 / 3) as i16;
    let base_radius_y = (radius.1 / 3) as i16;


    canvas.filled_ellipse(
        center.x as i16,
        center.y as i16,
        base_radius_x + TABLE_BORDER_SIZE,
        base_radius_y + TABLE_BORDER_SIZE,
        BROWN,
    )?;


    canvas.filled_ellipse(
        center.x as i16,
        center.y as i16,
        base_radius_x,
        base_radius_y,
        LIGHT_GREEN,
    )?;

    Ok(())
}
