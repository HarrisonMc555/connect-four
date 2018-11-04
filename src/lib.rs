pub const DEFAULT_FIRST_TURN: Team = Team(0);
pub const DEFAULT_NUM_TEAMS: usize = 2;
pub const DEFAULT_NUM_IN_ROW: usize = 4;
pub const DEFAULT_NUM_ROWS: usize = 6;
pub const DEFAULT_NUM_COLS: usize = 7;

const DEFAULT_EMPTY_CHAR: char = '_';

pub type Grid = Vec<Vec<Cell>>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    OutOfBounds,
    ColumnFull,
    NotThatTeamsTurn,
    InvalidTeam,
    GameOver,
}

type Cell = Option<Team>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Team(pub usize);

pub struct GameState {
    cells: Grid,
    cur_turn: Team,
    num_teams: usize,
    num_in_row: usize,
    // num_rows is determined by the length of `cells`.
    num_cols: usize,
}

impl GameState {
    pub fn default() -> GameState {
        GameState {
            cells: GameState::create_empty_grid(
                DEFAULT_NUM_ROWS,
                DEFAULT_NUM_COLS,
            ),
            cur_turn: DEFAULT_FIRST_TURN,
            num_teams: DEFAULT_NUM_TEAMS,
            num_cols: DEFAULT_NUM_COLS,
            num_in_row: DEFAULT_NUM_IN_ROW,
        }
    }

    pub fn new(
        first_turn: Team,
        num_teams: usize,
        num_rows: usize,
        num_cols: usize,
        num_in_row: usize,
    ) -> Result<GameState, Error> {
        if first_turn.0 > num_teams {
            return Err(Error::InvalidTeam);
        }
        Ok(GameState {
            cells: GameState::create_empty_grid(num_rows, num_cols),
            num_teams,
            cur_turn: first_turn,
            num_cols,
            num_in_row,
        })
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

    pub fn drop_chip(&mut self, team: Team, col: usize) -> Result<(), Error> {
        if self.game_over() {
            return Err(Error::GameOver);
        }
        if self.cur_turn != team {
            return Err(Error::NotThatTeamsTurn);
        }
        self.drop_chip_cells(col)?;
        self.cur_turn = self.next_turn();
        Ok(())
    }

    pub fn game_over(&self) -> bool {
        (0..self.num_teams).any(|team_num| self.has_won(Team(team_num)))
    }

    pub fn has_won(&self, team: Team) -> bool {
        self.has_won_vertically(team)
            || self.has_won_horizontally(team)
            || self.has_won_diagonally(team)
    }

    pub fn who_won(&self) -> Option<Team> {
        (0..self.num_teams).map(|team_num| Team(team_num))
            .find(|&team| self.has_won(team))
    }

    pub fn to_string_arr(&self) -> Vec<String> {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&cell| GameState::cell_to_char(cell))
                    .collect()
            }).collect()
    }

    fn has_won_vertically(&self, team: Team) -> bool {
        self.cells.windows(self.num_in_row).any(|rows| {
            (0..self.num_cols)
                .any(|index| rows.iter().all(|row| row[index] == Some(team)))
        })
    }

    fn has_won_horizontally(&self, team: Team) -> bool {
        self.cells.iter().any(|row| {
            row.windows(self.num_in_row)
                .any(|slice| slice.iter().all(|&c| c == Some(team)))
        })
    }

    fn has_won_diagonally(&self, team: Team) -> bool {
        self.cells.windows(self.num_in_row).any(|rows| {
            (0..self.num_cols - self.num_in_row + 1).any(|offset| {
                rows.iter()
                    .enumerate()
                    .all(|(index, row)| row[offset + index] == Some(team))
                    || rows.iter().enumerate().all(|(index, row)| {
                        row[offset + self.num_in_row - index - 1] == Some(team)
                    })
            })
        })
    }

    fn cell_to_char(cell: Cell) -> char {
        if let Some(team) = cell {
            if team.0 > 16 {
                panic!("Cannot convert team {} to a char");
            }
        }
        match cell {
            Some(team) => std::char::from_digit(team.0 as u32, 16).unwrap(),
            None => DEFAULT_EMPTY_CHAR,
        }
    }

    fn next_turn(&self) -> Team {
        let next_team_num = (self.cur_turn.0 + 1) % self.num_teams;
        Team(next_team_num)
    }

    fn drop_chip_cells(&mut self, col: usize) -> Result<(), Error> {
        if col >= self.num_cols {
            return Err(Error::OutOfBounds);
        }
        let row = self.highest_unfilled_row(col)?;
        self.cells[row][col] = Some(self.cur_turn);
        Ok(())
    }

    fn highest_unfilled_row(&self, col: usize) -> Result<usize, Error> {
        self.cells
            .iter()
            .enumerate()
            .find(|(_, row)| row[col] == None)
            .map(|(index, _)| index)
            .ok_or(Error::ColumnFull)
    }

    fn create_empty_grid(num_rows: usize, num_cols: usize) -> Grid {
        (0..num_rows)
            .map(|_| (0..num_cols).map(|_| None).collect())
            .collect()
    }
}
