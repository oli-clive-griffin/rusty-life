use std::{vec, thread::sleep, time::Duration, slice::Windows, collections::btree_map::Iter};

const WIDTH: usize = 30;

struct Canvas {
    cells: Vec<Cell>,
    width: usize,
}

impl Canvas {
    fn tick(&self) -> Canvas {
        let mut new_cell_vals = vec![false; &self.width * self.width];
        
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

                let num_alive_neighbours = neighbours.iter().filter(|&&&c| c.val).count();

                let new_val = match (cell.val, num_alive_neighbours) {
                    (true, 2 | 3) => true,
                    (false, 3) => true,
                    (_, _) => false,
                };

                new_cell_vals[(i * self.width) + j] = new_val;
            }

        }

        let new_cells: Vec<Cell> = new_cell_vals.iter().map(|&val| Cell { val }).collect();

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

#[derive(Clone, Copy, Debug)]
struct Cell {
    val: bool
}

impl Cell {
    fn to_string(&self) -> &str {
        match self.val {
            true => " • ",
            false => "   ",
        }
    }
}

fn main() {
    let mut cells = [false; WIDTH * WIDTH].map(|val| Cell { val }).to_vec();

    for (i, j) in [(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)] {
        cells[i * WIDTH  + j].val = true;
    }

    let mut canvas = Canvas { cells, width: WIDTH };

    loop {
        sleep(Duration::new(0, 80_000_000));
        println!("{}", canvas.to_string());

        let asdf = (0..WIDTH).map(|_| "\n".to_string()).collect::<String>();
        println!("{}", asdf);

        canvas = canvas.tick();
    }
}

