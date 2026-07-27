const VISITED_CODE: i32 = -101;

enum Direction {
    RIGHT, // Default
    DOWN,
    LEFT,
    UP
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::RIGHT => (0, 1),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::UP => (-1, 0)
        }
    }
}

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![]
        }


        let mut y = 0;
        let mut x = 0;

        let m = matrix.len() as isize;
        let n = matrix[0].len() as isize;

        let mut res = Vec::with_capacity((m * n) as usize);

        let mut dir = Direction::RIGHT;

        for _ in 0..(m * n) {
            res.push(matrix[y as usize][x as usize]);
            matrix[y as usize][x as usize] = VISITED_CODE;

            let (dy, dx) = dir.delta();

            let mut next_y = y + dy;
            let mut next_x = x + dx;

            // collision checking
            if  next_x < 0 || next_x >= n || next_y < 0 || next_y >= m || matrix[next_y as usize][next_x as usize] == VISITED_CODE {
                dir = dir.next();
                let (ddy, ddx) = dir.delta();

                next_y = y + ddy;
                next_x = x + ddx;
            }

            y = next_y;
            x = next_x;
        }

        res
    }
}
