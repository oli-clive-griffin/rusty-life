use std::{thread::sleep, time::Duration, fmt};

const WIDTH: usize = 50;
const HEIGHT: usize = 30;
const MILLIS: u32 = 100;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Dead => write!(f, " "),
            Cell::Alive => write!(f, "•"),
        }
    }
}

struct Canvas {
    rows: [[Cell; WIDTH]; HEIGHT],
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = self.rows
            .map(|row| row.map(|cell| format!("{}", cell)).join("  "))
            .join("\n");

        write!(f, "{}", str)
    }
}

impl Canvas {
    fn tick(&mut self) -> () {
        let mut new_rows = [[Cell::Dead; WIDTH]; HEIGHT];
        
        for r in 0..self.rows.len() {
            for c in 0..self.rows[r].len() {

                let count = self.get_surrounding_count(r, c);

                new_rows[r][c] = match (self.rows[r][c], count) {
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (_, _) => Cell::Dead,
                };
            };
        };

        self.rows = new_rows;
    }

    fn get_surrounding_count(&mut self, r: usize, c: usize) -> i32 {
        let mut count = 0;
        let row_up = if r == 0 { self.rows.len() - 1 } else { r-1 };
        let row_down = if r+1 == HEIGHT { 0 } else { r+1 };
        let col_left = if c == 0 { self.rows[0].len() - 1 } else { c-1 };
        let col_right = if c+1 == WIDTH { 0 } else { c+1 };
        let checks = [
            (row_up, col_left), (row_up, c), (row_up, col_right),
            (r, col_left), (r, col_right),
            (row_down, col_left), (row_down, c), (row_down, col_right),
        ];

        for (x, y) in checks {
            if c == y && r == x {
                continue;
            }

            if self.rows[x][y] == Cell::Alive {
                count += 1;
            };
        };

        return count;
    }
}

fn main() {
    let mut rows = [[Cell::Dead; WIDTH]; HEIGHT];

    for (i, j) in [(5, 4), (6, 5), (7, 3), (7, 4), (7, 5)] {
        rows[i][j] = Cell::Alive
    }

    let mut canvas = Canvas { rows };

    loop {
        for _ in 0..30 { println!("\n"); }
        println!("{}", canvas);

        canvas.tick();

        sleep(Duration::new(0, MILLIS * 1_000_000));
    }
}
