use std::{vec, thread::sleep, time::Duration};
use modular::*;

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
            let nw = (((i+ (WIDTH * WIDTH) - WIDTH - 1) ) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let n = (((i+ (WIDTH * WIDTH) - WIDTH) ) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let ne = (((i+ (WIDTH * WIDTH) - WIDTH + 1) ) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);

            let left = (((i+ (WIDTH * WIDTH) - 1) ) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let right = (((i+ (WIDTH * WIDTH) + 1) ) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);

            let sw = (((i+ (WIDTH * WIDTH) + WIDTH - 1) ) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let s = (((i+ (WIDTH * WIDTH) + WIDTH) ) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);
            let se = (((i+ (WIDTH * WIDTH) + WIDTH + 1) ) % (WIDTH * WIDTH)).or_zero(WIDTH * WIDTH);

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

    fn to_string(&self) -> String {
        let mut str = "".to_string();
        for row in self.cells.chunks(self.width) {
            for cell in row {
                str += cell.to_string();
            }
            str += "\n";
        }
        str
    }

}

impl Cell {
    fn from_bool(b: bool) -> Cell {
        match b {
            true => Cell::Alive,
            false => Cell::Dead,
        }
    }


    fn to_string(&self) -> &str {
        match self {
            Cell::Dead => "   ",
            Cell::Alive => " • ",
        }
    }
}

fn main() {
    // let mut cells = (0..WIDTH * WIDTH).map(|val| Cell::from_bool(false)).collect();
    let mut cells = [Cell::Dead; WIDTH * WIDTH];

    for (i, j) in [(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)] {
        cells[i * WIDTH  + j] = Cell::Alive
    }

    let mut canvas = Canvas { cells: cells, width: WIDTH };

    loop {
        sleep(Duration::new(0, 80_000_000));
        println!("{}", canvas.to_string());

        let asdf = (0..WIDTH).map(|_| "\n".to_string()).collect::<String>();
        println!("{}", asdf);

        canvas = canvas.tick();
    }
}