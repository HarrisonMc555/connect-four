use game::*;
use std::io;

struct Args {
    num_teams: usize,
    num_rows: usize,
    num_cols: usize,
    num_in_row: usize,
    first_turn: Team,
}

pub fn run() {
    println!("Running CLI");
    let mut game = get_game_from_user();
    while !game.game_over() {
        play_turn(&mut game);
    }
    display_end(&game);
}

fn play_turn(game: &mut GameState) {
    display_board(&game);
    println!();
    let team = game.cur_turn();
    println!("{}'s turn:", team);
    loop {
        let col = get_usize_from_user_in_range(
            "the column to drop tile in",
            0,
            game.num_cols(),
        );
        match game.drop_chip(team, col) {
            Ok(_) => break,
            Err(e) => print_error(e),
        }
    }
}

fn display_end(game: &GameState) {
    clear_screen();
    display_board(&game);
    println!();
    println!("{} wins!", game.cur_turn());
}

fn display_board(game: &GameState) {
    clear_screen();
    println!("{}", grid_string(&game));
}

fn grid_string(game: &GameState) -> String {
    let grid_s = game.to_string_arr()
        .into_iter()
        .rev()
        .map(|row| row.chars()
             .map(|c| char::to_string(&c)).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n");
    let header: String = (0..game.num_cols()).map(|i| format!("{:X}",
                                                              i)).collect::<Vec<_>>().join(" ");
    let lines = "-".repeat(game.num_cols()*2 - 1);
    format!("{}\n{}\n{}", header, lines, grid_s)
}

fn print_error(err: Error) {
    let message = match err {
        Error::OutOfBounds => "that column was out of bounds",
        Error::ColumnFull => "that column was full",
        Error::NotThatTeamsTurn => "it was not that team's turn",
        Error::InvalidTeam => "that was not a valid team",
        Error::GameOver => "the game was already over",
    };
    println!("That was an invalid move because {}, try again.", message);
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn get_game_from_user() -> GameState {
    clear_screen();
    loop {
        let args = get_args_from_user();
        let game_opt = match args {
            Some(a) => GameState::new(
                a.first_turn,
                a.num_teams,
                a.num_rows,
                a.num_cols,
                a.num_in_row,
            ),
            None => Ok(GameState::default()),
        };
        if let Ok(game_ok) = game_opt {
            return game_ok;
        } else {
            println!("Invalid game parameters, try again.");
        }
    }
}

fn get_args_from_user() -> Option<Args> {
    println!("Use default setup?");
    let yes = get_yes_no_from_user("whether to use the default setup");
    if yes {
        return None;
    }
    let num_teams = get_usize_from_user_in_range("the number of teams", 0, 16);
    let num_rows = get_usize_from_user("the number of rows");
    let num_cols = get_usize_from_user("the number of columns");
    let num_in_row = get_usize_from_user("the number in a row to win");
    let first_turn =
        get_usize_from_user_in_range("the team to go first", 0, num_teams);
    Some(Args {
        num_teams,
        num_rows,
        num_cols,
        num_in_row,
        first_turn: Team::new(first_turn),
    })
}

fn get_yes_no_from_user(message: &str) -> bool {
    println!("Please enter yes or no for {}.", message);
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");
        match input.trim().to_lowercase().as_ref() {
            "yes" => return true,
            "y" => return true,
            "no" => return false,
            "n" => return false,
            _ => println!("Not a yes or no answer, try again."),
        }
    }
}

fn get_usize_from_user(message: &str) -> usize {
    println!("Please enter a number for {}.", message);
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");
        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => println!("Not a valid number, try again."),
        }
    }
}

fn get_usize_from_user_in_range(
    message: &str,
    min_val: usize,
    max_val: usize,
) -> usize {
    if min_val + 1 > max_val {
        panic!("Impossible range");
    }
    println!(
        "Please enter a number for {} between {} and {}.",
        message,
        min_val,
        max_val - 1
    );
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");
        match input.trim().parse() {
            Ok(n) => if min_val <= n && n < max_val {
                return n;
            } else {
                println!(
                    "Not between {} and {}, try again.",
                    min_val,
                    max_val - 1
                );
            },
            Err(_) => println!("Not a valid number, try again."),
        }
    }
}
