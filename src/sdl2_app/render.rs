use crate::logic::card;
use crate::logic::player;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

pub const CARD_HEIGHT: u32 = 120;
pub const CARD_WIDTH: u32 = 95;
pub const FOLD_BUTTON: (i32, i32) = (0, 0);


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

pub struct Button {
    rect: Rect,
    text: String,
    is_clicked: bool,
}

impl Button {
    pub fn new(center: &Point, height: u32, width: u32, text: String) -> Self {
        Button {
            rect: Rect::from_center(*center, width, height),
            text,
            is_clicked: false,
        }
    }

    pub fn is_hovered(&self, mouse_x: i32, mouse_y: i32) -> bool {
        self.rect.contains_point(Point::new(mouse_x, mouse_y))
    }

    pub fn draw_button(&self, canvas: &mut sdl2::render::WindowCanvas, font: &sdl2::ttf::Font) -> Result<(), String> {
        let color = if self.is_clicked {
            Color::RGB(100, 100, 100)
        } else {
            Color::RGB(200, 200, 200)
        };

        let text_color = Color::RGB(0, 0, 0);

        canvas.set_draw_color(color);
        canvas.fill_rect(self.rect)?;
        draw_text(canvas, &self.text, &self.rect, font, text_color)?;
        Ok(())
    }

    pub fn handle_button_events(event: &Event, button: &mut Button) {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if button.is_hovered(*x, *y) {
                    button.is_clicked = true;
                    println!("Gumb je bil kliknjen!");
                }
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                button.is_clicked = false;
            }
            _ => {}
        }
    }


    // soon this will make a list of all buttons [FOLD; CALL; RAISE]
    pub fn init_fold_button(canvas: &mut WindowCanvas) ->  Self {
        let (width, heigth) = canvas.output_size().unwrap();
        let screen_center = Point::new((width as i32) / 2, (heigth as i32) / 2 + 100);
        let button_position = screen_center + Point::new(player::Player::PLAYER1_CARDS.0, -player::Player::PLAYER1_CARDS.1)
        + Point::new(0, 0);
        let button_target = Rect::from_center(button_position, 100, 50);
        Button {
            rect: button_target,
            text: String::from("FOLD"),
            is_clicked: false
        }
    }
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
    let card_target1 = Rect::from_center(player_center, CARD_WIDTH, CARD_HEIGHT);
    let screen_position2 = player_center + Point::new(30, 0);
    let card_target2 = Rect::from_center(screen_position2, CARD_WIDTH, CARD_HEIGHT);
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

    let player_name_position = player_center + Point::new(25, 85);
    let text_target = Rect::from_center(player_name_position, 150 as u32, 75 as u32);

    let _ = draw_text(canvas, &name_text, &text_target, font, text_color);

    let balance_color = Color::RGB(0, 0, 10);
    let balance_text = format!("Balance: {}", player.money);

    let balance_screen_position = player_name_position + Point::new(0, 50);
    let balance_text_target = Rect::from_center(balance_screen_position, 150 as u32, 75 as u32);
    let _ = draw_text(
        canvas,
        &balance_text,
        &balance_text_target,
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

    // let test_button = Button::new(0, 0, 50, 50, String::from("test"));
    // Button::draw_button(&test_button, canvas, font)?;

    Ok(())
}
