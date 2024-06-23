mod sudoku_logic;

use sudoku_logic::SudokuGrid;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SudokuGridDto {
    pub sudokuGrid: Vec<Vec<u8>>,
}

#[tokio::main]
async fn main() {
    let url = "http://localhost:8080/krabba/sudoku";

    match SudokuGrid::read_sudoku_from_api(url).await {
        Ok(dto) => {

            let mut grid = SudokuGrid::from_dto(dto);
            grid.display();

            if grid.solve_sudoku() {
                println!("================== solved ==================");
                grid.display()
            }
        }
        Err(e) => println!("Error fetching Sudoku grid: {:?}", e),
    }
}
