impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];

        for y in 0..9 {
            for x in 0..9 {
                if board[y][x] == '.' {
                    continue;
                }

                let num = (board[y][x] as u8 - b'1') as usize;

                let box_idx: usize = ((y / 3) * 3 + (x / 3)) as usize;
                if boxes[box_idx][num] || rows[y][num] || cols[x][num] {
                    return false
                }

                boxes[box_idx][num] = true;
                rows[y][num] = true;
                cols[x][num] = true;
            }
        }

        true
    }
}
