use std::default::Default;
use std::sync::{ RwLock, RwLockReadGuard, Arc };
use std::sync::mpsc::{ Receiver };
use pancurses::{
    COLOR_BLACK,
    COLOR_BLUE,
    COLOR_MAGENTA,
    COLOR_PAIR,
    Input,
    Window,
    chtype,
    color_content,
    endwin,
    init_color as curses_init_color,
    init_pair,
    initscr,
    noecho,
    set_title,
    start_color
};

use models::Grid;
use grid_displayers::GridDisplayer;
use super::Color;

pub struct Pancurses {
    main_window: Window,
    cell_character: char,
    cell_color: i16,
    cell_background_color: i16,
    background_color: i16,
    next_color_index: i16
}

/// Curses accept RGB ranges from 0 to 1000.
/// 
/// We use this magic number to be able to use the far more common 0 to 255
/// range.
///
/// We can get the equivalent 0 to 1000 value by multiplying the 0 to 255 value
/// by this magic number.
const COLOR_MAGIC: f32 = 1000.0 / 255.0;

/// The pair number used for the background color
const DEAD_CELL_PAIR: chtype = 1;

/// The pair number used for the cell color + background cell color
const LIVING_CELL_PAIR: chtype = 2;

impl Pancurses {
    pub fn new() -> Pancurses {
        let main_window = initscr();

        // Causes getch to return an error if there is no character queued
        // (the default is to block on getch)
        main_window.nodelay(true);

        // Initialize the eight basic default colors
        start_color();

        // Disable echoing of typed characters into the terminal
        noecho();

        set_title("Rust of Life");

        Pancurses {
            main_window: main_window,
            cell_character: '#',
            cell_color: 0,
            cell_background_color: 0,
            background_color: 0,
            next_color_index: 8 // 0...7 are occupied by the default colors
        }.cell_color(Color::from(color_content(COLOR_MAGENTA)))
        .cell_background_color(Color::from(color_content(COLOR_BLUE)))
        .background_color(Color::from(color_content(COLOR_BLACK)))
    }

    pub fn cell_color(mut self, color: Color) -> Pancurses {
        self.set_cell_color(color);
        self
    }

    pub fn cell_background_color(mut self, color: Color) -> Pancurses {
        self.set_cell_background_color(color);
        self
    }

    pub fn background_color(mut self, color: Color) -> Pancurses {
        self.set_background_color(color);
        self
    }

    pub fn set_cell_color(&mut self, color: Color) {
        self.cell_color = self.init_color(color);
        init_pair(
            LIVING_CELL_PAIR as i16,
            self.cell_color,
            self.cell_background_color
        );
    }

    pub fn set_cell_background_color(&mut self, color: Color) {
        self.cell_background_color = self.init_color(color);
        init_pair(
            LIVING_CELL_PAIR as i16,
            self.cell_color,
            self.cell_background_color
        );
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = self.init_color(color);
        init_pair(
            DEAD_CELL_PAIR as i16,
            self.background_color,
            self.background_color
        );
    }


    fn init_color<C>(&mut self, color: C) -> i16
        where C: Into<(i16, i16, i16)> {

        let (r, g, b) = C::into(color);

        curses_init_color(
            self.next_color_index,
            (r as f32 * COLOR_MAGIC) as i16,
            (g as f32 * COLOR_MAGIC) as i16,
            (b as f32 * COLOR_MAGIC) as i16
        );

        self.next_color_index += 1;
        (self.next_color_index - 1)
    }

    fn draw_cell(&self, x: i32, y: i32, alive: bool) {
        self.main_window.mv(y, x);
        match alive {
            true => {
                self.main_window.addch(
                    self.cell_character as chtype | COLOR_PAIR(LIVING_CELL_PAIR) 
                )
            },
            false => {
                self.main_window.addch(
                    self.cell_character as chtype | COLOR_PAIR(DEAD_CELL_PAIR) 
                )
            }
        };
    }

    fn refresh(&self) {
        self.main_window.refresh();
    }
}

impl GridDisplayer for Pancurses {
    fn draw(&self, receiver: Receiver<Arc<RwLock<Grid>>>) {
        loop {
            let locked_grid: Arc<RwLock<Grid>> = receiver.recv().unwrap();
            let lock_guard: RwLockReadGuard<Grid> = locked_grid.read().unwrap();
            let ref grid: Grid = *lock_guard;

            for (y, row) in grid.into_iter().enumerate() {
                for (x, cell) in row.into_iter().enumerate() {
                    self.draw_cell(x as i32, y as i32, cell.is_alive());
                }
            }

            self.refresh();

            match self.main_window.getch() {
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
