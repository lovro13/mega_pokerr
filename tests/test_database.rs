
#[cfg(test)]
mod tests {
    use mega_pokerr::logic::{constants::DATABASE_PATH, game::init_game, player, save_game::{create_tables, load_game, save_game}};
    use rusqlite::Connection;

    #[test]
    fn test_saving_game() {
        let mut connection = Connection::open(DATABASE_PATH).unwrap();
        let player_list = player::Player::init_players();
        let game = init_game(player_list);
        let _ = create_tables(&mut connection);
        let _ = save_game(&game.borrow(), &mut connection);
    }

    #[test]
    fn test_loading_game() {
        let mut connection = Connection::open(DATABASE_PATH).unwrap();
        let tx = connection.transaction().unwrap();
        let game = load_game(1, &tx).unwrap();
        match game {
            Some(x) => {println!("{}", x.pot);},
            None => {println!("no game with id 1")},
        }
        // println!("{:?}", game);
    }
}
