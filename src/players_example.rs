static mut PLAYER1: Player = Player {
    name: card::Names::Player1,
    card: EMPTY_CARD,
    card_position: PLAYER1_CARDS,
    card_state: card::CardState::Opened
};
static mut PLAYER2: Player = Player {
    name: card::Names::Player2,
    card: EMPTY_CARD,
    card_position: PLAYER2_CARDS,
    card_state: card::CardState::Opened
};
static mut PLAYER3: Player = Player {
    name: card::Names::Player3,
    card: EMPTY_CARD,
    card_position: PLAYER3_CARDS,
    card_state: card::CardState::Opened
};
static mut PLAYER4: Player = Player {
    name: card::Names::Player4,
    card: EMPTY_CARD,
    card_position: PLAYER4_CARDS,
    card_state: card::CardState::Opened
};
static mut PLAYER5: Player = Player {
    name: card::Names::Player5,
    card: EMPTY_CARD,
    card_position: PLAYER5_CARDS,
    card_state: card::CardState::Opened
};
static mut PLAYER6: Player = Player {
    name: card::Names::Player6,
    card: EMPTY_CARD,
    card_position: PLAYER6_CARDS,
    card_state: card::CardState::Opened
};
static mut PLAYER7: Player = Player {
    name: card::Names::Player7,
    card: EMPTY_CARD,
    card_position: PLAYER8_CARDS,
    card_state: card::CardState::Opened
};
static mut PLAYER8: Player = Player {
    name: card::Names::Player8,
    card: EMPTY_CARD,
    card_position: PLAYER8_CARDS,
    card_state: card::CardState::Opened
};// to bo moral se drgac nrdit, pomoje da najbol v  main, pa da render uzame player_list
// pa pol dela s temu, ka te se bojo povsod rabl