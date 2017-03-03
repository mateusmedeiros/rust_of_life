use std::io;
use std::io::{ BufRead, BufReader, Read, Seek, SeekFrom };
use std::fs::File;

use models::Grid;

pub fn read_grid_from_file(input_file_path: &String) -> Result<Grid, io::Error> {
    let file = try!(File::open(input_file_path));
    let mut reader = BufReader::new(file);

    let x = reader.by_ref().lines().next().unwrap().unwrap().chars().count();
    reader.seek(SeekFrom::Start(0)).unwrap();

    let y = reader.by_ref().lines().count();
    reader.seek(SeekFrom::Start(0)).unwrap();

    let mut grid = Grid::new(x, y);
    for (y, line) in reader.lines().enumerate() {
        for (x, character) in line.unwrap().chars().enumerate() {
            match character {
                'o' => { grid[y][x].arise() },
                _ => { grid[y][x].die() }
            }
        }
    }

    Ok(grid)
}
