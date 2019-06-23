extern crate connect_four;

use connect_four::game::*;

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
    for i in 0..DEFAULT_NUM_COLUMNS {
        let col1 = (i * 2) % DEFAULT_NUM_COLUMNS;
        let col2 = (i * 2 + 1) % DEFAULT_NUM_COLUMNS;
        game.drop_chip(Team::new(0), col1)?;
        game.drop_chip(Team::new(1), col2)?;
    }
    Ok(())
}

#[test]
fn win_horizontally() -> Result<(), Error> {
    let mut game = GameState::default();
    for i in 0..DEFAULT_WINNING_LENGTH {
        // Team 0
        game.drop_chip(Team::new(0), i)?;
        // Team 1
        if i < DEFAULT_WINNING_LENGTH - 1 {
            game.drop_chip(Team::new(1), i)?;
        }
    }
    // Team 0 should have won
    assert!(game.game_over());
    assert!(game.has_won(Team::new(0)));
    assert_eq!(game.who_won(), Some(Team::new(0)));
    Ok(())
}

#[test]
fn win_vertically() -> Result<(), Error> {
    let mut game = GameState::default();
    for i in 0..DEFAULT_WINNING_LENGTH {
        // Team 0
        game.drop_chip(Team::new(0), 0)?;
        // Team 1
        if i < DEFAULT_WINNING_LENGTH - 1 {
            game.drop_chip(Team::new(1), 1)?;
        }
    }
    // Team 0 should have won
    assert!(game.game_over());
    assert!(game.has_won(Team::new(0)));
    assert_eq!(game.who_won(), Some(Team::new(0)));
    Ok(())
}

#[test]
fn win_diagonally_up_right() -> Result<(), Error> {
    let mut game = GameState::default();
    // Only valid when DEFAULT_WINNING_LENGTH == 4
    game.drop_chip(Team::new(0), 0)?; // Team 0 (0, 0)
    game.drop_chip(Team::new(1), 1)?; // Team 1 (0, 1)
    game.drop_chip(Team::new(0), 1)?; // Team 0 (1, 1)
    game.drop_chip(Team::new(1), 2)?; // Team 1 (0, 2)
    game.drop_chip(Team::new(0), 2)?; // Team 0 (1, 2)
    game.drop_chip(Team::new(1), DEFAULT_NUM_COLUMNS - 1)?; // Team 1 (0, -1)
    game.drop_chip(Team::new(0), 2)?; // Team 0 (2, 2)
    game.drop_chip(Team::new(1), 3)?; // Team 1 (0, 3)
    game.drop_chip(Team::new(0), 3)?; // Team 0 (1, 3)
    game.drop_chip(Team::new(1), 3)?; // Team 1 (2, 3)
    game.drop_chip(Team::new(0), 3)?; // Team 0 (3, 3)

    // Team 0 should have won
    assert!(game.game_over());
    assert!(game.has_won(Team::new(0)));
    assert_eq!(game.who_won(), Some(Team::new(0)));
    Ok(())
}

#[test]
fn win_diagonally_up_left() -> Result<(), Error> {
    let mut game = GameState::default();
    // Only valid when DEFAULT_WINNING_LENGTH == 4
    game.drop_chip(Team::new(0), 3)?; // Team 0 (0, 3)
    game.drop_chip(Team::new(1), 2)?; // Team 1 (0, 2)
    game.drop_chip(Team::new(0), 2)?; // Team 0 (1, 2)
    game.drop_chip(Team::new(1), 1)?; // Team 1 (0, 1)
    game.drop_chip(Team::new(0), 1)?; // Team 0 (1, 1)
    game.drop_chip(Team::new(1), DEFAULT_NUM_COLUMNS - 1)?; // Team 1 (0, -1)
    game.drop_chip(Team::new(0), 1)?; // Team 0 (2, 1)
    game.drop_chip(Team::new(1), 0)?; // Team 1 (0, 0)
    game.drop_chip(Team::new(0), 0)?; // Team 0 (1, 0)
    game.drop_chip(Team::new(1), 0)?; // Team 1 (2, 0)
    game.drop_chip(Team::new(0), 0)?; // Team 0 (3, 0)

    // Team 0 should have won
    // print_grid(&game, "");
    assert!(game.game_over());
    assert!(game.has_won(Team::new(0)));
    assert_eq!(game.who_won(), Some(Team::new(0)));
    Ok(())
}

#[test]
fn drop_out_of_bounds() {
    let mut game = GameState::default();
    let result = game.drop_chip(Team::new(0), 0);
    assert!(result.is_ok());
    let result = game.drop_chip(Team::new(1), 1);
    assert!(result.is_ok());
    let result = game.drop_chip(Team::new(0), DEFAULT_NUM_COLUMNS);
    assert_eq!(result, Err(Error::OutOfBounds));
    let result = game.drop_chip(Team::new(0), DEFAULT_NUM_COLUMNS + 1);
    assert_eq!(result, Err(Error::OutOfBounds));
    let result = game.drop_chip(Team::new(0), DEFAULT_NUM_COLUMNS * 2 + 1);
    assert_eq!(result, Err(Error::OutOfBounds));
    let result = game.drop_chip(Team::new(0), 0);
    assert!(result.is_ok());
    let result = game.drop_chip(Team::new(1), 1);
    assert!(result.is_ok());
}

#[test]
fn drop_with_wrong_team() {
    let mut game = GameState::default();
    let result = game.drop_chip(Team::new(0), 0);
    assert!(result.is_ok());
    let result = game.drop_chip(Team::new(1), 1);
    assert!(result.is_ok());
    let result = game.drop_chip(Team::new(1), 2);
    assert_eq!(result, Err(Error::NotThatTeamsTurn));
    let result = game.drop_chip(Team::new(0), 0);
    assert!(result.is_ok());
    let result = game.drop_chip(Team::new(1), 1);
    assert!(result.is_ok());
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
    for i in 0..DEFAULT_WINNING_LENGTH {
        // Team 0
        game.drop_chip(Team::new(0), i)?;
        if i < DEFAULT_WINNING_LENGTH - 1 {
            // Team 1
            game.drop_chip(Team::new(1), i)?;
        }
    }
    let result = game.drop_chip(Team::new(1), 0);
    assert_eq!(result, Err(Error::GameOver));
    Ok(())
}

#[test]
fn custom_game() -> Result<(), Error> {
    let winning_length = 6;
    let mut game = GameState::new(Team::new(2), 3, 10, 9, winning_length)?;
    game.drop_chip(Team::new(2), 0)?;
    for i in 0..winning_length - 1 {
        game.drop_chip(Team::new(0), i)?;
        game.drop_chip(Team::new(1), i)?;
        game.drop_chip(Team::new(2), i + 1)?;
    }
    // print_grid(&game, "");
    assert!(game.game_over());
    Ok(())
}

#[test]
#[ignore]
fn sample_game() -> Result<(), Error> {
    let mut game = GameState::default();
    for i in 0..DEFAULT_WINNING_LENGTH {
        println!("\ti: {}", i);
        println!("\tbefore team 0 drop:");
        print_grid(&game, "\t\t");
        // Team 0
        let cur_turn = game.cur_turn();
        game.drop_chip(cur_turn, i)?;
        println!("\tafter team 0 drop:");
        print_grid(&game, "\t\t");
        // Team 1
        if i < DEFAULT_WINNING_LENGTH - 1 {
            println!("\tabout to drop team 1:");
            let cur_turn = game.cur_turn();
            game.drop_chip(cur_turn, i)?;
            println!("\tafter team 1 drop:");
            print_grid(&game, "\t\t");
        }
    }
    // Team 0 should have won
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
        .collect::<Vec<_>>()
        .join("\n")
}
