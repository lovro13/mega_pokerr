use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    render::WindowCanvas,
};

use crate::logic::card::{Card, CardColor, CardNumber};
use crate::sdl2_app::positions::{CARD_HEIGHT, CARD_WIDTH};

impl Card {
    pub fn card_to_file(self: &Card) -> String {
        // sprejme karto in vrne ime fila
        let Card { color, number } = self;
        let string2 = match color {
            CardColor::Hearts => String::from("_of_hearts.png"),
            CardColor::Spades => String::from("_of_spades.png"),
            CardColor::Diamonds => String::from("_of_diamonds.png"),
            CardColor::Clubs => String::from("_of_clubs.png"),
            CardColor::Empty => String::from(""),
        };
        let string1 = match number {
            CardNumber::N2 => String::from("2"),
            CardNumber::N3 => String::from("3"),
            CardNumber::N4 => String::from("4"),
            CardNumber::N5 => String::from("5"),
            CardNumber::N6 => String::from("6"),
            CardNumber::N7 => String::from("7"),
            CardNumber::N8 => String::from("8"),
            CardNumber::N9 => String::from("9"),
            CardNumber::N10 => String::from("10"),
            CardNumber::NJ => String::from("jack"),
            CardNumber::NQ => String::from("queen"),
            CardNumber::NK => String::from("king"),
            CardNumber::NA => String::from("ace"),
            CardNumber::Empty => String::from("red_joker.png"),
        };
        String::from("assets/cards/") + &string1 + &string2
    }

    pub fn draw_card(
        &self,
        canvas: &mut WindowCanvas,
        position: Point,
        opened: bool,
        angle: f64,
        size: f64,
    ) -> Result<(), String> {
        let card_target = Rect::from_center(
            position,
            (CARD_WIDTH as f64 * size) as u32,
            (CARD_HEIGHT as f64 * size) as u32
        );
        let filename = if opened {
            self.card_to_file()
        } else {
            String::from("assets/cards/card_back.png")
        };
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(filename)?;
        canvas.copy_ex(&texture, None, card_target, angle, None, false, false)?;
        Ok(())
    }
}
