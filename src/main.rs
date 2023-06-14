use std::env;
use std::io;
use std::io::{Error, ErrorKind};
use tic_tac_toe::*;

fn main() {
    let difficulty = get_difficulty().unwrap_or(Difficulty::Medium);
    let map_size = get_map_size().unwrap_or(3);
    let mut game = match TicTacToe::build(map_size, difficulty) {
        Ok(g) => g,
        Err(message) => {
            println!("{message}");
            return;
        }
    };

    println!("Welcome to tic tac toe. Difficulty is {}. Here's the map:", difficulty);

    print_map(game.get_map());

    loop {
        let move_result = match game.game_status() {
            GameStatus::GameFinished => {
                println!("Game already finished");
                return;
            },
            GameStatus::MoveForX => make_manual_move(&mut game, true),
            GameStatus::MoveForO => if difficulty == Difficulty::ManualOnly {
				make_manual_move(&mut game, false)
            } else {
                make_auto_move(&mut game, false)
            },
        };

		match move_result {
			Ok(result) => {
				print_map(game.get_map());
				match result {
					AnalyzeResult::Win(winner) => {
						println!("{} Win!!!!!!", if winner { "X" } else { "O" });
						return;
					}
					AnalyzeResult::Draw => {
						println!("Draw...");
						return;
					}
					AnalyzeResult::ContinueGame => (),
				}
			}
			Err(message) => println!("{message}"),
		};
    }
}

fn make_manual_move(game: &mut TicTacToe, is_move_for_x: bool) -> Result<AnalyzeResult, String> {
    println!();
    println!(
        "Move for {}",
        if is_move_for_x {
            'x'
        } else {
            'o'
        }
    );

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let (i, j) = parse_input(&input).map_err(|err| err.to_string())?;
	Ok(game.manual_move(i, j)?)
}

fn make_auto_move(game: &mut TicTacToe, is_move_for_x: bool) -> Result<AnalyzeResult, String> {
    println!();
    println!(
        "Move for {} (automatic)",
        if is_move_for_x {
            'x'
        } else {
            'o'
        }
    );

    game.auto_move()
}

fn get_difficulty() -> Option<Difficulty> {
    env::args().skip(1).next()?.parse().ok()
}

fn get_map_size() -> Option<u8> {
    env::args().skip(2).next()?.parse().ok()
}

fn parse_input(input: &str) -> Result<(u8, u8), Box<dyn std::error::Error>> {
    let mut split = input.trim().split(' ');
    let row = split.next().ok_or("'row' is not provided")?.parse()?;
    let column = split.next().ok_or("'column' is not provided")?.parse()?;

    if let Some(next_param) = split.next() {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            format!("Redundant parameter: {next_param}"),
        )));
    }

    return Ok((row, column));
}

fn print_map(map: &Vec<Vec<Option<bool>>>) {
    print!("  ");
    (0..map.len()).for_each(|i| print!("{i} "));
    println!();
    let delimiter = format!("  {}", "-".repeat(map.len() * 2 - 1));
    for (i, line) in map.iter().enumerate() {
        print!("{i} ");
        for (j, cell) in line.iter().enumerate() {
            print!("{}", get_symbol(cell));
            if j < map.len() - 1 {
                print!("|");
            } else {
                println!();
            }
        }

        if i < map.len() - 1 {
            println!("{delimiter}");
        } else {
            println!();
        }
    }

    fn get_symbol(value: &Option<bool>) -> char {
        match value {
            Some(true) => 'x',
            Some(false) => 'o',
            None => ' ',
        }
    }
}
