#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum CellType {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Steam = 4,
}

#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
    updated: bool,
}

pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(world_size: usize) -> Self {
        Grid {
            rows: world_size,
            columns: world_size,
            cells: vec![
                vec![
                    Cell {
                        cell_type: CellType::Empty,
                        updated: false
                    };
                    world_size
                ];
                world_size
            ],
        }
    }
}

pub struct World {
    pub steps: u32,
    pub world_size: usize, // World height and width (in number of cells)
    pub grid: Grid,
}

impl World {
    pub fn new(world_size: usize) -> Self {
        World {
            steps: 0,
            world_size: world_size,
            grid: Grid::new(world_size),
        }
    }

    pub fn update(&mut self) {
        // Increment simulation step count
        self.steps += 1;
        // Loop through grid cells
        for row in 0..self.grid.rows {
            for column in 0..self.grid.columns {
                let cell = self.grid.cells[row][column].clone();
                if self.grid.cells[row][column].updated == false {
                    if cell.cell_type == CellType::Sand {
                        // If the row index is not 0, i.e. it is not the bottom row
                        if row > 0 {
                            if self.grid.cells[row - 1][column].cell_type != CellType::Sand
                                && self.grid.cells[row - 1][column].cell_type != CellType::Wall
                            {
                                // If the cell below is water, displace the water
                                if self.grid.cells[row - 1][column].cell_type == CellType::Water {
                                    let mut up_left = false;
                                    let mut up_right = false;
                                    if column > 0 {
                                        if self.grid.cells[row][column - 1].cell_type
                                            == CellType::Empty
                                        {
                                            up_left = true;
                                        }
                                    }
                                    if column < self.grid.columns - 1 {
                                        if self.grid.cells[row][column + 1].cell_type
                                            == CellType::Empty
                                        {
                                            up_right = true;
                                        }
                                    }
                                    if up_left && up_right {
                                        if rand::random() {
                                            up_left = false;
                                        } else {
                                            up_right = false;
                                        }
                                    }
                                    if up_left {
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row - 1][column].cell_type = CellType::Sand;
                                        self.grid.cells[row][column - 1].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row][column - 1].updated = true;
                                        self.grid.cells[row - 1][column].updated = true;
                                    } else if up_right {
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row - 1][column].cell_type = CellType::Sand;
                                        self.grid.cells[row][column + 1].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row][column + 1].updated = true;
                                        self.grid.cells[row - 1][column].updated = true;
                                    } else {
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row - 1][column].cell_type;
                                        self.grid.cells[row - 1][column].cell_type = CellType::Sand;
                                        self.grid.cells[row - 1][column].updated = true;
                                    }
                                } else {
                                    self.grid.cells[row][column].cell_type =
                                        self.grid.cells[row - 1][column].cell_type;
                                    self.grid.cells[row - 1][column].cell_type = CellType::Sand;
                                    self.grid.cells[row - 1][column].updated = true;
                                }
                            } else {
                                let mut down_left = false;
                                let mut down_right = false;
                                if column > 0 {
                                    if self.grid.cells[row - 1][column - 1].cell_type
                                        != CellType::Sand
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Wall
                                    {
                                        down_left = true;
                                    }
                                }
                                if column < self.grid.columns - 1 {
                                    if self.grid.cells[row - 1][column + 1].cell_type
                                        != CellType::Sand
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Wall
                                    {
                                        down_right = true;
                                    }
                                }
                                if down_left && down_right {
                                    if rand::random() {
                                        down_left = false;
                                    } else {
                                        down_right = false;
                                    }
                                }
                                if down_left {
                                    self.grid.cells[row][column].cell_type =
                                        self.grid.cells[row - 1][column - 1].cell_type;
                                    self.grid.cells[row - 1][column - 1].cell_type = CellType::Sand;
                                    self.grid.cells[row - 1][column - 1].updated = true;
                                } else if down_right {
                                    self.grid.cells[row][column].cell_type =
                                        self.grid.cells[row - 1][column + 1].cell_type;
                                    self.grid.cells[row - 1][column + 1].cell_type = CellType::Sand;
                                    self.grid.cells[row - 1][column + 1].updated = true;
                                }
                            }
                        }
                    } else if cell.cell_type == CellType::Water {
                        // If the row index is not 0, i.e. it is not the bottom row
                        if row > 0 {
                            if self.grid.cells[row - 1][column].cell_type != CellType::Sand
                                && self.grid.cells[row - 1][column].cell_type != CellType::Wall
                                && self.grid.cells[row - 1][column].cell_type != CellType::Water
                            {
                                self.grid.cells[row][column].cell_type =
                                    self.grid.cells[row - 1][column].cell_type;
                                self.grid.cells[row - 1][column].cell_type = CellType::Water;
                                self.grid.cells[row - 1][column].updated = true;
                            } else {
                                let mut down_left = false;
                                let mut down_right = false;
                                let mut left = false;
                                let mut right = false;
                                if column > 0 {
                                    if self.grid.cells[row - 1][column - 1].cell_type
                                        == CellType::Empty
                                        || self.grid.cells[row - 1][column - 1].cell_type
                                            == CellType::Steam
                                    {
                                        down_left = true;
                                    }
                                    if self.grid.cells[row][column - 1].cell_type == CellType::Empty
                                    {
                                        left = true;
                                    }
                                }
                                if column < self.grid.columns - 1 {
                                    if self.grid.cells[row - 1][column + 1].cell_type
                                        == CellType::Empty
                                        || self.grid.cells[row - 1][column + 1].cell_type
                                            == CellType::Steam
                                    {
                                        down_right = true;
                                    }
                                    if self.grid.cells[row][column + 1].cell_type == CellType::Empty
                                    {
                                        right = true;
                                    }
                                }
                                if down_left && down_right {
                                    if rand::random() {
                                        down_left = false;
                                    } else {
                                        down_right = false;
                                    }
                                }
                                if left && right {
                                    if rand::random() {
                                        left = false;
                                    } else {
                                        right = false;
                                    }
                                }
                                if down_left || down_right {
                                    if down_left {
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row - 1][column - 1].cell_type;
                                        self.grid.cells[row - 1][column - 1].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row - 1][column - 1].updated = true;
                                    } else if down_right {
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row - 1][column + 1].cell_type;
                                        self.grid.cells[row - 1][column + 1].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row - 1][column + 1].updated = true;
                                    }
                                } else {
                                    if right {
                                        self.grid.cells[row][column + 1].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row][column + 1].updated = true;
                                    } else if left {
                                        self.grid.cells[row][column - 1].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row][column - 1].updated = true;
                                    }
                                }
                            }
                        }
                        // Otherwise if the row index is 0, i.e. it is the bottom row
                        else {
                            let mut left = false;
                            let mut right = false;
                            if column > 0 {
                                if self.grid.cells[row][column - 1].cell_type == CellType::Empty {
                                    left = true;
                                }
                            }
                            if column < self.grid.columns - 1 {
                                if self.grid.cells[row][column + 1].cell_type == CellType::Empty {
                                    right = true;
                                }
                            }
                            if left && right {
                                if rand::random() {
                                    left = false;
                                } else {
                                    right = false;
                                }
                            }
                            if right {
                                self.grid.cells[row][column + 1].cell_type = CellType::Water;
                                self.grid.cells[row][column].cell_type = CellType::Empty;
                                self.grid.cells[row][column + 1].updated = true;
                            } else if left {
                                self.grid.cells[row][column - 1].cell_type = CellType::Water;
                                self.grid.cells[row][column].cell_type = CellType::Empty;
                                self.grid.cells[row][column - 1].updated = true;
                            }
                        }
                    } else if cell.cell_type == CellType::Steam {
                        // Randomly condense steam back into water
                        if rand::random::<f32>() > 0.999 {
                            self.grid.cells[row][column].cell_type = CellType::Water;
                        }
                        // If the row is not the top row
                        else if row < self.grid.rows - 1 {
                            if self.grid.cells[row + 1][column].cell_type == CellType::Empty {
                                self.grid.cells[row][column].cell_type = CellType::Empty;
                                self.grid.cells[row + 1][column].cell_type = CellType::Steam;
                                self.grid.cells[row + 1][column].updated = true;
                            } else {
                                let mut up_left = false;
                                let mut up_right = false;
                                let mut left = false;
                                let mut right = false;
                                if column > 0 {
                                    if self.grid.cells[row + 1][column - 1].cell_type
                                        == CellType::Empty
                                    {
                                        up_left = true;
                                    }
                                    if self.grid.cells[row][column - 1].cell_type == CellType::Empty
                                    {
                                        left = true;
                                    }
                                }
                                if column < self.grid.columns - 1 {
                                    if self.grid.cells[row + 1][column + 1].cell_type
                                        == CellType::Empty
                                    {
                                        up_right = true;
                                    }
                                    if self.grid.cells[row][column + 1].cell_type == CellType::Empty
                                    {
                                        right = true;
                                    }
                                }
                                if up_left && up_right {
                                    if rand::random() {
                                        up_left = false;
                                    } else {
                                        up_right = false;
                                    }
                                }
                                if left && right {
                                    if rand::random() {
                                        left = false;
                                    } else {
                                        right = false;
                                    }
                                }
                                if up_left || up_right {
                                    if up_left {
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row + 1][column - 1].cell_type =
                                            CellType::Steam;
                                        self.grid.cells[row + 1][column - 1].updated = true;
                                    } else if up_right {
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row + 1][column + 1].cell_type =
                                            CellType::Steam;

                                        self.grid.cells[row + 1][column + 1].updated = true;
                                    }
                                } else {
                                    if right {
                                        self.grid.cells[row][column + 1].cell_type =
                                            CellType::Steam;
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row][column + 1].updated = true;
                                    } else if left {
                                        self.grid.cells[row][column - 1].cell_type =
                                            CellType::Steam;
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row][column - 1].updated = true;
                                    }
                                }
                            }
                        }
                        // Otherwise if it is the top row
                        else {
                            let mut left = false;
                            let mut right = false;
                            if column > 0 {
                                if self.grid.cells[row][column - 1].cell_type == CellType::Empty {
                                    left = true;
                                }
                            }
                            if column < self.grid.columns - 1 {
                                if self.grid.cells[row][column + 1].cell_type == CellType::Empty {
                                    right = true;
                                }
                            }
                            if left && right {
                                if rand::random() {
                                    left = false;
                                } else {
                                    right = false;
                                }
                            }
                            if right {
                                self.grid.cells[row][column + 1].cell_type = CellType::Steam;
                                self.grid.cells[row][column].cell_type = CellType::Empty;
                                self.grid.cells[row][column + 1].updated = true;
                            } else if left {
                                self.grid.cells[row][column - 1].cell_type = CellType::Steam;
                                self.grid.cells[row][column].cell_type = CellType::Empty;
                                self.grid.cells[row][column - 1].updated = true;
                            }
                        }
                    }
                }
                self.grid.cells[row][column].updated = true;
            }
        }
        // Reset updated status to false
        for row in 0..self.grid.rows {
            for column in 0..self.grid.columns {
                self.grid.cells[row][column].updated = false;
            }
        }
    }
}
