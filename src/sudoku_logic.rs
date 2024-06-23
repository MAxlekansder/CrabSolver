use reqwest::Error;
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SudokuGridDto {
    pub sudokuGrid: Vec<Vec<u8>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SudokuGrid {
     cells: [[u8; 9]; 9] // unsigned integer
}

impl SudokuGrid {
    pub fn new() -> Self { SudokuGrid { cells: [[0; 9]; 9] } }

    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }


    pub fn set_cell(&mut self, row: usize, col: usize, value: u8) {
        self.cells[row][col] = value
    }

    pub fn display(&self) {
        for row in &self.cells {
            for &cell in row {
                let cell_str = if cell == 0 {
                    ".".to_string()
                } else {
                    cell.to_string()
                };
                print!("{} ", cell_str);
            }
            println!();
        }
    }




    pub fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.get_cell(row, col) == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    pub async fn read_sudoku_from_api(url: &str) -> Result<SudokuGridDto, Error> {
        let response = reqwest::get(url).await?.json::<SudokuGridDto>().await?;
        Ok(response)
    }

    pub fn used_in_col(&self, col: usize, num: u8) -> bool {
        for row in 0..9 {
            if self.get_cell(row, col) == num {
                return true;
            }
        }
        false
    }

    pub fn used_in_row(&self, row: usize, num: u8) -> bool {
        for col in 0..9 {
            if self.get_cell(row, col) == num {
                return true;
            }
        }
        false
    }

    pub fn used_in_subgrid(&self, start_row: usize, start_col: usize, num: u8) -> bool {
        // Iterate through the 3x3 subgrid
        for row in 0..3 {
            for col in 0..3 {
                // Calculate actual grid coordinates based on start_row and start_col
                let actual_row = start_row + row;
                let actual_col = start_col + col;

                // Check if the number exists in the subgrid
                if self.get_cell(actual_row, actual_col) == num {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_valid_move(&self, row: usize, col: usize, num: u8) -> bool {
        !self.used_in_row(row, num)
            && !self.used_in_col(col, num)
            && !self.used_in_subgrid(row - row % 3, col - col % 3, num)
    }

    pub fn from_dto(dto: SudokuGridDto) -> Self {
        let mut grid = SudokuGrid::new();
        for (row_idx, row_data) in dto.sudokuGrid.iter().enumerate() {
            for (col_idx, &value) in row_data.iter().enumerate() {
                grid.set_cell(row_idx, col_idx, value);
            }
        }
        grid
    }

    pub fn solve_sudoku(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty_cell() {
            for num in 1..=9 {
                if self.is_valid_move(row, col, num as u8) {
                    self.set_cell(row, col, num as u8);
                    if self.solve_sudoku() {
                        return true;
                    }
                    self.set_cell(row, col, 0);
                }
            }
            false
        } else {
            true
        }
    }
}