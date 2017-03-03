use std::ops::{Index, IndexMut};
use std::iter::{ IntoIterator };
use std::slice::Iter;
use models::Cell;

#[derive(Clone)]
pub struct GridRow {
    row: Vec<Cell> 
}

impl GridRow {
    pub fn new(size: usize) -> GridRow {
        GridRow { row: vec![Cell::default(); size] }
    }
}

impl<'a> IntoIterator for &'a GridRow {
    type Item = &'a Cell;
    type IntoIter = Iter<'a, Cell>;

    fn into_iter(self) -> Iter<'a, Cell> {
        self.row.iter()
    }
}


impl Index<usize> for GridRow {
    type Output = Cell;

    fn index(&self, index: usize) -> &Cell {
        &(self.row[index])
    }
}

impl IndexMut<usize> for GridRow {
    fn index_mut(&mut self, index: usize) -> &mut Cell {
        &mut (self.row[index])
    }
}
