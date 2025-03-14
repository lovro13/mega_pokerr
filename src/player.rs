mod card;

pub enum Names {
    Player1, Player2, Player3, Player4, 
    Player5, Player6, Player7, Player8
}

pub struct Player {
    name: Names,
    card: card::Card,
    
}
