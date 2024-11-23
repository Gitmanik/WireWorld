use core::fmt;

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
pub struct Grid {
    cells: Vec<CellState>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            cells: vec![CellState::Empty; (width * height) as usize],
            width: width,
            height: height,
        }
    }
    pub fn pretty_print(&self)
    {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_cell(x,y));
            }
            println!();
        }
    }
    pub fn get_cell(&self, x: usize, y: usize) -> &CellState { &self.cells[self.idx(x, y)] }
    fn idx(&self, x: usize, y: usize) -> usize { y * self.height + x }
}