
use std::{cell::RefCell, f32::NAN, rc::Rc};
use sauron::prelude::*;
use web_sys::KeyboardEvent;

use crate::logic::{
    constants::DEFAULT_PLAYER_COUNT, 
    game::{Game, Streets,  init_game},
    player::{Player, PlayerPosition},
    card::Card,
    round::{begin_round, next_turn},
};



struct Settings {
    player_count: usize,
}

struct Ui {
    main_menu: bool,
    pause: bool,
}

struct App {
    last_key: String,
    settings: Settings,
    game: Option<Game>,
    ui: Ui,
    end_round: bool,

}

impl App {
    fn new() -> Self {
        App { 
            last_key: "None".to_string(),
            settings: Settings {player_count: DEFAULT_PLAYER_COUNT},
            game: None,
            ui: Ui {main_menu: true, pause: true},
            end_round: false
        }
    }

    // Player node
    fn render_player(&self, player_name: &str, player_id: usize, card1: String, card2: String) -> Node<Msg> {
        // Calculate elliptical position
        let player_count = self.settings.player_count as f64;
        let angle = (player_id as f64 - 1.) * 2. * std::f64::consts::PI / player_count
        + (2. * std::f64::consts::PI) / 3.;
        
        // Velikost elipse
        let x_0 = 50.; // v % 
        let y_0 = 50.; // v % 
        let radius_x = 60.; // v %
        let radius_y = 40.; // v %
        
        let x = x_0 + radius_x * angle.cos();
        let y = y_0 + radius_y * angle.sin();
        
        // Pozicija 1. in 2. karte
        let style_card1 = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x - 5.), y
        );

        let style_card2 = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x + 2.), y
        );
        node! {
            <div class={format!("player-{}", player_id)}>
                <div style={style_card1}>
                    <img 
                    src={card1} 
                    alt={format!("Card for {}", player_name)}
                    height="150px"
                    />
                </div>
                <div style={style_card2}>
                    <img 
                    src={card2} 
                    alt={format!("Card for {}", player_name)}
                    height="150px"
                    />
                </div>
            </div>
        }
    }
    
    // Igralna miza
    fn render_game(&self) -> Node<Msg> {
        if let Some(ref game) = self.game {
            let players = &game.players;
            let mut hand_cards: Vec<((String, String))> = Vec::new();
            for i in 0..self.settings.player_count {
                if let Some(player) = players.get(i) {
                    let (card1, card2) = &player.hand_cards;
                    let hand = (card1.to_png(), card2.to_png());
                    hand_cards.push(hand);
                } else {panic!("Player not found (gui)")}
            };
            if self.end_round {
                node! {
                    <div class="game-table-container" 
                    style="position: relative; width: 50vw; height: 80vh; margin: 0 auto 50px auto;"
                    >
                        {
                            for i in 1..=self.settings.player_count {
                            let card_index = i - 1;
                            self.render_player(&format!("Player {}", i), i, 
                            hand_cards[card_index].0.clone(), hand_cards[card_index].1.clone())
                        }}
                    </div>
                }
            } else {
                node! {
                    <div class="game-table-container" 
                    style="position: relative; width: 50vw; height: 80vh; margin: 0 auto 50px auto;"
                    >
                        {self.render_player(&format!("Player {}", 1), 1, 
                            hand_cards[0].0.clone(), hand_cards[0].1.clone())
                        }
                        {for i in 2..=self.settings.player_count {
                            self.render_player(&format!("Player {}", i), i, 
                                "assets/cards/card_back.png".to_string(), "assets/cards/card_back.png".to_string())
                        }}
                    </div>
                }
            }
        } else {panic!("Game failed to render (gui)")}
    }

    // Začetni meni
    fn render_menu(&self) -> Node<Msg> {
        node! {
            <div>
                <div class="player-count-selection">
                    <input type="button"
                        value="+"
                        on_click=|_| Msg::PlayerCountUp
                    />
                    <button class="count" 
                        on_click=|_|{Msg::PlayerCountReset} >
                        {text(self.settings.player_count)}
                    </button>
                    <input type="button"
                        value="-"
                        on_click=|_| Msg::PlayerCountDown
                    />
                </div>
                <div>
                    <button class="start" 
                        on_click=|_|{Msg::StartGame} >
                        "Start Game"
                    </button>
                </div>
            </div>
        }
    }

    // Začni igro
    fn start_game(&mut self) -> () {
        let mut players= Player::init_players_with_count(self.settings.player_count);
        let deck = Card::make_ordered_deck();
        let deck = Card::scramble_deck(deck);
        self.game = Some(Game {
            street: Streets::PreFlop,
            pot: 0,
            players: players,
            deck,
            table_cards: Vec::new(),
            position_on_turn: PlayerPosition::UnderTheGun,
            round_number: 0,
            quit: false,
            player_count: self.settings.player_count.clone(), 
        });
        if let Some(ref mut game) = self.game {
            begin_round(game, self.settings.player_count);
        }
    }

}

enum Msg {
    Fold,
    Check,
    Raise,
    TogglePause,
    StartGame,
    PlayerCountUp,
    PlayerCountDown,
    PlayerCountReset,
    KeyPressed(String),
    None,
}



impl Application for App {
    type MSG = Msg;
    
    fn view(&self) -> Node<Msg> {
        node! {
            <div
                on_keydown={|event: KeyboardEvent| {
                    let key = event.key();
                    match key.as_str() {
                        "+" => Msg::PlayerCountUp,
                        "-" => Msg::PlayerCountDown,
                        "r" | "R" => Msg::PlayerCountReset,
                        "Enter" => Msg::StartGame,
                        _ => Msg::KeyPressed(key),
                    }
                }}
                tabindex="0"
                style="outline: none; width: 100%; height: 100%;"
                autofocus=true
            >
            <main>
                {
                    if self.ui.main_menu {
                        self.render_menu()
                    } else {
                        self.render_game()
                    }
                }
                <div class="key-display">
                    <p>{text(format!("Last key pressed: {}", self.last_key))}</p>
                    <p><small>"Keyboard shortcuts: 
                    + (PlayerCountUp), 
                    - (PlayerCountDown),
                    r/R (PlayerCountReset)"
                    </small></p>
                </div>
            </main>
            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::None => return Cmd::none(),
            Msg::PlayerCountUp => {
                if self.settings.player_count < 8 {self.settings.player_count += 1};
                self.last_key = "+ (PlayerCountUp)".to_string();
            },
            Msg::PlayerCountDown => {
                if self.settings.player_count > 2 {self.settings.player_count -= 1};
                self.last_key = "- (PlayerCountDown)".to_string();
            },
            Msg::PlayerCountReset => {
                self.settings.player_count = 6;
                self.last_key = "r (PlayerCountReset)".to_string();
            },
            Msg::KeyPressed(key) => {
                self.last_key = format!("{} (no action)", key);
            },
            Msg::StartGame => {
                self.ui.pause = false;
                self.ui.main_menu = false;
                self.last_key = "Enter (StartGame)".to_string();

                self.start_game();
            },
            _ => return Cmd::none(),
        }
        Cmd::none()
    }
    
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(App::new());
}
