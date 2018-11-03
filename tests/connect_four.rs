extern crate connect_four;

use connect_four::*;

#[test]
fn create_game_state() -> Result<(), ()> {
    let _game = GameState::new(Team::Team1);
    Ok(())
}

#[test]
fn drop_chip() -> Result<(), Error> {
    let mut game = GameState::new(Team::Team1);
    game.drop_chip(0)?;
    Ok(())
}

#[test]
fn drop_multiple_chips() -> Result<(), Error> {
    let mut game = GameState::new(Team::Team1);
    for i in 0..2*NUM_COLS {
        let col = i % NUM_COLS;
        game.drop_chip(col)?;
    }
    Ok(())
}

#[test]
fn win_horizontally() -> Result<(), Error> {
    let mut game = GameState::new(Team::Team1);
    for i in 0..NUM_IN_ROW {
        // Team 1
        game.drop_chip(i)?;
        // Team 2
        if i < NUM_IN_ROW - 1 {
            game.drop_chip(i)?;
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
    let mut game = GameState::new(Team::Team1);
    for i in 0..NUM_IN_ROW {
        // Team 1
        game.drop_chip(0)?;
        // Team 2
        if i < NUM_IN_ROW - 1 {
            game.drop_chip(1)?;
        }
    }
    // Team 1 should have won
    assert!(game.game_over());
    assert!(game.has_won(Team::Team1));
    assert_eq!(game.who_won(), Some(Team::Team1));
    Ok(())
}

#[test]
fn drop_out_of_bounds() {
    let mut game = GameState::new(Team::Team1);
    let result = game.drop_chip(NUM_COLS);
    assert_eq!(result, Err(Error::OutOfBounds));
    let result = game.drop_chip(NUM_COLS + 1);
    assert_eq!(result, Err(Error::OutOfBounds));
    let result = game.drop_chip(NUM_COLS*2 + 1);
    assert_eq!(result, Err(Error::OutOfBounds));
}

#[test]
fn drop_in_full_column() {
    let mut game = GameState::new(Team::Team1);
    for _ in 0..NUM_ROWS {
        let result = game.drop_chip(0);
        assert!(result.is_ok());
    }
    // Column should be filled, now should overflow
    let result = game.drop_chip(0);
    assert_eq!(result, Err(Error::ColumnFull));
    // If we try again, we should get the same error
    let result = game.drop_chip(0);
    assert_eq!(result, Err(Error::ColumnFull));
    // If we try a different column, it should be fine
    let result = game.drop_chip(1);
    assert!(result.is_ok());
}

#[test]
fn no_drops_after_game_over() -> Result<(), Error> {
    let mut game = GameState::new(Team::Team1);
    for i in 0..NUM_IN_ROW {
        // Team 1
        game.drop_chip(i)?;
        if i < NUM_IN_ROW - 1 {
            // Team 2
            game.drop_chip(i)?;
        }
    }
    let result = game.drop_chip(0);
    assert_eq!(result, Err(Error::GameOver));
    Ok(())
}

#[test]
#[ignore]
fn sample_game() -> Result<(), Error> {
    let mut game = GameState::new(Team::Team1);
    for i in 0..NUM_IN_ROW {
        println!("\ti: {}", i);
        println!("\tbefore team 1 drop:");
        print_grid(&game, "\t\t");
        // Team 1
        game.drop_chip(i)?;
        println!("\tafter team 1 drop:");
        print_grid(&game, "\t\t");
        // Team 2
        if i < NUM_IN_ROW - 1 {
            println!("\tabout to drop team 2:");
            game.drop_chip(i)?;
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
    game.to_string_arr().into_iter().rev().map(|s| prefix.to_owned() + &s)
        .collect::<Vec<_>>().join("\n")
}
