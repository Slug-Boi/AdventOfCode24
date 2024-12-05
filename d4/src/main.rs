use std::io;
use std::collections::HashSet;

fn main() {
    let input: Result<Vec<String>, io::Error> = io::stdin().lines().collect();
    let input = input.unwrap();

    // let result = line_search(&input);

    // println!("{}", result);

    let result2 = line_search2(input);

    println!("{}", result2);
}

fn line_search(input: Vec<String>) -> i32 {
    let mut result = 0;
    let rows = input.len();
    let cols = input[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if input[r].as_bytes()[c] == b'X' {
                // Check horizontal
                if c >= 3 && input[r].as_bytes()[c-1] == b'M' && input[r].as_bytes()[c-2] == b'A' && input[r].as_bytes()[c-3] == b'S' {
                    result += 1;
                }
                if c + 3 < cols && input[r].as_bytes()[c+1] == b'M' && input[r].as_bytes()[c+2] == b'A' && input[r].as_bytes()[c+3] == b'S' {
                    result += 1;
                }
                // Check vertical
                if r >= 3 && input[r-1].as_bytes()[c] == b'M' && input[r-2].as_bytes()[c] == b'A' && input[r-3].as_bytes()[c] == b'S' {
                    result += 1;
                }
                if r + 3 < rows && input[r+1].as_bytes()[c] == b'M' && input[r+2].as_bytes()[c] == b'A' && input[r+3].as_bytes()[c] == b'S' {
                    result += 1;
                }
                // Check diagonal (top-left to bottom-right)
                if r >= 3 && c >= 3 && input[r-1].as_bytes()[c-1] == b'M' && input[r-2].as_bytes()[c-2] == b'A' && input[r-3].as_bytes()[c-3] == b'S' {
                    result += 1;
                }
                if r + 3 < rows && c + 3 < cols && input[r+1].as_bytes()[c+1] == b'M' && input[r+2].as_bytes()[c+2] == b'A' && input[r+3].as_bytes()[c+3] == b'S' {
                    result += 1;
                }
                // Check diagonal (top-right to bottom-left)
                if r >= 3 && c + 3 < cols && input[r-1].as_bytes()[c+1] == b'M' && input[r-2].as_bytes()[c+2] == b'A' && input[r-3].as_bytes()[c+3] == b'S' {
                    result += 1;
                }
                if r + 3 < rows && c >= 3 && input[r+1].as_bytes()[c-1] == b'M' && input[r+2].as_bytes()[c-2] == b'A' && input[r+3].as_bytes()[c-3] == b'S' {
                    result += 1;
                }
            }
        }
    }
    result
}

fn match_char(cell: Option<u8>, expected: u8) -> bool {
    match cell {
        Some(c) => c == expected,
        None => false,
    }
}

fn get_cell(grid: &[String], x: usize, y: usize) -> Option<u8> {
    if x < grid.len() && y < grid[0].len() {
        Some(grid[x].as_bytes()[y])
    } else {
        None
    }
}

fn is_valid_mas(grid: &[String], x1: usize, y1: usize, x2: usize, y2: usize, x3: usize, y3: usize) -> bool {
    (match_char(get_cell(grid, x1, y1), b'M') &&
     match_char(get_cell(grid, x2, y2), b'A') &&
     match_char(get_cell(grid, x3, y3), b'S')) ||
    (match_char(get_cell(grid, x1, y1), b'S') &&
     match_char(get_cell(grid, x2, y2), b'A') &&
     match_char(get_cell(grid, x3, y3), b'M'))
}

fn get_mas(grid: &[String]) -> Vec<bool> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = Vec::new();

    for x in 0..rows - 2 {
        for y in 0..cols - 2 {
            result.push(
                is_valid_mas(grid, x, y, x + 1, y + 1, x + 2, y + 2) &&
                is_valid_mas(grid, x, y + 2, x + 1, y + 1, x + 2, y)
            );
        }
    }

    result
}

fn line_search2(input: Vec<String>) -> i32 {
    let mas_patterns = get_mas(&input);
    mas_patterns.iter().filter(|&&x| x).count() as i32
}