
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {

        fn is_valid(board: &mut Vec<Vec<char>>, i:usize, j:usize, c: char) -> bool {
            for idx in 0..9 {
                if board[idx][j] == c || board[i][idx] == c || 
                    board[3 * (i/3) + idx / 3][3 * (j/3) + (idx % 3)] == c
                {
                    return false;
                }
            }
            true
        }

        fn solve_index(board: &mut Vec<Vec<char>>, idx: usize) -> bool {
            if idx == 81 {
                return true;
            }
            let i = idx / 9;
            let j = idx % 9;

            if board[i][j] != '.' {
                return solve_index(board, idx + 1);
            }
            
            for c in "123456789".chars() {
                if is_valid(board, i, j, c) {
                    board[i][j] = c;
                    if solve_index(board, idx + 1) {
                        return true;
                    }
                    board[i][j] = '.';
                }
            }
            false
        }

        solve_index(board, 0usize);
    }
}

/* */

// LEARN

struct SudokuSolver {
    row: Vec<Vec<bool>>,
    column: Vec<Vec<bool>>,
    grid: Vec<Vec<bool>>,
    pub be_fill: Vec<(usize, usize, i32)>,
}

impl SudokuSolver {
    fn new(board: &mut Vec<Vec<char>>) -> Self {
        let mut row = vec![vec![true; 9]; 9];
        let mut column = vec![vec![true; 9]; 9];
        let mut grid = vec![vec![true; 9]; 9];
        let mut be_fill = Vec::new();
        board.iter().enumerate().for_each(|(x, line)|
            line.iter().enumerate().for_each(|(y, num)|
                if let Some(n) = num.to_digit(10) {
                    let n = n as usize - 1;
                    row[x][n] = false;
                    column[y][n] = false;
                    grid[x / 3 * 3 + y / 3][n] = false;
                } else {
                    be_fill.push((x, y, 0));
                }
            )
        );
        SudokuSolver {
            row, column, grid, be_fill
        }
    }

    fn solve(&mut self, i: usize) -> bool {
        if i == self.be_fill.len() {
            return true;
        }
        let (x, y, _) = self.be_fill[i];
        for n in 0..9 {
            if self.row[x][n] && self.column[y][n] && self.grid[x / 3 * 3 + y / 3][n] {
                self.row[x][n] = false;
                self.column[y][n] = false;
                self.grid[x / 3 * 3 + y / 3][n] = false;
                self.be_fill[i].2 = n as i32;
                if self.solve(i + 1) {
                    return true;
                }
                self.row[x][n] = true;
                self.column[y][n] = true;
                self.grid[x / 3 * 3 + y / 3][n] = true;
            }
        }
        false
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut solver = SudokuSolver::new(board);
        solver.solve(0 as usize);
        solver.be_fill.iter().for_each(|(x, y, n)|
            board[*x][*y] = (*n as u8 + b'1') as char
        );
    }
}

/* */

// LEARN

use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

#[derive(Debug, Clone)]
struct EmptySudokuCell{
    position: u16,
    posibilities: HashSet<char>
}

impl EmptySudokuCell {
    fn new(pos: u16) -> Self{
        EmptySudokuCell { position: pos, posibilities: HashSet::from(['1','2','3','4','5','6','7','8','9']) }
    }
}

impl Eq for EmptySudokuCell {}

impl PartialEq for EmptySudokuCell {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.posibilities.len() == other.posibilities.len()
    }
}

impl PartialOrd for EmptySudokuCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EmptySudokuCell{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let len_cmp = self.posibilities.len().cmp(&other.posibilities.len());
        match len_cmp {
            std::cmp::Ordering::Equal => {},
            res => return res,
        }
        self.position.cmp(&other.position)
    }
}

#[derive(Debug, Clone)]
struct SudokuSolver([[char; 9]; 9], BinaryHeap<Reverse<EmptySudokuCell>>);

impl SudokuSolver {
    fn new(board: Vec<Vec<char>>) -> Self {
        let mut filled_cells: Vec<(u16, char)> = vec![];
        let mut empty_cells: Vec<EmptySudokuCell> = vec![];

        for (y, row) in board.iter().enumerate(){
            let y = y as u16;
            for (x, cell) in row.iter().enumerate(){
                let x = x as u16;
                match cell {
                    '.' => empty_cells.push(EmptySudokuCell::new(x+(y*9))),
                    v @ '1'..='9' => filled_cells.push((x+(y*9), *v)),
                    _ => unreachable!()
                }
            }
        }

        for (pos, v) in filled_cells{
            let x = pos % 9;
            let y = pos / 9;
            let iter = empty_cells.iter_mut().filter(|e| {
                let e_x = e.position % 9;
                let e_y = e.position / 9;

                let same_row = e_y == y;
                let same_col = e_x == x;
                let same_box = (e_x / 3) == (x / 3) && (e_y / 3) == (y / 3);
                same_row || same_col || same_box
            });

            for empty in iter{
                empty.posibilities.remove(&v);
            }
        }

        SudokuSolver(*TryInto::<Box<[[char; 9]; 9]>>::try_into(board.iter().map(|v| *TryInto::<Box<[char; 9]>>::try_into(v.clone().into_boxed_slice()).unwrap()).collect::<Vec<_>>().into_boxed_slice()).unwrap(), empty_cells.into_iter().map(|v| Reverse(v)).collect())
    }

    fn is_solved(&self) -> bool{
        self.1.len() == 0
    }    

    fn step(mut self) -> Vec<Self>{
        let Some(Reverse(cell_data)) = self.1.pop() else {return vec![];};
        
        let x = cell_data.position % 9;
        let y = cell_data.position / 9;
        let mut boards = vec![];

        for variant in cell_data.posibilities{
            let new_tree = self.1.clone().into_iter().map(|Reverse(mut val)| {
                let e_x = val.position % 9;
                let e_y = val.position / 9;
                let same_row = e_y == y;
                let same_col = e_x == x;
                let same_box = (e_x / 3) == (x / 3) && (e_y / 3) == (y / 3);
                
                if same_row || same_col || same_box{
                    val.posibilities.remove(&variant);
                }
                Reverse(val)
            }).collect::<BinaryHeap<Reverse<EmptySudokuCell>>>();

            let mut arrays = self.0.clone();
            arrays[y as usize][x as usize] = variant;

            boards.push(SudokuSolver(arrays, new_tree))
        }
        boards
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut solvers = vec![SudokuSolver::new(board.clone())];

        while let Some(b) = solvers.pop() {
            if b.is_solved() {
                for (target_row, row) in board.iter_mut().zip(b.0.iter()){
                    for (target_cell, cell) in target_row.iter_mut().zip(row.iter()){
                        *target_cell = *cell
                    }
                }
                return;
            }
            solvers.append(&mut b.step())
        }
    }
}
