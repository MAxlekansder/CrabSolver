mod sudoku_grid;

use sudoku_grid::SudokuGrid;
use serde::{Deserialize, Serialize};  

#[derive(Debug, Deserialize, Serialize)]
pub struct SudokuGridDto {
    pub sudokuGrid: Vec<Vec<u8>>,
}

#[tokio::main]
async fn main() {
    let url = "http://localhost:8080/krabba/sudoku";

    match SudokuGrid::read_sudoku_from_api(url).await {
        Ok(dto) => {

            let mut grid = SudokuGrid::new();

            // Iterate over the sudokuGrid in SudokuGridDto
            for (row_idx, row_data) in dto.sudokuGrid.iter().enumerate() {
                for (col_idx, &value) in row_data.iter().enumerate() {
                    grid.set_cell(row_idx, col_idx, value);
                }
            }

            // Display the populated SudokuGrid
            grid.display();

            if grid.solve_sudoku() {
                println!("================== solved ==================");
                grid.display()
            }
        }
        Err(e) => println!("Error fetching Sudoku grid: {:?}", e),
    }
}
