use std::{thread::sleep, time::Duration, fmt};

const WIDTH: usize = 30;
const HEIGHT: usize = 15;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Dead => write!(f, "."),
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
    fn tick(&self) -> Canvas {
        let mut new_rows = [[Cell::Dead; WIDTH]; HEIGHT];
        
        for r in 0..self.rows.len() {
            for c in 0..self.rows[r].len() {
                let mut count = 0;

                let min_r: usize = if r == 0 { 0 } else { r-1 };
                let max_r: usize = if r+1 == HEIGHT { r } else { r+1 };
                for x in min_r..=max_r {

                    let min_c: usize = if c == 0 { 0 } else { c-1 };
                    let max_c: usize = if c+1 == WIDTH { c } else { c+1 };
                    // println!("{}, {}, {}, {}", min_c, max_c, min_r, max_r);
                    for y in min_c..=max_c {
                        if c == y && r == x { continue; }

                        if self.rows[x][y] == Cell::Alive {
                            count += 1;
                        };
                    };
                };

                new_rows[r][c] = match (self.rows[r][c], count) {
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (_, _) => Cell::Dead,
                };
            };
        };

        Canvas { rows: new_rows }
    }
}

fn main() {
    let mut rows = [[Cell::Dead; WIDTH]; HEIGHT];

    for (i, j) in [(3, 4), (4, 5), (5, 3), (5, 4), (5, 5)] {
        rows[i][j] = Cell::Alive
    }

    let mut canvas = Canvas { rows };

    loop {
        for _ in 0..30 { println!("\n"); }
        println!("{}", canvas);

        canvas = canvas.tick();
        sleep(Duration::new(0, 400_000_000));
    }
}
