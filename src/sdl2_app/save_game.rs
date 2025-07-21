use rusqlite::{params, Connection, Result, Transaction};

use crate::logic::{
    game::Game
};

pub fn create_tables(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    let _ = tx.execute_batch(
        "CREATE TABLE IF NOT EXISTS saves (
            game_id INTEGER PRIMARY KEY AUTOINCREMENT,
            game_state TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now', 'localtime')) 
            );",
    );
    log::debug!("Created table if it didnt exist before");
    tx.commit()?;
    Ok(())
}

pub fn save_game(game: &Game, conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    log::debug!("Saving game with players : {:#?}", game.players);
    let game_state = serde_json::to_string(game).unwrap();
    log::debug!("converted game to json: {game_state:#?}");
    let update = tx.execute(
        "INSERT INTO saves (game_state) VALUES(?1)",
        params![game_state],
    )?;
    log::debug!("database changed: {update}");
    tx.commit()?;
    Ok(())
}

pub fn load_game(game_id: i64, tx: &Transaction) -> Result<Option<Game>> {
    let mut stmt = tx.prepare("SELECT game_state FROM saves WHERE game_id = ?1")?;

    let game_result = stmt.query_row([game_id], |row| {
        let game_state: String = row.get(0)?;
        Ok(game_state)
    });

    match game_result {
        Ok(game_state) => {
            let game: Game = serde_json::from_str(&game_state).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            let mut new_game = Game {
                street: crate::logic::game::Streets::PreFlop,
                pot: 0,
                players: game.players,
                deck: game.deck,
                table_cards: vec![],
                position_on_turn: crate::logic::player::PlayerPosition::NotPlaying,
                round_number: game.round_number,
                quit: false,
                player_count: game.player_count,
            };
            for player in new_game.players.iter_mut() {
                player.chips += player.current_bet;
                player.current_bet = 0
            }
            log::debug!("loaded game with players: {:?}", new_game);
            Ok(Some(new_game))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn list_saved_games(conn: &Connection) -> Result<Vec<(i64, String)>> {
    let mut stmt =
        conn.prepare("SELECT game_id, created_at FROM saves ORDER BY created_at DESC")?;
    let rows = stmt.query_map([], |row| {
        let game_id: i64 = row.get(0)?;
        let created_at: String = row.get(1)?;
        Ok((game_id, created_at))
    })?;
    let mut saves = Vec::new();
    for row in rows {
        saves.push(row?);
    }
    Ok(saves)
}
