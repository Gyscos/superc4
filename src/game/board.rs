pub type Pos = (usize,usize);

#[derive(Copy,Clone)]
pub enum Cell {
    Player(usize),
    Empty,
    Blocked,
}

pub struct Board {
    size: usize,
    cells: Vec<Cell>,
}

impl Board {

    pub fn new(size: usize) -> Self {
        Board {
            size: size,
            cells: (0..size*size).map(|_| Cell::Empty).collect(),
        }
    }

    pub fn get(&self, pos: Pos) -> Cell {
        let cell_id = self.cell_id(pos);
        self.cells[cell_id]
    }

    pub fn block(&mut self, pos: Pos) -> Result<(),()> {
        self.set_cell(pos, Cell::Blocked)
    }

    pub fn play(&mut self, pos: Pos, player: usize) -> Result<(),()> {
        self.set_cell(pos, Cell::Player(player))
    }

    fn set_cell(&mut self, pos: Pos, cell: Cell) -> Result<(),()> {
        let cell_id = self.cell_id(pos);
        match self.cells[cell_id] {
            Cell::Empty => {
                self.cells[cell_id] = cell;
                Ok(())
            },
            _ => Err(())
        }
    }

    fn cell_id(&self, (x,y): Pos) -> usize {
        x + self.size * y
    }
}
