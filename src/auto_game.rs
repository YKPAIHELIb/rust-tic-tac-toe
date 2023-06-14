mod easy_strategy;
mod medium_strategy;

use super::manual_game::{AnalyzeResult, GameStatus, TicTacToeManual};
use easy_strategy::EasyStrategy;
use medium_strategy::MediumStrategy;

use std::str::FromStr;
use std::fmt;

trait TicTacToeGameReader {
    fn get_map(&self) -> &Vec<Vec<Option<bool>>>;
    fn game_status(&self) -> GameStatus;
}

impl TicTacToeGameReader for TicTacToeManual {
    fn get_map(&self) -> &Vec<Vec<Option<bool>>> {
        TicTacToeManual::get_map(self)
    }

    fn game_status(&self) -> GameStatus {
        TicTacToeManual::game_status(self)
    }
}

trait TicTacToeStrategy<T: TicTacToeGameReader> {
    fn suggest_next_move(&self, game_reader: &T) -> Result<(u8, u8), String>;
}

#[derive(PartialEq, Clone, Copy)]
pub enum Difficulty {
    ManualOnly,
    Easy,
    Medium,
    Hard,
}

impl FromStr for Difficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().trim() {
			"manual" => Ok(Difficulty::ManualOnly),
			"easy" => Ok(Difficulty::Easy),
			"medium" => Ok(Difficulty::Medium),
			"hard" => Ok(Difficulty::Hard),
			_ => Err(())
		}
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::ManualOnly => write!(f, "Manual Game"),
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

pub struct TicTacToe {
    manual_game: TicTacToeManual,
    auto_move_strategy: Option<Box<dyn TicTacToeStrategy<TicTacToeManual>>>,
}

impl TicTacToe {
    pub fn build(map_size: u8, difficulty: Difficulty) -> Result<Self, &'static str> {
        Ok(Self {
            manual_game: TicTacToeManual::build(map_size)?,
            auto_move_strategy: match difficulty {
                Difficulty::ManualOnly => None,
                Difficulty::Easy => Some(Box::new(EasyStrategy)),
                Difficulty::Medium => Some(Box::new(MediumStrategy)),
                Difficulty::Hard => todo!(),
            },
        })
    }

    pub fn get_map(&self) -> &Vec<Vec<Option<bool>>> {
        &self.manual_game.get_map()
    }

    pub fn game_status(&self) -> GameStatus {
        self.manual_game.game_status()
    }

    pub fn manual_move(&mut self, row: u8, col: u8) -> Result<AnalyzeResult, String> {
        self.manual_game.make_move(row, col)
    }

    pub fn auto_move(&mut self) -> Result<AnalyzeResult, String> {
        let (row, col) = self
            .auto_move_strategy
            .as_ref()
            .ok_or("Manual Game selected")?
            .suggest_next_move(&self.manual_game)?;

        self.manual_game.make_move(row, col)
    }
}
