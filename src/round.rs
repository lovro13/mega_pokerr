mod card;

enum Round {
    UsersTurn,
    BotsTurn(card::Player)
}