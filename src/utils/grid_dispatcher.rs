use std::time;
use std::thread;
use std::sync::{ RwLock, Arc };
use std::sync::mpsc::{ channel, Receiver };

use models::Grid;

pub fn create_dispatcher(grid: Grid) -> Receiver<Arc<RwLock<Grid>>> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let locked_grid = Arc::new(RwLock::new(grid));
        loop {
            tx.send(locked_grid.clone()).unwrap();
            thread::sleep(time::Duration::from_millis(300));

            let mut writable_grid = locked_grid.write().unwrap();
            writable_grid.iterate();
        }     
    });

    rx
}
