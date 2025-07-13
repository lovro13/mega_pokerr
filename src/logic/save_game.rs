use rusqlite::{Result, Transaction};

pub fn create_tables(tx: &Transaction) -> Result<()> {
    log::debug!("Creating tables");
    tx.execute_batch(
    "CREATE TABLE IF NOT EXISTS game_state (
            round INTEGER PRIMARY KEY,
            street TEXT NOT NULL,
            pot INTEGER NOT NULL,
            position_on_turn TEXT NOT NULL,
            quit BOOLEAN NOT NULL,
            player_count INTEGER NOT NULL
        );"
    )
}

// fn save_game_state(tx: &Transaction, game: &Game) -> Result<()> {
//     tx.execute("INSERT OR REPLACE INTO game_state
//                 (round, street, pot, position_on_turn, quit, player_count)
//                 VALUES(?1, ?2, ?3, ?4, ?5, ?6)", 
//                 params![
//                     game.round_number as i64,
//                     serde_json::to_string(&game.street).unwrap(),
//                     game.pot as i64,
//                     serde_json::to_string(&game.position_on_turn).unwrap(),
//                     game.quit,
//                     game.player_count as i64
//                     ])?; 
//     Ok(())
// }

// fn save_player(tx: &Transaction, player: Player, round: i32) -> Result<()> {
//     tx.execute("INSERT OR REPLACE INTO game_state
//                 (round, id, hand_cards, position_on_turn, quit, player_count)
//                 VALUES(?1, ?2, ?3, ?4, ?5, ?6)", params)?;
//     Ok(())
// }

// pub fn save_game(game: Game) -> Result<()> {
//     let conn = Connection::open_in_memory()?;

//     conn.execute("CREATE TABLE game (
//             id   INTEGER PRIMARY KEY,
//             name TEXT NOT NULL,
//             data BLOB
//         )", ())?;

        

//     let mut stmt = conn.prepare("SELECT");
//     Ok(())
// }