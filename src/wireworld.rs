use core::fmt;
use std::fs;

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
    pub fn new(width: u32, height: u32) -> Grid {
        Grid {
            cells: vec![CellState::Empty; (width * height) as usize],
            width: width,
            height: height,
        }
    }

    pub fn from_file(file_path: &str) -> Result<Grid, &str> {
        let content = fs::read_to_string(file_path);
        if content.is_err() {
            eprintln!("Could not read file: {}", file_path);
            return Err("Could not read file");
        }

        let content = content.unwrap();
        let lines = content.split('\n');

        let mut cells: Vec<CellState> = Vec::new();
        let mut width :i32 = -1;
        let mut height :u32 = 0;

        for line in lines {
            let mut new_width = 0;
            for c in line.chars() {
                match c {
                    '.' => cells.push(CellState::Empty),
                    '>' => cells.push(CellState::Head),
                    '<' => cells.push(CellState::Tail),
                    '-' => cells.push(CellState::Conductor),
                    _ => { continue; }
                }

                new_width += 1;
            }
            if width != -1 && new_width != width {
                eprintln!("File {} is malformed.", file_path);
                return Err("File is malformed.");
            }
            width = new_width;
            height += 1;
        }

        println!("Finished loading {}x{} grid from {}", width, height, file_path);
        
        Ok(Grid {
            cells: cells,
            width: width as u32,
            height: height,
        })
    }

    pub fn to_file(&self, file_path: &str) -> std::io::Result<()>
    {
        let data:String = self.serialize();
        fs::write(file_path, data)
    }

    pub fn serialize(&self) -> String {
        let mut serialized: String = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                serialized.push_str(format!("{}", self.get_cell(x as i32,y as i32)).as_str());
            }
            serialized.push('\n');
        }

        serialized
    }
    pub fn pretty_print(&self) {
        println!("{}", self.serialize());
    }
    pub fn get_width(&self) -> u32 { self.width }
    pub fn get_height(&self) -> u32 { self.height }
    pub fn get_cell(&self, x: i32, y: i32) -> CellState {
        if x >= self.width as i32 || y >= self.height as i32
        || x < 0 || y < 0 {
            return CellState::Empty;
        }

        self.cells[self.idx(x as u32, y as u32)].clone()
    }
    pub fn set_cell(&mut self, x: u32, y: u32, new_val: CellState) {

        if x >= self.width || y >= self.height{
            return;
        }

        let idx = self.idx(x, y);
        self.cells[idx] = new_val;
    }
    fn idx(&self, x: u32, y: u32) -> usize { (y * self.width + x) as usize }

    fn count_neighbours(&self, looking_for: CellState, x: u32, y: u32) -> u32 {
        let mut count: u32 = 0;
        let x:i32 = x as i32;
        let y:i32 = y as i32;
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
                let mut cell = self.get_cell(x as i32, y as i32);
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