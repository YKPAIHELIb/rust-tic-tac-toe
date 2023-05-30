pub struct TicTacToeManual {
    map: Vec<Vec<Option<bool>>>,
    game_status: GameStatus,
}

impl TicTacToeManual {
    pub fn build(map_size: u8) -> Result<Self, &'static str> {
        if map_size > 9 || map_size < 3 {
            return Err("Map size should be one digit >= 3");
        }

        Ok(Self {
            map: vec![vec![None; map_size as usize]; map_size as usize],
            game_status: GameStatus::MoveForX,
        })
    }

    pub fn get_map(&self) -> &Vec<Vec<Option<bool>>> {
        &self.map
    }

    pub fn game_status(&self) -> GameStatus {
        self.game_status
    }

    pub fn make_move(&mut self, row: u8, col: u8) -> Result<AnalyzeResult, String> {
        match self.game_status {
			GameStatus::GameFinished => Err(String::from("Game already finished")),
            GameStatus::MoveForX | GameStatus::MoveForO => {
				self.validate_coordinates(row, col)?;
				self.validate_cell_not_set(row, col)?;

                let is_move_for_x = self.game_status == GameStatus::MoveForX;
				self.map[row as usize][col as usize] = Some(is_move_for_x);
                
				let analyze_result = self.analyze_map();
                self.game_status = match analyze_result {
					AnalyzeResult::Win(_) | AnalyzeResult::Draw => GameStatus::GameFinished,
					AnalyzeResult::ContinueGame => if is_move_for_x {
                        GameStatus::MoveForO
                    } else {
                        GameStatus::MoveForX
                    }
                };

				Ok(analyze_result)
            }
        }
    }

    fn validate_coordinates(&self, row: u8, col: u8) -> Result<(), String> {
        if row as usize > self.map.len() - 1 || col as usize > self.map.len() - 1 {
            return Err(format!("Invalid indexes: {row}, {col}"));
        } else {
            Ok(())
        }
    }

    fn validate_cell_not_set(&self, row: u8, col: u8) -> Result<(), String> {
        if self.map[row as usize][col as usize].is_some() {
            return Err(String::from("Element already set"));
        } else {
            Ok(())
        }
    }

    fn analyze_map(&self) -> AnalyzeResult {
        for line in self.map.iter() {
            if let Some(winner) = check_line_winner(line.iter()) {
                return AnalyzeResult::Win(winner);
            }
        }

        let map_len = self.map.len();
        for col_index in 0..map_len {
            let column = (0..map_len).map(|i| &self.map[i][col_index]).into_iter();
            if let Some(winner) = check_line_winner(column) {
                return AnalyzeResult::Win(winner);
            }
        }

        let diagonal = (0..map_len).map(|i| &self.map[i][i]).into_iter();
        if let Some(winner) = check_line_winner(diagonal) {
            return AnalyzeResult::Win(winner);
        }

        let diagonal = (0..map_len)
            .map(|i| &self.map[i][map_len - i - 1])
            .into_iter();
        if let Some(winner) = check_line_winner(diagonal) {
            return AnalyzeResult::Win(winner);
        }

        if self
            .map
            .iter()
            .all(|line| line.iter().all(|elem| elem.is_some()))
        {
            return AnalyzeResult::Draw;
        } else {
            return AnalyzeResult::ContinueGame;
        }

        fn check_line_winner<'a>(mut line: impl Iterator<Item = &'a Option<bool>>) -> Option<bool> {
            let first = *line.next()?;
            if first.is_none() || line.any(|next| *next != first) {
                None
            } else {
                first
            }
        }
    }
}

pub enum AnalyzeResult {
    Win(bool),
    Draw,
    ContinueGame,
}

#[derive(PartialEq, Clone, Copy)]
pub enum GameStatus {
    MoveForX,
    MoveForO,
    GameFinished,
}
