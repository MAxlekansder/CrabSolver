mod sudoku_grid;

use sudoku_grid::SudokuGrid;

#[tokio::main]
async fn main() {
    let url = "http://localhost:8080/krabba/sudoku";

    match SudokuGrid::read_sudoku_from_api(url).await {
        Ok(mut grid) => {
            println!("Sudoku grid fetched from API:");
            grid.display();
            if grid.solve_sudoku() {
                println!("Sudoku solved:");
                grid.display();
            } else {
                println!("No solution found for the Sudoku grid.");
            }
        }
        Err(e) => {
            eprintln!("Error fetching Sudoku grid: {}", e);
        }
    }
}
