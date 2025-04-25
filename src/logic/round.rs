use crate::logic::card;
use crate::logic::player;

enum Round { // zelooo popraviti
    UsersTurn,
    BotsTurn(player::Player),
}

pub enum Streets {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

pub fn begin_round(player_list: &mut Vec<player::Player>) {
    let curr_street = Streets::PreFlop;
    let deck = card::Card::make_ordered_deck();
    let mut deck = card::Card::scramble_deck(deck);

    for player in player_list {
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
        player.cards = (card1, card2)
    }

    // TODO za vse streets napisati kaj se zgodi med igro, torej je treba dodati gumbe da igralec lahko igra
}
