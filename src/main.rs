extern crate core;

use core::fmt;
use nannou::prelude::*;

#[derive(Clone)]
enum CellState {
    Empty,
    Head,
    Tail,
    Conductor,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellState::Empty => write!(f, "."),
            CellState::Head => write!(f, ">"),
            CellState::Tail => write!(f, "<"),
            CellState::Conductor => write!(f, "-"),
        }
    }
}

const GRID_HEIGHT:usize = 10;
const GRID_WIDTH:usize = 30;

fn print_grid(grid: &Vec<CellState>, width: usize, height: usize)
{
    for h in 0..height {
        for w in 0..width {
            print!("{}", grid[h*height + w]);
        }
        println!();
    }
}

fn main() {
    println!("WireWorld - Paweł Reich s193682 2024");

    let grid = vec![CellState::Empty; (GRID_HEIGHT * GRID_WIDTH) as usize];

    print_grid(&grid, GRID_WIDTH, GRID_HEIGHT);

}
fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}