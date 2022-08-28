use std::{vec, thread::sleep, time::Duration};

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

impl Canvas {
    fn tick(&self) -> Canvas {
        let mut new_cells = [Cell::Dead; WIDTH * WIDTH];
        
        for (i, row) in self.cells.chunks(self.width).enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let up = if i == 0 { None } else { Some(i - 1) };
                let down = if i == self.width - 1 { None } else  {Some(i + 1)};

                let left = if j == 0 { None } else { Some(j - 1) };
                let right = if j == self.width - 1 { None } else  {Some(j + 1)};

                let neighbours = match (up, down, left, right) {
                    (Some(up), Some(down), Some(left), Some(right)) => vec![
                        &self.cells[(up * self.width) + left],
                        &self.cells[(up * self.width) + j],
                        &self.cells[(up * self.width) + right],

                        &self.cells[(i * self.width) + left],
                        &self.cells[(i * self.width) + right],

                        &self.cells[(down * self.width) + left],
                        &self.cells[(down * self.width) + j],
                        &self.cells[(down * self.width) + right],
                    ],
                    _ => vec![],
                };

                if neighbours.len() < 8 { continue; }

                let num_alive_neighbours = neighbours.iter().filter(|&&&c| c == Cell::Alive).count();

                let new_val = match (cell, num_alive_neighbours) {
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (_, _) => Cell::Dead,
                };

                new_cells[(i * self.width) + j] = new_val;
            }

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

