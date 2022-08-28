use std::{thread::sleep, time::Duration, fmt::{Display, Formatter, self}};

const WIDTH: usize = 30;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

struct Canvas {
    cells: [Cell; WIDTH * WIDTH],
    width: usize,
}

trait OrZero {
    fn or_zero(self, not: usize) -> usize;
}

impl OrZero for usize {
    fn or_zero(self, not: usize) -> usize {
        if self == not { 0 } else { self }
    }
}

impl Canvas {
    fn tick(&self) -> Canvas {
        let mut new_cells = [Cell::Dead; WIDTH * WIDTH];
        
        for (i, cell) in self.cells.iter().enumerate() {
            let nw = ((i+ (WIDTH * WIDTH) - WIDTH - 1) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let n = ((i+ (WIDTH * WIDTH) - WIDTH) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let ne = ((i+ (WIDTH * WIDTH) - WIDTH + 1) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);

            let left = ((i+ (WIDTH * WIDTH) - 1) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let right = ((i+ (WIDTH * WIDTH) + 1) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);

            let sw = ((i+ (WIDTH * WIDTH) + WIDTH - 1) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let s = ((i+ (WIDTH * WIDTH) + WIDTH) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let se = ((i+ (WIDTH * WIDTH) + WIDTH + 1) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);

            let neighbours = [
                &self.cells[nw],
                &self.cells[n],
                &self.cells[ne],

                &self.cells[left],
                &self.cells[right],

                &self.cells[sw],
                &self.cells[s],
                &self.cells[se],
            ];

            let new_val = {
                let num_alive_neighbours = neighbours.iter()
                    .filter(|&&&c| c == Cell::Alive)
                    .count();

                match (cell, num_alive_neighbours) {
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (_, _) => Cell::Dead,
                }
            };

            new_cells[i] = new_val;
        }

        Canvas { cells: new_cells, width: self.width }
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cells
            .map(|c| c.to_string())
            .join("  "))
    }

}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Dead => write!(f, " "),
            Cell::Alive => write!(f, "•"),
        }
    }
}

fn main() {
    let mut cells = [Cell::Dead; WIDTH * WIDTH];

    for (i, j) in [(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)] {
        cells[i * WIDTH  + j] = Cell::Alive
    }

    let mut canvas = Canvas { cells, width: WIDTH };

    loop {
        sleep(Duration::new(0, 80_000_000));
        println!("{}", canvas);

        let asdf = (0..WIDTH).map(|_| "\n".to_string()).collect::<String>();
        println!("{}", asdf);

        canvas = canvas.tick();
    }
}