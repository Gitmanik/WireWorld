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
struct Grid {
    cells: Vec<CellState>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            cells: vec![CellState::Empty; (width * height) as usize],
            width: width,
            height: height,
        }
    }
    fn pretty_print(&self)
    {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_cell(x,y));
            }
            println!();
        }
    }
    fn get_cell(&self, x: usize, y: usize) -> &CellState { &self.cells[self.idx(x, y)] }
    fn idx(&self, x: usize, y: usize) -> usize { y * self.height + x }
}

fn main() {
    println!("WireWorld - Paweł Reich s193682 2024");

    let grid = Grid::new(30,10);
    grid.pretty_print();
}
fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}