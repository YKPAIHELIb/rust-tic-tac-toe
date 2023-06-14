use super::super::manual_game::GameStatus;
use super::{TicTacToeGameReader, TicTacToeStrategy};
use rand::Rng;

pub struct MediumStrategy;
impl<T: TicTacToeGameReader> TicTacToeStrategy<T> for MediumStrategy {
    fn suggest_next_move(&self, game_reader: &T) -> Result<(u8, u8), String> {
        if let GameStatus::GameFinished = game_reader.game_status() {
            return Err(String::from("Game already finished"));
        }

        let map = game_reader.get_map();
        let is_move_for_x = game_reader.game_status() == GameStatus::MoveForX;

        if let Some(line_to_win) = find_almost_finished_line(map, is_move_for_x) {
            return Ok(get_index_to_finish_line(&line_to_win, map));
        }

        if let Some(line_to_protect_from) = find_almost_finished_line(map, !is_move_for_x) {
            return Ok(get_index_to_finish_line(&line_to_protect_from, map));
        }

        let free_cells: Vec<_> = (0..map.len())
            .map(|i| (0..map.len()).map(move |j| (i, j)))
            .flatten()
            .filter(|(i, j)| map[*i][*j].is_none())
            .collect();

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..free_cells.len());

        let (row, col) = free_cells[random_index];
        return Ok((row as u8, col as u8));

        fn find_almost_finished_line(
            map: &Vec<Vec<Option<bool>>>,
            search_for_x: bool,
        ) -> Option<Vec<(usize, usize)>> {
            for (i, line) in map.iter().enumerate() {
                if check_almost_finished(line.iter(), search_for_x) {
                    return Some((0..map.len()).map(|j| (i, j)).collect());
                }
            }

            for j in 0..map.len() {
                let column = (0..map.len()).map(|i| &map[i][j]);
                if check_almost_finished(column.into_iter(), search_for_x) {
                    return Some((0..map.len()).map(|i| (i, j)).collect());
                }
            }

            let diagonal: Vec<(usize, usize)> = (0..map.len()).map(|i| (i, i)).collect();
            if check_almost_finished(
                diagonal.iter().map(|(i, j)| &map[*i][*j]).into_iter(),
                search_for_x,
            ) {
                return Some(diagonal);
            }

            let diagonal: Vec<(usize, usize)> =
                (0..map.len()).map(|i| (i, map.len() - i - 1)).collect();
            if check_almost_finished(
                diagonal.iter().map(|(i, j)| &map[*i][*j]).into_iter(),
                search_for_x,
            ) {
                return Some(diagonal);
            }

            return None;

            fn check_almost_finished<'a>(
                line: impl Iterator<Item = &'a Option<bool>>,
                search_for_x: bool,
            ) -> bool {
                let mut set_count = 0;
                let mut empty_count = 0;
                let mut overall_count = 0;

                for cell in line {
                    overall_count += 1;
                    if cell.is_none() {
                        empty_count += 1;
                    } else if cell.unwrap() == search_for_x {
                        set_count += 1;
                    }
                }

                empty_count == 1 && set_count == overall_count - 1
            }
        }

        fn get_index_to_finish_line(line: &Vec<(usize, usize)>, map: &Vec<Vec<Option<bool>>>) -> (u8, u8) {
            line.iter()
                .find(|(i, j)| map[*i][*j].is_none())
                .map(|(i, j)| (*i as u8, *j as u8))
                .expect("Should've find None because line was validated to have it")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        pub GameReader {}

        impl TicTacToeGameReader for GameReader {
            fn get_map(&self) -> &Vec<Vec<Option<bool>>>;
            fn game_status(&self) -> GameStatus;
        }
    }

    #[test]
    fn protect_from_lose_1() {
        let map = vec![
            vec![Some(true), Some(true), Some(false)],
            vec![Some(true), Some(false), None],
            vec![None, None, None],
        ];

        let mut mock_game_reader = MockGameReader::new();
        mock_game_reader.expect_get_map().return_const(map);
        mock_game_reader
            .expect_game_status()
            .returning(|| GameStatus::MoveForO);

        let strategy = MediumStrategy;
        let result = strategy.suggest_next_move(&mock_game_reader);

        assert!(result.is_ok());
        let (row, col) = result.unwrap();
        assert_eq!((row, col), (2, 0));
    }

    #[test]
    fn protect_from_lose_2() {
        let map = vec![
            vec![Some(false), Some(true), None],
            vec![None, Some(true), None],
            vec![None, None, None],
        ];

        let mut mock_game_reader = MockGameReader::new();
        mock_game_reader.expect_get_map().return_const(map);
        mock_game_reader
            .expect_game_status()
            .returning(|| GameStatus::MoveForO);

        let strategy = MediumStrategy;
        let result = strategy.suggest_next_move(&mock_game_reader);

        assert!(result.is_ok());
        let (row, col) = result.unwrap();
        assert_eq!((row, col), (2, 1));
    }
}
