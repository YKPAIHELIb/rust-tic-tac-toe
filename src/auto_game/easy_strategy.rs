use super::TicTacToeStrategy;
use super::super::manual_game::{GameStatus, TicTacToeManual};
use rand::Rng;

pub struct EasyStrategy;
impl TicTacToeStrategy for EasyStrategy {
    fn suggest_next_move(&self, game: &TicTacToeManual) -> Result<(u8, u8), String> {
        if let GameStatus::GameFinished = game.game_status() {
            return Err(String::from("Game already finished"));
        }

        let map = game.get_map();

        let free_cells: Vec<_> = (0..map.len())
            .map(|i| (0..map.len())
                .map(move |j| (i, j)))
			.flatten()
			.filter(|(i,j)| map[*i][*j].is_none())
			.collect();
    
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..free_cells.len());

        let (row, col) = free_cells[random_index];
        Ok((row as u8, col as u8))
    }
}
