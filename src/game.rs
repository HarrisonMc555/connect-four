use array2d::Array2D;
use itertools::iproduct;
use std::{char, fmt};

pub const DEFAULT_FIRST_TURN: Team = Team(0);
pub const DEFAULT_NUM_TEAMS: usize = 2;
pub const DEFAULT_WINNING_LENGTH: usize = 4;
pub const DEFAULT_NUM_ROWS: usize = 6;
pub const DEFAULT_NUM_COLUMNS: usize = 7;

pub const MAX_PRINTABLE_TEAMS: usize = 16;
const DEFAULT_EMPTY_CHAR: char = '_';

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
pub struct Team(usize);

impl Team {
    pub fn new(team: usize) -> Team {
        Team(team)
    }
}

pub struct GameState {
    cells: Array2D<Cell>,
    cur_turn: Team,
    num_teams: usize,
    winning_length: usize,
}

impl GameState {
    pub fn default() -> GameState {
        GameState::new(
            DEFAULT_FIRST_TURN,
            DEFAULT_NUM_TEAMS,
            DEFAULT_NUM_ROWS,
            DEFAULT_NUM_COLUMNS,
            DEFAULT_WINNING_LENGTH,
        )
        .unwrap()
    }

    pub fn new(
        first_turn: Team,
        num_teams: usize,
        num_rows: usize,
        num_columns: usize,
        winning_length: usize,
    ) -> Result<GameState, Error> {
        if first_turn.0 > num_teams {
            return Err(Error::InvalidTeam);
        }
        let rows = GameState::create_empty_grid_rows(num_rows, num_columns);
        Ok(GameState {
            cells: Array2D::from_rows(&rows),
            num_teams,
            cur_turn: first_turn,
            winning_length,
        })
    }

    #[allow(dead_code)]
    pub fn num_rows(&self) -> usize {
        self.cells.num_rows()
    }

    pub fn num_columns(&self) -> usize {
        self.cells.num_columns()
    }

    pub fn cur_turn(&self) -> Team {
        self.cur_turn
    }

    pub fn drop_chip(&mut self, team: Team, column: usize) -> Result<(), Error> {
        if self.game_over() {
            return Err(Error::GameOver);
        }
        if self.cur_turn != team {
            return Err(Error::NotThatTeamsTurn);
        }
        self.drop_chip_cells(column)?;
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
        (0..self.num_teams)
            .map(Team)
            .find(|&team| self.has_won(team))
    }

    pub fn to_string_arr(&self) -> Vec<String> {
        self.cells
            .rows_iter()
            .map(|row_iter| {
                row_iter
                    .map(|&cell| GameState::cell_to_char(cell))
                    .collect()
            })
            .collect()
    }

    fn has_won_vertically(&self, team: Team) -> bool {
        self.vertical_starting_coordinates()
            .map(|(row, column)| self.vertical_sequence_coordinates(row, column))
            .any(|coordinates| self.won_with_coordinates(coordinates, team))
    }

    fn has_won_horizontally(&self, team: Team) -> bool {
        self.horizontal_starting_coordinates()
            .map(|(row, column)| self.horizontal_sequence_coordinates(row, column))
            .any(|coordinates| self.won_with_coordinates(coordinates, team))
    }

    fn has_won_diagonally(&self, team: Team) -> bool {
        self.has_won_diagonally_up_left(team) || self.has_won_diagonally_up_right(team)
        // self.cells
        //     .as_rows()
        //     .windows(self.winning_length)
        //     .any(|rows| {
        //         (0..self.cells.num_columns() - self.winning_length + 1).any(|offset| {
        //             rows.iter()
        //                 .enumerate()
        //                 .all(|(index, row)| row[offset + index] == Some(team))
        //                 || rows.iter().enumerate().all(|(index, row)| {
        //                     row[offset + self.winning_length - index - 1] == Some(team)
        //                 })
        //         })
        //     })
    }

    fn has_won_diagonally_up_left(&self, team: Team) -> bool {
        self.diagonal_up_left_starting_coordinates()
            .map(|(row, column)| self.diagonal_up_left_sequence_coordinates(row, column))
            .any(|coordinates| self.won_with_coordinates(coordinates, team))
    }

    fn has_won_diagonally_up_right(&self, team: Team) -> bool {
        self.diagonal_up_right_starting_coordinates()
            .map(|(row, column)| self.diagonal_up_right_sequence_coordinates(row, column))
            .any(|coordinates| self.won_with_coordinates(coordinates, team))
    }

    fn won_with_coordinates<I>(&self, mut coordinates: I, team: Team) -> bool
    where
        I: Iterator<Item = (usize, usize)>,
    {
        coordinates.all(|coords| self.cells[coords] == Some(team))
    }

    fn vertical_starting_coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        iproduct!(
            0..=(self.num_rows() - self.winning_length),
            0..self.num_columns()
        )
    }

    fn vertical_sequence_coordinates(
        &self,
        row: usize,
        column: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let last_row = row + self.winning_length;
        (row..last_row).map(move |r| (r, column))
    }

    fn horizontal_starting_coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        iproduct!(
            0..self.num_rows(),
            0..=(self.num_columns() - self.winning_length)
        )
    }

    fn horizontal_sequence_coordinates(
        &self,
        row: usize,
        column: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let last_column = column + self.winning_length;
        (column..last_column).map(move |c| (row, c))
    }

    fn diagonal_up_left_starting_coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        iproduct!(
            0..=(self.num_rows() - self.winning_length),
            0..=(self.num_columns() - self.winning_length)
        )
    }

    fn diagonal_up_left_sequence_coordinates(
        &self,
        row: usize,
        column: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let last_row = row + self.winning_length;
        let last_column = column + self.winning_length;
        (row..last_row).zip(column..last_column)
    }

    fn diagonal_up_right_starting_coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        iproduct!(
            self.winning_length..self.num_rows(),
            0..=(self.num_columns() - self.winning_length)
        )
    }

    fn diagonal_up_right_sequence_coordinates(
        &self,
        row: usize,
        column: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let first_row = row - self.winning_length;
        let last_column = column + self.winning_length;
        (first_row..row).rev().zip(column..last_column)
    }

    fn cell_to_char(cell: Cell) -> char {
        if let Some(team) = cell {
            if team.0 > MAX_PRINTABLE_TEAMS {
                panic!("Cannot convert team {} to a char", team);
            }
        }
        match cell {
            Some(team) => char::from_digit(team.0 as u32, 16).unwrap(),
            None => DEFAULT_EMPTY_CHAR,
        }
    }

    fn next_turn(&self) -> Team {
        let next_team_num = (self.cur_turn.0 + 1) % self.num_teams;
        Team(next_team_num)
    }

    fn drop_chip_cells(&mut self, column: usize) -> Result<(), Error> {
        if column >= self.cells.num_columns() {
            return Err(Error::OutOfBounds);
        }
        let row = self.highest_unfilled_row(column)?;
        self.cells[(row, column)] = Some(self.cur_turn);
        Ok(())
    }

    fn highest_unfilled_row(&self, column: usize) -> Result<usize, Error> {
        self.cells
            .column_iter(column)
            .enumerate()
            .find(|(_, cell)| cell.is_none())
            .map(|(index, _)| index)
            .ok_or(Error::ColumnFull)
    }

    fn create_empty_grid_rows(num_rows: usize, num_columns: usize) -> Vec<Vec<Cell>> {
        (0..num_rows)
            .map(|_| (0..num_columns).map(|_| None).collect())
            .collect()
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Team {}", self.0)
    }
}
