use std::default::Default;
use std::sync::{ RwLock, RwLockReadGuard, Arc };
use std::sync::mpsc::{ Receiver };
use pancurses::{
    COLOR_BLACK,
    COLOR_PAIR,
    endwin,
    init_color,
    init_pair,
    initscr,
    Input,
    noecho,
    set_title,
    start_color
};

use models::Grid;
use grid_displayers::GridDisplayer;
use super::Character;

pub struct Pancurses {
    true_color: bool
}

impl Default for Pancurses {
    fn default() -> Pancurses {
        Pancurses { true_color: true } 
    }
}

impl Pancurses {
    pub fn new() -> Pancurses {
        Pancurses::default()
    }

    pub fn true_color(mut self, value: bool) -> Pancurses {
        self.true_color = value;
        self
    }
}

impl GridDisplayer for Pancurses {
    fn draw(&self, receiver: Receiver<Arc<RwLock<Grid>>>) {
        let main_window = initscr();
        let magic: f32 = 1000.0 / 255.0;
        noecho();
        start_color();
        set_title("Rust of Life");
        init_color(8, (105.0 * magic) as i16, (210.0 * magic) as i16, (231.0 * magic) as i16);
        init_color(9, (213.0 * magic) as i16, (213.0 * magic) as i16, (213.0 * magic) as i16);
        init_pair(2, 8, 9);
        init_pair(1, COLOR_BLACK, 8);
        
        let bg = COLOR_PAIR(2);
        let fg = COLOR_PAIR(1);

        let living_cell = &Character { character: Some('o'), color: Some(fg), attributes: None };
        let dead_cell = &Character { character: Some('-'), color: Some(bg), attributes: None };

        main_window.nodelay(true);

        loop {
            let locked_grid: Arc<RwLock<Grid>> = receiver.recv().unwrap();
            let lock_guard: RwLockReadGuard<Grid> = locked_grid.read().unwrap();
            let ref grid: Grid = *lock_guard;

            main_window.mv(0, 0);
            for (row_number, row) in grid.into_iter().enumerate() {
                for cell in row {
                    match cell.is_alive() {
                        true => main_window.addch(living_cell),
                        false => main_window.addch(dead_cell)
                    };
                }

                main_window.mv(row_number as i32, 0);
            }

            main_window.refresh();

            match main_window.getch() {
                Some(Input::Character('q')) => break,
                Some(Input::Character('x')) => break,
                _ => continue
            }
        }

        endwin();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;
    use super::super::Character;
    use pancurses::{ initscr, endwin, newwin };
    use pancurses::COLOR_WHITE;
    use pancurses;
    use pancurses::chtype;

    #[test]
    fn test_pancurses() {
        let main_window = initscr();
        let magic = 1000.0 / 255.0;
        pancurses::start_color();
        //pancurses::init_color(8, 1000, 0, 0);
        //pancurses::init_color(9, 0, 1000, 0);
        pancurses::init_color(8, (105.0 * magic) as i16, (210.0 * magic) as i16, (231.0 * magic) as i16);
        pancurses::init_color(9, (213.0 * magic) as i16, (213.0 * magic) as i16, (213.0 * magic) as i16);
        pancurses::init_pair(2, 8, 9);
        pancurses::init_pair(1, 9, 8);
        
        let bg = pancurses::COLOR_PAIR(2);
        let fg = pancurses::COLOR_PAIR(1);

        let teste_character = &Character { character: Some('サ'), color: Some(fg), attributes: None };
        let teste_character2 = &Character { character: Some('@'), color: Some(bg), attributes: None };
        main_window.addch(teste_character);
        // main_window.addch(teste_character);
        // main_window.addch(teste_character2);
        // main_window.addch(teste_character);

        main_window.refresh();
        std::thread::sleep_ms(3000);
        endwin();
    }
}
