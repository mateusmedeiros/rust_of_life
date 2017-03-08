use std::sync::{ RwLock, RwLockReadGuard, Arc };
use std::sync::mpsc::{ Receiver };

use models::Grid;
use grid_displayers::GridDisplayer;

#[allow(dead_code)]
pub struct SimpleTerminal;

impl SimpleTerminal {
    #[allow(dead_code)]
    pub fn new() -> SimpleTerminal {
        SimpleTerminal
    }
}

impl GridDisplayer for SimpleTerminal {
    fn draw(&self, receiver: Receiver<Arc<RwLock<Grid>>>) {
        loop {
            let locked_grid: Arc<RwLock<Grid>> = receiver.recv().unwrap();
            let lock_guard: RwLockReadGuard<Grid> = locked_grid.read().unwrap();
            let ref grid: Grid = *lock_guard;

            for row in grid {
                for cell in row {
                    match cell.is_alive() {
                        true => print!("o"),
                        false => print!("_")
                    };
                }
                print!("\n");
            }
            print!("\n\n");
        }
    }
}
