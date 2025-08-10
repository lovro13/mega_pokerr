
use core::panic;
use sauron::prelude::*;
use web_sys::KeyboardEvent;

use crate::logic::choose_winner::choose_winner;
use crate::logic::{
    card::{Card, CardColor, CardNumber}, 
    constants::DEFAULT_PLAYER_COUNT, 
    game::{self, init_game, Game, Streets}, 
    player::{self, Player, PlayerPosition, Id}, 
    round::{begin_round, next_turn}
};
use crate::sauron_app::constants::{BET_ANIMATION_TIME, MAIN_PLAYER, DEFAULT_RAISE};
use crate::sauron_app::{
    make_bets::{Response, make_bets, active_bet, fold_bet},
    ai::{get_bet,}
    };



struct Settings {
    player_count: usize,
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
    css_class: String,  // CSS class for animation state
    message: String, // Message to display during animation
    animation_type: Animated, // Type of animation
    toggle: bool,  // Toggle between animation classes to force restart
}

struct App {
    last_key: String,
    settings: Settings,
    game: Option<Game>,
    ui: Ui,
    end_round: bool,
    animations: Animation,
    req_bet: u32,
    raise: u32,
    

}

impl App {
    fn new() -> Self {
        App { 
            last_key: "None".to_string(),
            settings: Settings {player_count: DEFAULT_PLAYER_COUNT},
            game: None,
            ui: Ui {main_menu: true, pause: true, waiting_input: false, waiting_new_round: false},
            end_round: false,
            animations: Animation { css_class: String::new(), animation_type: Animated::EndRound, message: String::new(), toggle: false },
            req_bet: 0,
            raise: DEFAULT_RAISE,
            
        }
    }

    

    // Player chips
    // fn player_chips(&Self, chips: &u32) -> Node<Msg> {

    // }

    // Player node
    fn render_player(&self, player_id: usize) -> Node<Msg> {
        let player = match self.game {
            None => panic!("Game failed to render (gui - render_player)"),
            Some(ref game) => {
                match game.players.get(player_id - 1) {
                    None => panic!("Player not found (gui - render_player)"),
                    Some(player) => player
                }
            }
        };
        let (card1_a, card2_b) = &player.hand_cards;
        let (card1, card2) = if self.ui.waiting_new_round || (player_id == 1)  {
            (card1_a.to_png(), card2_b.to_png())
        } else {("assets/cards/card_back.png".to_string(), "assets/cards/card_back.png".to_string())
        }; // nastavlja katere roke so prikazane
        let player_name = format!("Player {}", player_id);

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
        
        // Pozicija 1. in 2. karte besedila, ter kovanca
        let style_card1 = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x - 5.), y
        );
        let style_card2 = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x + 2.), y
        );
        let style_text = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x - 4.), (y + 15.)
        );
        let style_token = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (x - 4.), (y - 15.)
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
                        <div style={style_card1}>
                            <p>
                                "Folded"
                            </p>
                        </div>
                    }  
                }}

                <div style={style_text}>
                    <p>
                    {text(player_name)}
                    </p>
                    <p>
                    {text(format!("Chips :{}", player.chips))}   
                    </p>
                    <p>
                    {text(format!("Current bet: {}", player.current_bet))}   
                    </p>
                    <p>
                    {text(format!("Possition: {}", player.position.eval_to_int()))}   
                    </p>
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
                {if player.id == MAIN_PLAYER {
                        node! {
                            <div>
                                <div class="player-raise-selection">
                                    <input type="button"
                                        value="+"
                                        on_click=|_| Msg::PlayerCountUp
                                    />
                                    <button class="count" 
                                        on_click=|_|{Msg::PlayerCountReset} >
                                        {text(self.raise)}
                                    </button>
                                    <input type="button"
                                        value="-"
                                        on_click=|_| Msg::PlayerCountDown
                                    />
                                </div>
                                <div>
                                    <button class="raise" 
                                        on_click=|_|{Msg::Confirm} >
                                        {text(format!("Raise {}", self.raise))}
                                    </button>
                                </div>
                                <div>
                                    <button class="Call" 
                                        on_click=|_|{Msg::Call} >
                                        "Call"
                                    </button>
                                </div>
                                <div>
                                    <button class="Fold" 
                                        on_click=|_|{Msg::Fold} >
                                        "Fold"
                                    </button>
                                </div>
                            </div>
                        }
                    
                } else {node! {<div></div>}}
                }
                // <div class="player-chip">
                //     {    
                //     }
                // </div>
            </div>
        }
    }

    // Karte na mizi
    fn render_table_card(&self, card: &Card, i: usize) -> Node<Msg> {
        let style = format!(
            "position: absolute; left: {}%; top: {}%; transform: translate(-50%, -50%);",
            (50 + i*20), 50
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
            None => panic!("Game failed to render (gui - render_player)"),
            Some(ref game) => &game.table_cards};
        let mut i = 0;

        node! {
            <div class="game-table-container" 
            style="position: relative; width: 50vw; height: 80vh; margin: 0 auto 50px auto;"
            >
                { // igralci
                    for i in 1..=self.settings.player_count {
                    self.render_player(i)
                }}
            
                <div style="position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%);">
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
                {if !self.animations.css_class.is_empty() {node!{<div 
                    // key={format!("animation-{}", self.animations.counter)}
                    class={format!("animation-overlay {}", self.animations.css_class)}
                    style="position: absolute; 
                    left: 50%; 
                    top: 50%; 
                    transform: translate(-50%, -50%);
                    background-color: green;
                    padding: 10px;
                    border-radius: 8px;
                    color: white;
                    font-weight: bold;"
                    
                    on_animationend=|_| Msg::EndAnimation
                    >
                        <p>{text(self.animations.message.clone())}</p>
                    </div>}} 
                else {node!{<div></div>}}
                }
                {if self.ui.waiting_new_round {node! { <div 
                    class="End-round-msg"
                    style="position: absolute; 
                    left: 50%; 
                    top: 50%; 
                    transform: translate(-50%, -50%);
                    background-color: green;
                    padding: 10px;
                    border-radius: 8px;
                    color: white;
                    font-weight: bold;"
                    >
                        <p>{text(self.animations.message.clone())}</p>
                    </div>}}
                else {node!{<div></div>}}
                }
            </div>
        }
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
                        on_click=|_|{Msg::Confirm} >
                        "Start Game"
                    </button>
                </div>
            </div>
        }
    }

    // Začni igro
    fn start_game(&mut self) {
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
        self.start_round();
    }

    // Začni krog
    fn start_round(&mut self) {
        let response = match self.game {
            None => {panic!("Faild to start game - no game exists(gui - start_game)")},
            Some(ref mut game) => {
                begin_round(game, self.settings.player_count);
                make_bets(game, 0, get_bet)
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
                        self.update(Msg::BetResponse(Response::WaitingPlayer1(req_bet)));
                    },
                    Response::OnePlayerRemaning => {
                        self.update(Msg::BetResponse(Response::OnePlayerRemaning));
                    },
                    Response::BetPlaced(id, req_bet, decision ) => {
                        
                        let animating_player = id.to_str();
                        let decision = match decision {
                            None => {String::from("folded")},
                            Some(0) => {String::from("called")},
                            Some(x) => {format!("raised by {}", x)},
                        };
                        self.animations.message = format!("{} {}", animating_player, decision);
                        self.animations.toggle = !self.animations.toggle;  // Toggle to force animation restart
                        self.animations.css_class = if self.animations.toggle {
                            "bet-animation-a".to_string()
                        } else {
                            "bet-animation-b".to_string()
                        };
                    },
                    Response::StreetFinished(id,decision ) => {
                        
                        let animating_player = id.to_str();
                        let decision = match decision {
                            None => {String::from("folded")},
                            Some(0) => {String::from("called")},
                            Some(x) => {format!("raised by {}", x)},
                        };
                        self.animations.message = format!("{} {}", animating_player, decision);
                        self.animations.toggle = !self.animations.toggle;  // Toggle to force animation restart
                        self.animations.css_class = if self.animations.toggle {
                            "bet-animation-a".to_string()
                        } else {
                            "bet-animation-b".to_string()
                        };
                    },
                    Response::EndRound => {
                        self.animate(Animated::EndRound);
                    }
                }
            },
            Animated::EndRound => {
                self.ui.waiting_new_round = true;
                self.animations.animation_type = Animated::EndRound;

                self.req_bet = 0;
                let winners_str = match self.game {
                    None => {panic!("Game not found (gui - EndRound)")},
                    Some(ref mut game) => {
                        let pot = game.pot;
                        let winners = choose_winner(game);
                        let winner_names = Vec::from_iter(winners.iter().map(|player| player.id.to_str()));
                        let n = winners.len() as u32;
                        for player in winners {
                            player.chips += pot/n;
                        };
                        //game.pot = 0;
                        match winner_names.len() {
                            0 => panic!("No winners found (gui - animate EndRound)"),
                            1 => format!("{} wins", winner_names[0].clone()),
                            2 => format!("{} and {} win", winner_names[0], winner_names[1]),
                            _ => {
                                let all_but_last = &winner_names[..winner_names.len()-1];
                                let last = &winner_names[winner_names.len()-1];
                                format!("{} and {} win", all_but_last.join(", "), last)
                            }
                        }
                        
                    }
                };

                

                self.animations.message = format!("End of round: {}", winners_str);
                // self.animations.toggle = !self.animations.toggle;  // Toggle to force animation restart
                // self.animations.css_class = if self.animations.toggle {
                //     "bet-animation-a".to_string()
                // } else {
                //     "bet-animation-b".to_string()
                // };
                self.ui.waiting_new_round = true;
            }
        }
       
        

        // animacija konca kroga
        // fn animate_end_round(&mut self, winners: Vec<Player>)

        

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
                        "j" | "J" => Msg::Fold,
                        "k" | "K" => Msg::Call,
                        "l" | "L" => Msg::Raise,
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
                    <p><small>
                    {text(format!("{} {} {} {} {}",
                        self.settings.player_count,
                        self.ui.main_menu,
                        self.ui.waiting_input,
                        self.animations.toggle,
                        self.animations.message,
                    )
                    )}


            // settings: Settings {player_count: DEFAULT_PLAYER_COUNT},
            // game: None,
            // ui: Ui {main_menu: true, pause: true, waiting_input: false},
            // end_round: false,
            // animations: Animation { animating: false, player: MAIN_PLAYER, decision: None },
            // req_bet: 0,
            // raise: 10,
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
                </div>
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
                    self.raise += 10;
                    self.last_key = "+ (RaiseUp)".to_string();
                }
            },
            Msg::PlayerCountDown => {
                if self.ui.main_menu {
                    if self.settings.player_count > 2 {self.settings.player_count -= 1};
                    self.last_key = "- (PlayerCountDown)".to_string();
                } else if self.ui.waiting_input {
                    self.raise -= 10;
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
                        // end round
                        self.animate(Animated::EndRound);
                    },
                    Response::BetPlaced(id, req_bet, decision ) => {
                        // Debug: show what we received
                        // new bet
                        let response = match self.game {
                            None => {panic!("Failed to start game - no game exists(gui - Msg::BetResponse(BetPlaced))")},
                            Some(ref mut game) => {
                                let response = make_bets(game, req_bet, get_bet);
                                // Debug: log what make_bets returned
                                response
                            }
                        };
                        self.animate(Animated::Bet(response));
                    },
                    Response::StreetFinished(id, decision ) => {
                        // new bet
                        self.req_bet = 0;
                        let msg = match self.game {
                            None => {panic!("Faild to start game - no game exists(gui - Msg::BetResponse(RoundFinished))")},
                            Some(ref mut game) => {
                                // end street
                                next_turn(game);
                                game.round_number = 0;
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
                            _ => {panic!("Unexpected message type in Msg::BetResponse(RoundFinished)")}
                        }
                        
                    },
                    Response::EndRound => {
                        self.animate(Animated::EndRound);
                    }
                }
            },
            Msg::Fold => {
                if self.ui.waiting_input {
                    self.ui.waiting_input = false;
                    let current_req_bet = self.req_bet.clone();
                    let response = match self.game {
                        None => {panic!("Faild to start game - no game exists(gui - Msg::Fold)")},
                        Some(ref mut game) => {
                            fold_bet(game)
                        }
                    };
                    self.animate(Animated::Bet(response));
                }
            },
            Msg::Raise => {
                if self.ui.waiting_input {
                    self.ui.waiting_input = false;
                    let response = match self.game {
                        None => {panic!("Faild to start game - no game exists(gui - Msg::Raise)")},
                        Some(ref mut game) => {  
                            active_bet(game, self.req_bet, self.raise)
                            // match active_bet(game, self.req_bet, self.raise) {
                            //     Response::BetPlaced(id, req_bet, decision ) => {
                            //         assert!(id == MAIN_PLAYER);
                            //         self.req_bet = req_bet;
                            //         Msg::BetResponse(make_bets(game, req_bet, get_bet))
                            //     },
                            //     Response::WaitingPlayer1(_) => {panic!("impossible Response enum(gui - Msg::Raise)")},
                            //     Response::OnePlayerRemaning => {
                            //         Msg::NewRound
                            //     },
                            //     Response::StreetFinished(id, decision ) => {
                            //         assert!(id == MAIN_PLAYER);
                            //         Msg::BetResponse(make_bets(game, 0, get_bet))
                            //     },
                            //     Response::EndRound => {
                            //         Msg::NewRound
                            //     },
                            // }
                        }
                    };
                    self.animate(Animated::Bet(response));
                    // match msg {
                    //    Msg::BetResponse(response) => {
                    //        self.animate(Animated::Bet(response));
                    //    },
                    //    Msg::NewRound => {
                    //        self.animate(Animated::EndRound);
                    //    },
                    //    _ => {panic!("Unexpected message type in Msg::Raise")}
                    // };
                };
            },
            Msg::Call => {
                if self.ui.waiting_input {
                    self.ui.waiting_input = false;
                    let response = match self.game {
                        None => {panic!("Faild to start game - no game exists(gui - Msg::Raise)")},
                        Some(ref mut game) => {
                            active_bet(game, self.req_bet, 0)
                            // match active_bet(game, self.req_bet, self.raise) {
                            //     Response::BetPlaced(id, req_bet, decision ) => {
                            //         assert!(id == MAIN_PLAYER);
                            //         self.req_bet = req_bet;
                            //         Msg::BetResponse(make_bets(game, req_bet, get_bet))
                            //     },
                            //     Response::WaitingPlayer1(_) => {panic!("impossible Response enum(gui - Msg::Raise)")},
                            //     Response::OnePlayerRemaning => {
                            //         Msg::NewRound
                            //     },
                            //     Response::StreetFinished(id, decision ) => {
                            //         assert!(id == MAIN_PLAYER);
                            //         Msg::BetResponse(make_bets(game, 0, get_bet))
                            //     },
                            //     Response::EndRound => {
                            //         Msg::NewRound
                            //     },
                            // }
                        }
                    };
                    self.animate(Animated::Bet(response));  
                    // match msg {
                    //    Msg::BetResponse(response) => {
                    //        self.animate(Animated::Bet(response));
                    //    },
                    //    Msg::NewRound => {
                    //        self.animate(Animated::EndRound);
                    //    },
                    //    _ => {panic!("Unexpected message type in Msg::Call")}
                    // };
                };
            },
            Msg::NewRound => {
                self.req_bet = 0;
                self.ui.waiting_new_round = false;
                match self.game {
                    None => {panic!("Game not found (gui - EndRound)")},
                    Some(ref mut game) => {
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
                self.animations.css_class.clear();
                let msg = match self.animations.animation_type.clone() {
                    Animated::Bet(response) => {
                        Msg::BetResponse(response)
                    },
                    Animated::EndRound => {
                        // Msg::NewRound
                        Msg::None
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
    Program::mount_to_body(App::new());
}
