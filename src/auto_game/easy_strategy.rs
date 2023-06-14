use super::{TicTacToeStrategy,TicTacToeGameReader};
use super::super::manual_game::{GameStatus};
use rand::Rng;

pub struct EasyStrategy;
impl<T: TicTacToeGameReader> TicTacToeStrategy<T> for EasyStrategy {
    fn suggest_next_move(&self, game_reader: &T) -> Result<(u8, u8), String> {
        if let GameStatus::GameFinished = game_reader.game_status() {
            return Err(String::from("Game already finished"));
        }

        let map = game_reader.get_map();

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
