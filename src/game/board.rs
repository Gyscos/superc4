use super::Player;

// Limited to 128 should be enough...
pub type Pos = (i8,i8);

/// Represents a cell from the board. Each cell can be empty,
/// blocked, or full with a player's disk.
#[derive(Copy,Clone,PartialEq)]
pub enum Cell {
    Full(Player),
    Empty,
    Blocked,
}

/// Represents the entire board of a game.
pub struct Board {
    pub size: usize,
    cells: Vec<Cell>,
}

#[derive(Clone,Copy)]
pub struct Direction {
    x: i8,
    y: i8,
}

impl Direction {
    pub const UP: Direction = Direction{x: 0, y: 1};
    pub const DOWN: Direction = Direction{x: 0, y: -1};
    pub const LEFT: Direction = Direction{x: -1, y: 0};
    pub const RIGHT: Direction = Direction{x: 1, y: 0};
    pub const UP_RIGHT: Direction = Direction{x: 1, y: 1};
    pub const UP_LEFT: Direction = Direction{x:-1, y: 1};
    pub const DOWN_LEFT: Direction = Direction{x:-1,y:-1};
    pub const DOWN_RIGHT: Direction = Direction{x:1,y:-1};

    pub const CARDINAL: &'static [Direction] = &[Direction::UP,Direction::LEFT,Direction::DOWN,Direction::RIGHT];
    pub const AXIS: &'static [Direction] = &[Direction::UP,Direction::RIGHT,Direction::UP_RIGHT,Direction::DOWN_RIGHT];

    pub fn flip(self) -> Self {
        Direction{x: -self.x, y: -self.y}
    }

    pub fn from(self, (x,y): Pos) -> Pos {
        (x + self.x, y + self.y)
    }
}



impl Board {

    pub fn new(size: usize) -> Self {
        Board {
            size: size,
            cells: (0..size*size).map(|_| Cell::Empty).collect(),
        }
    }

    pub fn can_go(&self, (x,y): Pos, dir: Direction) -> bool {
        self.is_valid((x+dir.x, y+dir.y))
    }

    pub fn is_valid(&self, (x,y): Pos) -> bool {
        x >= 0 && x < self.size as i8
            && y >= 0 && y < self.size as i8
    }

    pub fn is_empty(&self, pos: Pos) -> bool {
        self.get(pos) == Cell::Empty
    }

    pub fn get(&self, pos: Pos) -> Cell {
        let cell_id = self.cell_id(pos);
        self.cells[cell_id]
    }

    pub fn block(&mut self, pos: Pos) {
        self.set_cell(pos, Cell::Blocked)
    }

    pub fn play(&mut self, pos: Pos, player: Player) {
        self.set_cell(pos, Cell::Full(player))
    }

    fn set_cell(&mut self, pos: Pos, cell: Cell) {
        let cell_id = self.cell_id(pos);
        self.cells[cell_id] = cell;
    }

    fn cell_id(&self, (x,y): Pos) -> usize {
        if x < 0 || y < 0 { panic!("invalid cell: {}:{}", x, y); }

        x as usize + self.size * y as usize
    }
}
