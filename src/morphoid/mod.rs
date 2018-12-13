use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Nothing,
    Cell(i64),
}

pub struct World {
    width: u32,
    height: u32,
    cells: Vec<Entity>,
}

impl World {
    pub fn new(width:u32, height:u32) -> World {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Entity::Cell(1)
                } else {
                    Entity::Nothing
                }
            })
            .collect();

        World {width, height, cells}
    }

    // TODO: synchronize
    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                new_cells[idx] = match cell {
                    Entity::Cell(gene_id) => Entity::Cell(gene_id + 1),
                    otherwise => otherwise,
                };
            }
        }

        self.cells = new_cells;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}


impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Entity::Nothing { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
