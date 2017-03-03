use std::ops::{ Index, IndexMut };
use std::iter::{ IntoIterator };
use std::slice::Iter;
use std::vec::Vec;

use models::Cell;
use models::grid_row::GridRow;

#[derive(Clone)]
pub struct Grid {
    internal_grid: Vec<GridRow>,
    cols: usize,
    rows: usize
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Grid {
        Grid {
            internal_grid: vec![GridRow::new(x); y],
            cols: x,
            rows: y
        }
    }

    fn get_neighbors(&self, x: isize, y: isize) -> Vec<&Cell> {
        let neighbor_coordinates = [
            (x + 1, y - 1),
            (x + 1, y    ),
            (x + 1, y + 1),
            (x - 1, y - 1),
            (x - 1, y    ),
            (x - 1, y + 1),
            (x    , y - 1),
            (x    , y + 1)
        ];

        neighbor_coordinates.iter()
            .filter(|&&(x, y)| {
                (x >= 0 && x < self.cols as isize) && 
                (y >= 0 && y < self.rows as isize)
            })
            .map(|&(x, y)| &(self[y as usize][x as usize]))
            .collect()
    }

    pub fn iterate(&mut self) {
        let old_grid = self.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let living_neighbors = old_grid
                    .get_neighbors(col as isize, row as isize).iter()
                    .filter(|cell| cell.is_alive())
                    .count();
                
                match (self[row][col].is_alive(), living_neighbors) {
                    (true,  0...1) => self[row][col].die(),
                    (true,  2...3) => self[row][col].arise(),
                    (true,  _)     => self[row][col].die(),
                    (false, 3)     => self[row][col].arise(),
                    (false,  _)    => self[row][col].die()
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a GridRow;
    type IntoIter = Iter<'a, GridRow>;

    fn into_iter(self) -> Iter<'a, GridRow> {
        self.internal_grid.iter()
    }
}

impl Index<usize> for Grid {
    type Output = GridRow;

    fn index(&self, index: usize) -> &GridRow {
        &(self.internal_grid[index])
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut GridRow {
        &mut (self.internal_grid[index])
    }
}
