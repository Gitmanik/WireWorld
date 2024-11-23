use core::fmt;

#[derive(Clone, PartialEq)]
pub enum CellState {
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
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
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
    pub fn get_width(&self) -> u32 {self.width}
    pub fn get_height(&self) -> u32 {self.height}
    pub fn get_cell(&self, x: u32, y: u32) -> CellState {
        if x > self.width || y > self.height {
            return CellState::Empty;
        }

        self.cells[self.idx(x, y) as usize].clone()
    }
    pub fn set_cell(&mut self, x: u32, y: u32, new_val: CellState) {

        assert!(x < self.width && y < self.height);

        let idx = self.idx(x, y);
        self.cells[idx] = new_val;
    }
    fn idx(&self, x: u32, y: u32) -> usize { (y * self.width + x) as usize}

    fn count_neighbours(&self, looking_for: CellState, x: u32, y: u32) -> u32
    {
        let mut count: u32 = 0;
        for x_test in x-1..=x+1 {
            for y_test in y-1..=y+1 {
                if self.get_cell(x_test, y_test) == looking_for {
                    count +=1;
                }
            }
        }
        count
    }
    
    pub fn tick(&mut self) {

        let mut new_cells = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let mut cell = self.get_cell(x, y);
                match cell
                {
                    CellState::Empty => {}
                    CellState::Head => {
                        cell = CellState::Tail;
                    }
                    CellState::Tail => {
                        cell = CellState::Conductor;
                    }
                    CellState::Conductor => {
                        let head_count = self.count_neighbours(CellState::Head, x, y);
                        if head_count == 1 || head_count == 2 {
                            cell = CellState::Head;
                        }
                    }
                }
                new_cells[self.idx(x, y)] = cell;
            }
        }
        self.cells = new_cells;
    }
}