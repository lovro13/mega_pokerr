
use sauron::js_sys::Array;
use sauron::prelude::*;
use sauron::web_sys::console;
use web_sys::KeyboardEvent;

use crate::logic::choose_winner::choose_winner;
use crate::logic::constants::{BIG_BLIND,};
use crate::logic::{
    card::{Card, CardColor, CardNumber}, 
    constants::DEFAULT_PLAYER_COUNT, 
    game::{Game, Streets}, 
    player::{self, Player, PlayerPosition}, 
    round::{begin_round, next_turn}
};
use crate::sauron_app::constants::{DEBUG_VIEW, DEFAULT_ANIMATION_TIME, DEFAULT_RAISE};
use crate::sauron_app::{
    make_bets::{Response, make_bets, active_bet, fold_bet},
    ai::{get_bet,},
    };

struct Settings {
    player_count: usize,
    animation_speed: u32,
}

struct Ui {
    main_menu: bool,
    pause: bool,
    waiting_input: bool,
    waiting_new_round: bool,
}

#[derive(Clone, Debug)]
enum Animated {
    Bet(Response),
    EndRound
}

struct Animation {
    css_class: String,
    message: String,
    animation_type: Animated,
    toggle: bool, 
}

struct App {
    last_key: String,
    settings: Settings,
    game: Option<Game>,
    ui: Ui,
    animations: Animation,
    req_bet: u32,
    raise: u32,
    debug: String,
}

impl App {
    fn new() -> Self {
        App { 
            last_key: "None".to_string(),
            settings: Settings {
                player_count: DEFAULT_PLAYER_COUNT, 
                animation_speed: DEFAULT_ANIMATION_TIME},
            game: None,
            ui: Ui {
                main_menu: true, 
                pause: false, 
                waiting_input: false, 
                waiting_new_round: false},
            animations: Animation { 
                css_class: String::new(), 
                animation_type: Animated::EndRound, 
                message: String::new(), 
                toggle: false },
            req_bet: 0,
            raise: DEFAULT_RAISE,
            debug: String::new(),
        }
    }

    // Player 1 controls
    fn player_input(&self) -> Node<Msg> {
        node!{
            <div class="player-input-container">
                <button class="fold-btn" 
                    on_click=|_|{Msg::Fold}>
                    "Fold [F]"
                </button>
                <button class="call-btn" 
                    on_click=|_|{Msg::Call}>
                    "Call [C]"
                </button>
                <button class="raise-btn" 
                    on_click=|_|{Msg::Raise}>
                    {text(format!("Raise {} [V]", self.raise))}
                </button>
                <div class="raise-controls">
                    <input type="button"
                        value="+"
                        on_click=|_| Msg::PlayerCountUp
                    />
                    <input type="button"
                        value="-"
                        on_click=|_| Msg::PlayerCountDown
                    />
                </div>
            </div>
        }
    }

    // Player node
    fn render_player(&self, player_id: usize) -> Node<Msg> {
        let player = match self.game {
            None => {
                console::log(&Array::of1(&JsValue::from_str("Game failed to render (gui - render_player)")));
                &Player::new()
            },
            Some(ref game) => {
                match game.players.get(player_id - 1) {
                    None => {
                        console::log(&Array::of1(&JsValue::from_str("Game failed to render (gui - render_player)")));
                        &Player::new()
                    },
                    Some(player) => player
                }
            }
        };
        let player_on_turn = match &self.game {
            None => {
                console::log(&Array::of1(&JsValue::from_str("Game failed to render (gui - render_player)")));
                &Player::new()
            },
            Some(game) => game.player_on_turn_immutable()
        };
        let (card1_a, card2_a) = &player.hand_cards;
        let (card1, card2) = if self.ui.waiting_new_round || (player_id == 1)  {
            (card1_a.to_png(), card2_a.to_png())
        } else {("assets/cards/card_back.png".to_string(), "assets/cards/card_back.png".to_string())
        }; 
        let player_name = format!("Player {}", player_id);

        // Calculate elliptical position
        let player_count = self.settings.player_count as f64;
        let angle = (player_id as f64 - 1.) * 2. * std::f64::consts::PI / player_count
        + (2. * std::f64::consts::PI) / 3.;
        
        // Velikost elipse %
        let x_0 = 50.;  
        let y_0 = 50.;
        let radius_x = 60.;
        let radius_y = 40.;
        
        let x = x_0 + radius_x * angle.cos();
        let y = y_0 + radius_y * angle.sin();
        
        // Pozicija 1. in 2. karte, besedila, kovanca
        let style_card1 = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x - 3.), y
        );
        let style_card2 = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x + 2.), y
        );
        let style_text = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);
            background: {};",
            (x - 4.), (y + 15.), {
                if player_on_turn.id == player.id {"#59f566ef"} else {"#b8a5a5ef"}
            }
        );
        let style_token = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x - 15.), (y)
        );

        node! {
            <div class={format!("player-{}", player_id)}>
                {if player.playing {
                    node! {
                        <div class="player-hand">
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
                } else {
                    node! {
                        <div class="folded"
                        style={style_card1}>
                            <p>
                                "Folded"
                            </p>
                        </div>
                    }  
                }}
                <div class="player-info-text"
                style={style_text}>
                    <p>
                    {text(player_name)}
                    </p>
                    <p>
                    {text(format!("Chips: {}", player.chips))}   
                    </p>
                    {
                        if DEBUG_VIEW {
                            node! {
                                <p>
                                {text(format!("Current bet: {}", player.current_bet))}   
                                </p>
                                <p>
                                {text(format!("Possition: {}", player.position.eval_to_int()))}   
                                </p>
                            } 
                        } else {node! {<div></div>}}
                    }
                </div>
                <div style={style_token}>
                    {if player.position == PlayerPosition::Dealer {
                        node! {
                            <img 
                            src="assets/dealer_token.png"
                            alt="Dealer token"
                            height="50px"
                            />
                        }
                    } else {node! {<div></div>}
                    }}
                </div>
            </div>
        }
    }

    // Karte na mizi
    fn render_table_card(&self, card: &Card, i: usize) -> Node<Msg> {
        let style = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (120 + i*30), 50
        );
        node! {
        <div style={style}>
            <img 
            src={card.to_png()} 
            alt="table card"
            height="150px"
            />
        </div>
        }
    }
    
    // Igralna miza
    fn render_game(&self) -> Node<Msg> { 
        let table_cards = match self.game {
            None => {
                console::log(&Array::of1(&JsValue::from_str("Game failed to render (gui - render_player)")));
                &Vec::new()
            },
            Some(ref game) => &game.table_cards};
        let game_pot = match self.game {
            None => {
                console::log(&Array::of1(&JsValue::from_str("Game failed to render (gui - render_player)")));
                &0
            },
            Some(ref game) => &game.pot};
        let mut i = 0;
        node! {
            <div class="game-table-container" 
            style="position: relative; width: 50vw; height: 80vh; margin: 0 auto 50px auto;"
            >
                { // igralci
                    for i in 1..=self.settings.player_count {
                    self.render_player(i)
                }}
                <div class="player-info-text"
                style="position: absolute; left: 50%; top: 35%; transform: translate(-50%, -50%);">
                    <p>{text(format!("Game pot: {}", game_pot))}</p>
                </div>
                <div style="position: absolute; left: 30%; top: 50%; transform: translate(-50%, -50%);">
                    <img 
                        src="assets/cards/card_back.png" 
                        alt="card deck"
                        height="150px"
                    />
                    {for card in table_cards {
                        i += 1;
                        self.render_table_card(card, i)
                    }}
                </div>
                <div class="animation-container">
                    {if !self.animations.css_class.is_empty() {node!{
                        <div class={format!("animation-overlay {}", self.animations.css_class)}
                            on_animationend=|_| {
                                web_sys::console::log_1(&"Animation ended".into());
                                Msg::EndAnimation
                            }
                        >
                        <p>{text(self.animations.message.clone())}</p>
                    </div>}} 
                    else {node!{<div></div>}}
                    }
                    {if self.ui.waiting_new_round {node! { <div 
                        class="End-round-msg"
                        style="position: absolute; 
                        left: 50%; 
                        top: 70%; 
                        transform: translate(-50%, -50%);
                        background-color: green;    
                        padding: 10px;
                        border-radius: 8px;
                        color: white;
                        font-weight: bold;"
                        >
                            <p>{text(self.animations.message.clone())}</p>
                            <input type="button"
                                class="new-round-btn"
                                value="New Round"
                                on_click=|_| Msg::NewRound
                                title="New Round"
                            />
                        </div>
                    }} else {node!{<div></div>}}
                    }
                </div>
            </div>
            {self.player_input()}
        }
    }

    // Začetni meni
    fn render_menu(&self) -> Node<Msg> {
        node! {
            <div class="main-menu">
                <h1>"MEGA POKER"</h1>
                <div class="player-count">
                    <p>"Select number of players:"</p>
                    <div style="display: flex; height: 50px; margin-top: 10px;">
                        <div class="player-count-selection">
                            <input type="button"
                                class="decrement-btn"
                                value="-"
                                on_click=|_| Msg::PlayerCountDown
                                title="Decrease player count"
                            />
                            <button class="count-btn" 
                                on_click=|_| Msg::PlayerCountReset>
                                {text(self.settings.player_count)}
                            </button>
                            <input type="button"
                                class="increment-btn"
                                value="+"
                                on_click=|_| Msg::PlayerCountUp
                                title="Increase player count"
                            />
                        </div>
                    </div>
                </div>
                <div class="menu-actions">
                    <button class="start" 
                        on_click=|_| Msg::Confirm
                        title="Start the game">
                        "Start Game"
                    </button>
                </div>
            </div>
        }
    }

    // Začni igro
    fn start_game(&mut self) {
        let players= Player::init_players_with_count(self.settings.player_count);
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
        self.start_round();
    }

    // Začni krog
    fn start_round(&mut self) {
        self.debug = "start round".to_string();
        let response = match self.game {
            None => {
                console::log(&Array::of1(&JsValue::from_str("Faild to start game - no game exists(gui - start_game)")));
                Response::EndRound
            },
            Some(ref mut game) => {
                begin_round(game, self.settings.player_count);
                make_bets(game, BIG_BLIND, get_bet)
                }
            };
        self.animate(Animated::Bet(response));
    }

    // animacija stave
    fn animate(&mut self, animation_type: Animated) {
        match animation_type {
            Animated::Bet(response) => {
                self.animations.animation_type = Animated::Bet(response.clone());
                match response.clone() {
                    Response::WaitingPlayer1(req_bet) => {
                        console::log(&Array::of1(&JsValue::from_str("Waiting player 1")));
                        self.req_bet = req_bet;
                        self.update(Msg::BetResponse(Response::WaitingPlayer1(req_bet)));
                    },
                    Response::OnePlayerRemaning => {
                        console::log(&Array::of1(&JsValue::from_str("One player left")));
                        self.animate(Animated::EndRound);
                    },
                    Response::BetPlaced(id, req_bet, decision ) => {
                        console::log(&Array::of1(&JsValue::from_str("Bet placed")));
                        self.debug = "bet placed".to_string();
                        self.req_bet = req_bet;
                        let animating_player = id.to_str();
                        let decision_text = match decision {
                            None => {String::from("folded")},
                            Some(0) => {String::from("called")},
                            Some(x) => {format!("raised by {}", x)},
                        };
                        self.animations.message = format!("{} {}", animating_player, decision_text);
                        self.animations.toggle = !self.animations.toggle;
                        self.animations.css_class = if self.animations.toggle {
                            "bet-animation-a".to_string()
                        } else {
                            "bet-animation-b".to_string()
                        };
                    },
                    Response::StreetFinished(id, decision) => {
                        console::log(&Array::of1(&JsValue::from_str("Street finished")));
                        let animating_player = id.to_str();
                        let decision_text = match decision {
                            None => {String::from("folded")},
                            Some(0) => {String::from("called")},
                            Some(x) => {format!("raised by {}", x)},
                        };
                        self.animations.message = format!("{} {}", animating_player, decision_text);
                        self.animations.toggle = !self.animations.toggle;
                        self.animations.css_class = if self.animations.toggle {
                            "bet-animation-a".to_string()
                        } else {
                            "bet-animation-b".to_string()
                        };
                    },
                    Response::EndRound => {
                        console::log(&Array::of1(&JsValue::from_str("End round")));
                        self.animate(Animated::EndRound);
                    }
                }
            },
            Animated::EndRound => {
                self.ui.waiting_new_round = true;
                self.animations.animation_type = Animated::EndRound;
                self.req_bet = 0;
                let winners_str = match self.game {
                    None => {
                        console::log(&Array::of1(&JsValue::from_str("Game not found (gui - EndRound)")));
                        "error".to_string()
                    },
                    Some(ref mut game) => {
                        let pot = game.pot;
                        let winners ={
                            let active_players: Vec<_> = 
                                game.players.iter_mut().filter(|p| p.playing).collect();
                            match active_players.len() {
                                0 => {
                                    console::log(&Array::of1(&JsValue::from_str("Error: no active players at EndRound")));
                                    vec![]
                                },
                                1 => active_players,
                                _ => choose_winner(game)
                            }
                        }; 
                        let winner_names = Vec::from_iter(winners.iter().map(|player| player.id.to_str()));
                        let n = winners.len() as u32;
                        for player in winners {
                            player.chips += pot/n;
                        };
                        //game.pot = 0;
                        match winner_names.len() {
                            0 => {
                                console::log(&Array::of1(&JsValue::from_str("No winners found (gui - animate EndRound)")));
                                "error".to_string()
                            },
                            1 => format!("{} wins {} chips", winner_names[0].clone(), pot),
                            2 => format!("{} and {} win {} chips", winner_names[0], winner_names[1], pot),
                            _ => {
                                let all_but_last = &winner_names[..winner_names.len()-1];
                                let last = &winner_names[winner_names.len()-1];
                                format!("{} and {} win {} chips", all_but_last.join(", "), last, pot)
                            }
                        }
                        
                    }
                };
                self.animations.message = format!("End of round: {}", winners_str);
                self.ui.waiting_new_round = true;
            }
        }
    }

    fn debug(&self) -> Node<Msg> {
        node! {
            <div class="key-display">
                <p>{text(format!("Last key pressed: {}", self.last_key))}</p>
                <p><small>"Keyboard shortcuts: 
                + (PlayerCountUp), 
                - (PlayerCountDown),
                r/R (PlayerCountReset)"
                </small></p>
                <p><small>
                {text(format!("{} {} {} {} {}",
                    self.settings.player_count,
                    self.ui.main_menu,
                    self.ui.waiting_input,
                    self.animations.toggle,
                    self.animations.message,
                )
                )}
                </small></p>
                <p><small>
                {if let Some(ref game) = self.game {
                    let player = game.player_on_turn_immutable();
                    node! {{text(format!("game: {} {} {} {}",
                    game.pot,
                    player.id.to_str(),
                    self.req_bet,
                    game.round_number,
                ))}}
                } else {node! {<div></div>}}
                }
                </small></p>
                <p><small>
                {text(format!("debug: {} {}",
                    self.debug,
                    self.req_bet
                ))}
                </small></p>
            </div>
        }
    }
}

enum Msg {
    None,
    Fold,
    Call,
    Raise,
    TogglePause,
    Confirm,
    NewRound,
    EndAnimation,
    PlayerCountUp,
    PlayerCountDown,
    PlayerCountReset,
    KeyPressed(String),
    BetResponse(Response),
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
                        "Enter" => Msg::Confirm,
                        "f" | "F" => Msg::Fold,
                        "c" | "C" => Msg::Call,
                        "v" | "V" => Msg::Raise,
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
                    {
                        if DEBUG_VIEW {self.debug()}
                        else {node! {<div></div>}}
                    }
                </main>
            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::None => return Cmd::none(),
            Msg::PlayerCountUp => {
                if self.ui.main_menu {
                    if self.settings.player_count < 8 {self.settings.player_count += 1};
                    self.last_key = "+ (PlayerCountUp)".to_string();
                } else if self.ui.waiting_input {
                    if self.raise < 10000 {self.raise += 10};
                    self.last_key = "+ (RaiseUp)".to_string();
                }
            },
            Msg::PlayerCountDown => {
                if self.ui.main_menu {
                    if self.settings.player_count > 2 {self.settings.player_count -= 1};
                    self.last_key = "- (PlayerCountDown)".to_string();
                } else if self.ui.waiting_input {
                    if self.raise > 10 {self.raise -= 10};
                    self.last_key = "- (RaiseDown)".to_string();
                }
            },
            Msg::PlayerCountReset => {
                if self.ui.main_menu {
                    self.settings.player_count = 6;
                    self.last_key = "r (PlayerCountReset)".to_string();
                } else if self.ui.waiting_input {
                    self.raise = DEFAULT_RAISE;
                    self.last_key = "r (RaiseReset)".to_string();
                }
            },
            Msg::KeyPressed(key) => {
                self.last_key = format!("{} (no action)", key);
            },
            Msg::Confirm => {
                if self.ui.main_menu {
                    self.ui.pause = false;
                    self.ui.main_menu = false;
                    self.last_key = "Enter (StartGame)".to_string();
                    console::log(&Array::of1(&JsValue::from_str("Starting game")));
                    self.start_game();
                } else if self.ui.waiting_new_round {
                    self.ui.waiting_new_round = false;
                    self.last_key = "Enter (NewRound)".to_string();
                    self.update(Msg::NewRound);
                }
            },
            Msg::BetResponse(response_old) => {
                match response_old {
                    Response::WaitingPlayer1(req_bet) => {
                        self.req_bet = req_bet;
                        self.ui.waiting_input = true;
                    },
                    Response::OnePlayerRemaning => {
                        self.debug = "1 player remaining".to_string();
                        self.animate(Animated::EndRound);
                    },
                    Response::BetPlaced(_, _, _) => {
                        let response = match self.game {
                            None => {
                                console::log(&Array::of1(&JsValue::from_str("Failed to start game - no game exists(gui - Msg::BetResponse(BetPlaced))")));
                                Response::EndRound
                            },
                            Some(ref mut game) => {
                                let response = make_bets(game, self.req_bet, get_bet);
                                response
                            }
                        };
                        self.animate(Animated::Bet(response));
                    },
                    Response::StreetFinished(_, _) => {
                        self.req_bet = 0;
                        let msg = match self.game {
                            None => {
                                console::log(&Array::of1(&JsValue::from_str("Failed to start game - no game exists(gui - Msg::BetResponse(RoundFinished))")));
                                Msg::None
                            },
                            Some(ref mut game) => {
                                next_turn(game);
                                game.round_number = 1;
                                match game.street {
                                    Streets::Showdown => {
                                        Msg::NewRound
                                    },
                                    _ => {
                                        Msg::BetResponse(make_bets(game, 0, get_bet))
                                    }
                                }
                                }
                            };
                        match msg {
                            Msg::BetResponse(response) => {
                                self.animate(Animated::Bet(response));
                            },
                            Msg::NewRound => {
                                self.animate(Animated::EndRound);
                            },
                            _ => {
                                console::log(&Array::of1(&JsValue::from_str("Unexpected message type in Msg::BetResponse(RoundFinished)")));
                            },
                        }
                    },
                    Response::EndRound => {
                        self.animate(Animated::EndRound);
                    }
                }
            },
            Msg::Fold => {
                self.debug = "player fold".to_string();
                self.last_key = "f (Fold)".to_string();
                if self.ui.waiting_input {
                    self.ui.waiting_input = false;
                    let response = match self.game {
                        None => {
                            console::log(&Array::of1(&JsValue::from_str("Faild to start game - no game exists(gui - Msg::Fold)")));
                            Response::EndRound
                        },
                        Some(ref mut game) => {
                            fold_bet(game, self.req_bet)
                        }
                    };
                    self.animate(Animated::Bet(response));
                }
            },
            Msg::Raise => {
                self.debug = "player raise".to_string();
                self.last_key = "v (Raise)".to_string();
                if self.ui.waiting_input {
                    self.ui.waiting_input = false;
                    let response = match self.game {
                        None => {
                            console::log(&Array::of1(&JsValue::from_str("Faild to start game - no game exists(gui - Msg::Raise)")));
                            Response::EndRound
                        },
                        Some(ref mut game) => {
                            game.round_number += 1;
                            active_bet(game, self.req_bet, self.raise)
                        }
                    };
                    self.animate(Animated::Bet(response));
                };
            },
            Msg::Call => {
                self.debug = "player call".to_string();
                self.last_key = "c (Call)".to_string();
                if self.ui.waiting_input {
                    self.ui.waiting_input = false;
                    let response = match self.game {
                        None => {
                            console::log(&Array::of1(&JsValue::from_str("Faild to start game - no game exists(gui - Msg::Call)")));
                            Response::EndRound
                        },
                        Some(ref mut game) => {
                            game.round_number += 1;
                            active_bet(game, self.req_bet, 0)
                        }
                    };
                    self.animate(Animated::Bet(response));  
                };
            },
            Msg::NewRound => {
                self.req_bet = 0;
                self.ui.waiting_new_round = false;
                match self.game {
                    None => {
                        console::log(&Array::of1(&JsValue::from_str("Game not found (gui - EndRound)")));
                    },
                    Some(ref mut game) => {
                        game.position_on_turn = PlayerPosition::SmallBlind;
                        for player in &mut game.players {
                            player.opened_cards = false;
                            player.playing = true;
                            player.current_bet = 0;
                            player.hand_cards = (
                                Card {
                                    color: CardColor::Empty,
                                    number: CardNumber::Empty,
                                },
                                Card {
                                    color: CardColor::Empty,
                                    number: CardNumber::Empty,
                                });
                            for _ in 2..game.player_count {
                                player.position = player.position.next_player_on_turn_for_count(game.player_count);
                            };
                        };
                        game.pot = 0;
                        game.deck = Card::make_ordered_deck();
                        game.street = Streets::PreFlop;
                        game.table_cards = Vec::new();
                        game.position_on_turn = player::PlayerPosition::UnderTheGun;
                        game.round_number = 1;
                        game.quit = false;
                    }
                };
                self.start_round();
            },
            Msg::EndAnimation => {
                match self.game {
                    None => {
                        console::log(&Array::of1(&JsValue::from_str("Game not found (gui - EndRound)")));
                    },
                    Some(ref mut game) => {
                    game.position_on_turn = game.position_on_turn.next_player_on_turn_for_count(game.player_count);
                    }
                };
                let msg = match self.animations.animation_type.clone() {
                    Animated::Bet(response) => {
                        Msg::BetResponse(response)
                    },
                    Animated::EndRound => {
                        Msg::NewRound
                    }
                };
                self.update(msg);
            },
            _ => return Cmd::none(),
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console::log(&Array::of1(&JsValue::from_str("Program started")));
    Program::mount_to_body(App::new());
}
