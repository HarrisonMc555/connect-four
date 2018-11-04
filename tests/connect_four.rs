extern crate connect_four;

use connect_four::*;

#[test]
fn create_game_state() -> Result<(), ()> {
    let _game = GameState::default();
    Ok(())
}

#[test]
fn drop_chip() -> Result<(), Error> {
    let mut game = GameState::default();
    game.drop_chip(DEFAULT_FIRST_TURN, 0)?;
    Ok(())
}

#[test]
fn drop_multiple_chips() -> Result<(), Error> {
    let mut game = GameState::default();
    for i in 0..DEFAULT_NUM_COLS {
        let col1 = (i*2) % DEFAULT_NUM_COLS;
        let col2 = (i*2 + 1) % DEFAULT_NUM_COLS;
        game.drop_chip(Team::Team1, col1)?;
        game.drop_chip(Team::Team2, col2)?;
    }
    Ok(())
}

#[test]
fn win_horizontally() -> Result<(), Error> {
    let mut game = GameState::default();
    for i in 0..DEFAULT_NUM_IN_ROW {
        // Team 1
        game.drop_chip(Team::Team1, i)?;
        // Team 2
        if i < DEFAULT_NUM_IN_ROW - 1 {
            game.drop_chip(Team::Team2, i)?;
        }
    }
    // Team 1 should have won
    assert!(game.game_over());
    assert!(game.has_won(Team::Team1));
    assert_eq!(game.who_won(), Some(Team::Team1));
    Ok(())
}

#[test]
fn win_vertically() -> Result<(), Error> {
    let mut game = GameState::default();
    for i in 0..DEFAULT_NUM_IN_ROW {
        // Team 1
        game.drop_chip(Team::Team1, 0)?;
        // Team 2
        if i < DEFAULT_NUM_IN_ROW - 1 {
            game.drop_chip(Team::Team2, 1)?;
        }
    }
    // Team 1 should have won
    assert!(game.game_over());
    assert!(game.has_won(Team::Team1));
    assert_eq!(game.who_won(), Some(Team::Team1));
    Ok(())
}

#[test]
fn win_diagonally() -> Result<(), Error> {
    let mut game = GameState::default();
    // Only valid when DEFAULT_NUM_IN_ROW == 4
    game.drop_chip(Team::Team1, 0)?; // Team 1 (0, 0)
    game.drop_chip(Team::Team2, 1)?; // Team 2 (0, 1)
    game.drop_chip(Team::Team1, 1)?; // Team 1 (1, 1)
    game.drop_chip(Team::Team2, 2)?; // Team 2 (0, 2)
    game.drop_chip(Team::Team1, 2)?; // Team 1 (1, 2)
    game.drop_chip(Team::Team2, DEFAULT_NUM_COLS - 1)?; // Team 2 (0, -1)
    game.drop_chip(Team::Team1, 2)?; // Team 1 (2, 2)
    game.drop_chip(Team::Team2, 3)?; // Team 2 (0, 3)
    game.drop_chip(Team::Team1, 3)?; // Team 1 (1, 3)
    game.drop_chip(Team::Team2, 3)?; // Team 2 (2, 3)
    game.drop_chip(Team::Team1, 3)?; // Team 1 (3, 3)
    // Team 1 should have won
    assert!(game.game_over());
    assert!(game.has_won(Team::Team1));
    assert_eq!(game.who_won(), Some(Team::Team1));
    Ok(())
}

#[test]
fn drop_out_of_bounds() {
    let mut game = GameState::default();
    let result = game.drop_chip(Team::Team1, DEFAULT_NUM_COLS);
    assert_eq!(result, Err(Error::OutOfBounds));
    let result = game.drop_chip(Team::Team1, DEFAULT_NUM_COLS + 1);
    assert_eq!(result, Err(Error::OutOfBounds));
    let result = game.drop_chip(Team::Team1, DEFAULT_NUM_COLS*2 + 1);
    assert_eq!(result, Err(Error::OutOfBounds));
}

#[test]
fn drop_in_full_column() {
    let mut game = GameState::default();
    for _ in 0..DEFAULT_NUM_ROWS {
        let cur_turn = game.cur_turn();
        let result = game.drop_chip(cur_turn, 0);
        assert!(result.is_ok());
    }
    // Column should be filled, now should overflow
    let cur_turn = game.cur_turn();
    let result = game.drop_chip(cur_turn, 0);
    assert_eq!(result, Err(Error::ColumnFull));
    // If we try again, we should get the same error
    let cur_turn = game.cur_turn();
    let result = game.drop_chip(cur_turn, 0);
    assert_eq!(result, Err(Error::ColumnFull));
    // If we try a different column, it should be fine
    let cur_turn = game.cur_turn();
    let result = game.drop_chip(cur_turn, 1);
    assert!(result.is_ok());
}

#[test]
fn no_drops_after_game_over() -> Result<(), Error> {
    let mut game = GameState::default();
    for i in 0..DEFAULT_NUM_IN_ROW {
        // Team 1
        game.drop_chip(Team::Team1, i)?;
        if i < DEFAULT_NUM_IN_ROW - 1 {
            // Team 2
            game.drop_chip(Team::Team2, i)?;
        }
    }
    let result = game.drop_chip(Team::Team2, 0);
    assert_eq!(result, Err(Error::GameOver));
    Ok(())
}

#[test]
fn custom_size() -> Result<(), Error> {
    let mut game = GameState::new(Team::Team2, 10, 9, 6);
    let num_cols = game.num_cols();
    let num_in_row = game.num_in_row();
    game.drop_chip(Team::Team2, num_cols - 1)?;
    for i in 0..num_in_row {
        game.drop_chip(Team::Team1, i)?;
        if i < num_in_row - 1 {
            game.drop_chip(Team::Team2, i)?;
        }
    }
    assert!(game.game_over());
    Ok(())
}

#[test]
#[ignore]
fn sample_game() -> Result<(), Error> {
    let mut game = GameState::default();
    for i in 0..DEFAULT_NUM_IN_ROW {
        println!("\ti: {}", i);
        println!("\tbefore team 1 drop:");
        print_grid(&game, "\t\t");
        // Team 1
        let cur_turn = game.cur_turn();
        game.drop_chip(cur_turn, i)?;
        println!("\tafter team 1 drop:");
        print_grid(&game, "\t\t");
        // Team 2
        if i < DEFAULT_NUM_IN_ROW - 1 {
            println!("\tabout to drop team 2:");
            let cur_turn = game.cur_turn();
            game.drop_chip(cur_turn, i)?;
            println!("\tafter team 2 drop:");
            print_grid(&game, "\t\t");
        }
    }
    // Team 1 should have won
    println!("final grid:");
    print_grid(&game, "");
    // Quit so we can see
    assert!(false);
    Ok(())
}

fn print_grid(game: &GameState, prefix: &str) {
    println!("{}", grid_string(game, prefix))
}

fn grid_string(game: &GameState, prefix: &str) -> String {
    game.to_string_arr()
        .into_iter()
        .rev()
        .map(|s| prefix.to_owned() + &s)
        .collect::<Vec<_>>().join("\n")
}
