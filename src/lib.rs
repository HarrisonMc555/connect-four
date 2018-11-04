pub const DEFAULT_NUM_ROWS: usize = 6;
pub const DEFAULT_NUM_COLS: usize = 7;
pub const DEFAULT_NUM_IN_ROW: usize = 4;
pub const DEFAULT_FIRST_TURN: Team = Team::Team1;

pub type Grid = Vec<Vec<Cell>>;

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
    num_cols: usize,
    num_in_row: usize,
}

impl GameState {
    pub fn default() -> GameState {
        GameState {
            cells: GameState::create_empty_grid(DEFAULT_NUM_ROWS, DEFAULT_NUM_COLS),
            cur_turn: DEFAULT_FIRST_TURN,
            num_cols: DEFAULT_NUM_COLS,
            num_in_row: DEFAULT_NUM_IN_ROW,
        }
    }

    pub fn new(first_turn: Team, num_rows: usize, num_cols: usize,
               num_in_row: usize) -> GameState {
        GameState {
            cells: GameState::create_empty_grid(num_rows, num_cols),
            cur_turn: first_turn,
            num_cols,
            num_in_row,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.cells.len()
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn num_in_row(&self) -> usize {
        self.num_in_row
    }

    pub fn grid(&self) -> &Grid {
        &self.cells
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
                 .map(|&cell| GameState::cell_to_char(cell))
                 .collect())
            .collect()
        // self.cells
        //     .iter()
        //     .map(|row| row.iter()
        //          .map(|&cell| GameState::cell_to_char(cell).to_string())
        //          .collect::<Vec<_>>()
        //          .join(" ")
        //     )
        //     .collect()
    }

    fn has_won_vertically(&self, team: Team) -> bool {
        self.cells
            .windows(self.num_in_row)
            .any(|rows| (0..self.num_cols)
                 .any(|index| rows.iter()
                      .all(|row| row[index] == Some(team))))
    }

    fn has_won_horizontally(&self, team: Team) -> bool {
        self.cells
            .iter()
            .any(|row|
                 row.windows(self.num_in_row)
                 .any(|slice| slice.iter()
                      .all(|&c| c == Some(team))))
    }

    fn has_won_diagonally(&self, team: Team) -> bool {
        self.cells
            .windows(self.num_in_row)
            .any(|rows| (0..self.num_cols - self.num_in_row + 1)
                 .any(|offset| rows.iter()
                      .enumerate()
                      .all(|(index, row)|
                           row[index + offset] == Some(team))))
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
            if col >= self.num_cols {
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

    fn create_empty_grid(num_rows: usize, num_cols: usize) -> Grid {
        (0..num_rows).map(|_| (0..num_cols).map(|_| None).collect()).collect()
    }
}
