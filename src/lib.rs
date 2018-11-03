pub const NUM_ROWS: usize = 6;
pub const NUM_COLS: usize = 7;
pub const NUM_IN_ROW: usize = 4;

pub type Grid = [[Cell; NUM_COLS]; NUM_ROWS];

#[derive (Copy, Clone, Debug, PartialEq)]
pub enum Error {
    OutOfBounds,
    ColumnFull,
    NotThatTeamsTurn,
    GameOver,
}

type Cell = Option<Team>;

#[derive (Copy, Clone, PartialEq, Debug)]
pub enum Team {
    Team1,
    Team2,
}

pub struct GameState {
    cells: Grid,
    cur_turn: Team,
}

impl GameState {
    pub fn new(first_turn: Team) -> GameState {
        GameState {
            cells: [[None; NUM_COLS]; NUM_ROWS],
            cur_turn: first_turn,
        }
    }

    pub fn grid(&self) -> Grid {
        self.cells
    }

    pub fn cur_turn(&self) -> Team {
        self.cur_turn
    }

    pub fn drop_chip(&mut self, col: usize) -> Result<(), Error> {
        if self.game_over() {
            return Err(Error::GameOver);
        }
        self.drop_chip_cells(col)?;
        self.cur_turn = GameState::next_turn(self.cur_turn);
        Ok(())
    }

    pub fn game_over(&self) -> bool {
        self.has_won(Team::Team1) || self.has_won(Team::Team2)
    }

    pub fn has_won(&self, team: Team) -> bool {
        self.has_won_vertically(team)
            || self.has_won_horizontally(team)
            || self.has_won_diagonally(team)
    }

    pub fn who_won(&self) -> Option<Team> {
        if self.has_won(Team::Team1) {
            Some(Team::Team1)
        } else if self.has_won(Team::Team2) {
            Some(Team::Team2)
        } else {
            None
        }
    }

    pub fn to_string_arr(&self) -> Vec<String> {
        self.cells
            .iter()
            .map(|row| row.iter()
                 .map(|&cell| GameState::cell_to_char(cell).to_string())
                 .collect::<Vec<_>>()
                 .join(" ")
            )
            .collect()
    }

    fn cell_to_char(cell: Cell) -> char {
        match cell {
            Some(Team::Team1) => 'O',
            Some(Team::Team2) => 'X',
            None => '_',
        }
    }

    fn next_turn(team: Team) -> Team {
        match team {
            Team::Team1 => Team::Team2,
            Team::Team2 => Team::Team1,
        }
    }

    fn drop_chip_cells(&mut self, col: usize) ->
        Result<(), Error> {
            if col >= NUM_COLS {
                return Err(Error::OutOfBounds);
            }
            let row = self.highest_unfilled_row(col)?;
            self.cells[row][col] = Some(self.cur_turn);
            Ok(())
        }

    fn highest_unfilled_row(&self, col: usize) ->
        Result<usize, Error> {
            self.cells
                .iter()
                .enumerate()
                .find(|(_, row)| row[col] == None)
                .map(|(index, _)| index)
                .ok_or(Error::ColumnFull)
        }

    fn has_won_vertically(&self, team: Team) -> bool {
        self.cells
            .windows(NUM_IN_ROW)
            .any(|rows| (0..NUM_COLS)
                 .any(|index| rows.iter()
                      .all(|row| row[index] == Some(team))))
    }

    fn has_won_horizontally(&self, team: Team) -> bool {
        self.cells
            .iter()
            .any(|row|
                 row.windows(NUM_IN_ROW)
                 .any(|slice| slice.iter()
                      .all(|&c| c == Some(team))))
    }

    fn has_won_diagonally(&self, team: Team) -> bool {
        self.cells
            .windows(NUM_IN_ROW)
            .any(|rows| (0..NUM_COLS - NUM_IN_ROW + 1)
                 .any(|offset| rows.iter()
                      .enumerate()
                      .all(|(index, row)|
                           row[index + offset] == Some(team))))
    }
}
