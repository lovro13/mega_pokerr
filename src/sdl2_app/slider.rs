use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use super::constants::SLIDER_FONT_SIZE;
use super::positions::ControlPosition;
use super::render_text::draw_text;

const SLIDER_HEIGHT: u32 = 30;
const SLIDER_WIDTH: u32 = 300;

pub struct Slider {
    pub track_rect: Rect,  // celotnega traku
    pub thumb_rect: Rect,  // pravoktnik k se premika
    pub min: i32,          // minimalna vrednost
    pub max: i32,          // maksimalna vrednost
    pub value: i32,        // trenutna vrednost
    pub is_dragging: bool, // ali se premika
}

impl Slider {
    pub fn new(center: Point, width: u32, height: u32, min: i32, max: i32) -> Self {
        let track_rect = Rect::new(center.x, center.y, width as u32, height as u32);
        let thumb_width = 20;

        let value = min;
        let total_range = (max - min) as f32;
        let thumb_x = center.x + ((value - min) as f32 / total_range * (width - thumb_width) as f32) as i32;
        
        let thumb_rect = Rect::new(
            thumb_x,
            center.y,
            thumb_width as u32,
            height as u32,
        );
        
        Slider {
            track_rect,
            thumb_rect,
            min,
            max,
            value,
            is_dragging: false,
        }
    }

    // Posodobi pozicijo drsaka glede na miško
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::MouseButtonDown { x, y, .. } => {
                let click_point = Point::new(*x, *y);
                if self.thumb_rect.contains_point(click_point) {
                    self.is_dragging = true;
                }
            }
            Event::MouseButtonUp { .. } => {
                self.is_dragging = false;
            }
            Event::MouseMotion { x, .. } if self.is_dragging => {
                // Popravljeno omejevanje glede na dejansko širino traku
                let min_value = self.track_rect.x().clone();
                let max_value = (self.track_rect.x() + self.track_rect.width() as i32
                    - self.thumb_rect.width() as i32)
                    .clone();
                let clamped_x = x.clamp(&min_value, &max_value);
                self.thumb_rect.set_x(*clamped_x);
                self.update_value();
            }
            _ => {}
        }
    }

    fn update_value(&mut self) {
        let relative_pos = self.thumb_rect.x() - self.track_rect.x();
        let total_range_pixels = self.track_rect.width() as i32 - self.thumb_rect.width() as i32;
        let value_range = self.max - self.min;

        self.value = self.min
            + ((relative_pos as f32 / total_range_pixels as f32) * value_range as f32) as i32;
    }
    // Risanje sliderja
    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ttf_context: &sdl2::ttf::Sdl2TtfContext,
    ) -> Result<(), String> {
        // Napiši število 123 nad levim robom self.track_rect
        let text_position = Point::new(self.track_rect.x() + 30, self.track_rect.y() - 20);
        let text_target = Rect::from_center(text_position, 60, 30);

        // Risanje traku
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        let value_text = self.value.to_string();
        draw_text(
            canvas,
            &value_text,
            text_target,
            ttf_context,
            SLIDER_FONT_SIZE,
            canvas.draw_color(),
            None,
            false,
        )?;
        canvas.fill_rect(self.track_rect)?;

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(self.thumb_rect)?;

        Ok(())
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
    pub fn init_raise_slider(canvas: &sdl2::render::Canvas<sdl2::video::Window>, min: i32, max: i32) -> Slider {
        let rect_pos = ControlPosition::init_control_positon(canvas).slider; 
        let slider = Slider::new(rect_pos, SLIDER_WIDTH, SLIDER_HEIGHT, min, max);
        // Tukaj lahko slider shranite v stanje aplikacije ali vrnete, odvisno od vaše arhitekture.
        // Primer: vrnite slider, če želite uporabiti to funkcijo za inicializacijo:
        slider
    }
}