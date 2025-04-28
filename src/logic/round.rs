use crate::logic::card;
use crate::logic::player;
use crate::logic::choose_winner::choose_winner;

#[derive(Debug, PartialEq, Clone)]
pub enum Streets {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown
}




pub struct Game {
    pub street: Streets,
    pub pot: u32,
    pub players: Vec<player::Player>,
    pub deck: Vec<card::Card>,
    pub table_cards: Vec<card::Card>,
    pub player_on_turn: player::PlayerPosition,
    pub round_number: u32,
}

impl Game {
    pub fn next_street(game: &mut Game) {
        game.street = match game.street {
            Streets::PreFlop => Streets::Flop,
            Streets::Flop => Streets::Turn,
            Streets::Turn => Streets::River,
            Streets::River => Streets::Showdown,
            Streets::Showdown => Streets::PreFlop
        }
    }
}

pub fn init_game(player_list: Vec<player::Player>) -> Game {
    let deck = card::Card::make_ordered_deck();
    let deck = card::Card::scramble_deck(deck);
    let mut_player_list = player_list;
    Game {
        street: Streets::PreFlop,
        pot: 30,
        players: mut_player_list,
        deck, 
        table_cards: Vec::new(),
        player_on_turn: player::PlayerPosition::UnderTheGun,
        round_number: 0
    }
}

pub fn begin_round(game : &mut Game) {
    let deck = card::Card::make_ordered_deck();
    let mut deck = card::Card::scramble_deck(deck);
    for player in game.players.iter_mut() {
    player.position = player::PlayerPosition::next_player_position(&player.position);
    let card1 = match deck.pop() {
        None => card::Card {
            color: card::CardColor::Empty,
            number: card::CardNumber::Empty,
        },
        Some(card) => card,
    };
    let card2 = match deck.pop() {
        None => card::Card {
            color: card::CardColor::Empty,
            number: card::CardNumber::Empty,
        },
        Some(card) => card,
    };
    if player.position == player::PlayerPosition::SmallBlind {
        player.money -= 10;
    }  else if player.position == player::PlayerPosition::BigBlind {
        player.money -= 20;
    }
    player.cards = (card1, card2)
    }
}

pub fn make_bets<'a>(round: &mut Game, get_bet: fn(&mut player::Player) -> Option<u32>) -> Vec<(player::Names, u32)> {
    // TODO: narediti loop, da se bo stavilo toliko časa dokler ne zmanka denarja ali pa dajo vsi isto stavoi

    let mut bets = Vec::new();
    for player in round.players.iter_mut() {
        if player.playing {
            match get_bet(player) {
                None => {
                    player.playing = false;
                },
                Some(bet) if bet <= player.money => {
                    player.money -= bet;
                    round.pot += bet;
                    bets.push((player.name.clone(), bet));
                }
                Some(bet) if bet > player.money => {
                    player.money = 0;
                    round.pot += bet;
                    bets.push((player.name.clone(), bet));
                }
                Some(_) => panic!("nekaj narobe pri stavah")
            }
        }
    }
    bets
}

pub fn next_turn(game: &mut Game)  {
    // gre na naslednji street in "položi karte na mizo kolikor je treba"
    let _ = match game.street.clone() {
        Streets::PreFlop => {}
        Streets::Flop => 
        {
            for _ in 0..2 {
                let card = match game.deck.pop() {
                    None => panic!("Deck is empty"),
                    Some(card) => card,
                };
                game.table_cards.push(card);
            }
        }
        Streets::River | Streets::Turn => 
        {
            let card = match game.deck.pop() {
                None => panic!("Deck is empty"),
                Some(card) => card,
            };
            game.table_cards.push(card);
        }
        Streets::Showdown => {choose_winner(game);}
    };
    Game::next_street(game);
}
