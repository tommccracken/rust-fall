#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum CellType {
    Empty = 0,
    Wall = 1,
    Wood = 2,
    Sand = 3,
    Water = 4,
    Oil = 5,
    Steam = 6,
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

    #[cfg(feature = "parallel")]
    pub fn update(&mut self) {
        // Increment simulation step count
        self.steps += 1;
        // Todo: Implement a solution that calls this function on multiple blocks in parallel
        self.update_block(0, 0, self.grid.columns, self.grid.rows);
        // Reset updated status to false
        self.grid.cells.par_iter_mut().for_each(|n| {
            for element in n {
                element.updated = false;
            }
        });
    }

    #[cfg(not(feature = "parallel"))]
    pub fn update(&mut self) {
        // Increment simulation step count
        self.steps += 1;
        self.update_block(0, 0, self.grid.columns, self.grid.rows);
        // Reset updated status to false
        for row in 0..self.grid.rows {
            for column in 0..self.grid.columns {
                self.grid.cells[row][column].updated = false;
            }
        }
    }

    fn update_block(&mut self, start_column: usize, start_row: usize, columns: usize, rows: usize) {
        // Update a 'block' of world grid cells (either a subset of the world, or the entire world)
        for row in start_row..(start_row + rows) {
            for column in start_column..(start_column + columns) {
                let cell = self.grid.cells[row][column].clone();
                if self.grid.cells[row][column].updated == false {
                    if cell.cell_type == CellType::Sand {
                        // If the row index is not 0, i.e. it is not the bottom row
                        if row > 0 {
                            if self.grid.cells[row - 1][column].cell_type != CellType::Sand
                                && self.grid.cells[row - 1][column].cell_type != CellType::Wall
                                && self.grid.cells[row - 1][column].cell_type != CellType::Wood
                            {
                                // If the cell below is water or oil, displace the water or oil
                                if self.grid.cells[row - 1][column].cell_type == CellType::Water
                                    || self.grid.cells[row - 1][column].cell_type == CellType::Oil
                                {
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
                                        self.grid.cells[row][column - 1].cell_type =
                                            self.grid.cells[row - 1][column].cell_type;
                                        self.grid.cells[row - 1][column].cell_type = CellType::Sand;
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row][column - 1].updated = true;
                                        self.grid.cells[row - 1][column].updated = true;
                                    } else if up_right {
                                        self.grid.cells[row][column + 1].cell_type =
                                            self.grid.cells[row - 1][column].cell_type;
                                        self.grid.cells[row - 1][column].cell_type = CellType::Sand;
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
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
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Wood
                                    {
                                        down_left = true;
                                    }
                                }
                                if column < self.grid.columns - 1 {
                                    if self.grid.cells[row - 1][column + 1].cell_type
                                        != CellType::Sand
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Wood
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
                                && self.grid.cells[row - 1][column].cell_type != CellType::Wood
                            {
                                // If the cell below is oil, displace the oil
                                if self.grid.cells[row - 1][column].cell_type == CellType::Oil {
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
                                        self.grid.cells[row][column - 1].cell_type =
                                            self.grid.cells[row - 1][column].cell_type;
                                        self.grid.cells[row - 1][column].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row][column - 1].updated = true;
                                        self.grid.cells[row - 1][column].updated = true;
                                    } else if up_right {
                                        self.grid.cells[row][column + 1].cell_type =
                                            self.grid.cells[row - 1][column].cell_type;
                                        self.grid.cells[row - 1][column].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row][column].cell_type = CellType::Empty;
                                        self.grid.cells[row][column + 1].updated = true;
                                        self.grid.cells[row - 1][column].updated = true;
                                    } else {
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row - 1][column].cell_type;
                                        self.grid.cells[row - 1][column].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row - 1][column].updated = true;
                                    }
                                } else {
                                    self.grid.cells[row][column].cell_type =
                                        self.grid.cells[row - 1][column].cell_type;
                                    self.grid.cells[row - 1][column].cell_type = CellType::Water;
                                    self.grid.cells[row - 1][column].updated = true;
                                }
                            } else {
                                let mut down_left = false;
                                let mut down_right = false;
                                let mut left = false;
                                let mut right = false;
                                if column > 0 {
                                    if self.grid.cells[row - 1][column - 1].cell_type
                                        != CellType::Sand
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Water
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Wood
                                    {
                                        down_left = true;
                                    }
                                    if self.grid.cells[row][column - 1].cell_type != CellType::Sand
                                        && self.grid.cells[row][column - 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row][column - 1].cell_type
                                            != CellType::Water
                                        && self.grid.cells[row][column - 1].cell_type
                                            != CellType::Wood
                                    {
                                        left = true;
                                    }
                                }
                                if column < self.grid.columns - 1 {
                                    if self.grid.cells[row - 1][column + 1].cell_type
                                        != CellType::Sand
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Water
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Wood
                                    {
                                        down_right = true;
                                    }
                                    if self.grid.cells[row][column + 1].cell_type != CellType::Sand
                                        && self.grid.cells[row][column + 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row][column + 1].cell_type
                                            != CellType::Water
                                        && self.grid.cells[row][column + 1].cell_type
                                            != CellType::Wood
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
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row][column + 1].cell_type;
                                        self.grid.cells[row][column + 1].cell_type =
                                            CellType::Water;
                                        self.grid.cells[row][column + 1].updated = true;
                                    } else if left {
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row][column - 1].cell_type;
                                        self.grid.cells[row][column - 1].cell_type =
                                            CellType::Water;
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
                                if self.grid.cells[row][column - 1].cell_type != CellType::Sand
                                    && self.grid.cells[row][column - 1].cell_type != CellType::Wall
                                    && self.grid.cells[row][column - 1].cell_type != CellType::Water
                                    && self.grid.cells[row][column - 1].cell_type != CellType::Wood
                                {
                                    left = true;
                                }
                            }
                            if column < self.grid.columns - 1 {
                                if self.grid.cells[row][column + 1].cell_type != CellType::Sand
                                    && self.grid.cells[row][column + 1].cell_type != CellType::Wall
                                    && self.grid.cells[row][column + 1].cell_type != CellType::Water
                                    && self.grid.cells[row][column + 1].cell_type != CellType::Wood
                                {
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
                                self.grid.cells[row][column].cell_type =
                                    self.grid.cells[row][column + 1].cell_type;
                                self.grid.cells[row][column + 1].cell_type = CellType::Water;
                                self.grid.cells[row][column + 1].updated = true;
                            } else if left {
                                self.grid.cells[row][column].cell_type =
                                    self.grid.cells[row][column - 1].cell_type;
                                self.grid.cells[row][column - 1].cell_type = CellType::Water;
                                self.grid.cells[row][column - 1].updated = true;
                            }
                        }
                    } else if cell.cell_type == CellType::Oil {
                        // If the row index is not 0, i.e. it is not the bottom row
                        if row > 0 {
                            if self.grid.cells[row - 1][column].cell_type != CellType::Sand
                                && self.grid.cells[row - 1][column].cell_type != CellType::Wall
                                && self.grid.cells[row - 1][column].cell_type != CellType::Water
                                && self.grid.cells[row - 1][column].cell_type != CellType::Oil
                                && self.grid.cells[row - 1][column].cell_type != CellType::Wood
                            {
                                self.grid.cells[row][column].cell_type =
                                    self.grid.cells[row - 1][column].cell_type;
                                self.grid.cells[row - 1][column].cell_type = CellType::Oil;
                                self.grid.cells[row - 1][column].updated = true;
                            } else {
                                let mut down_left = false;
                                let mut down_right = false;
                                let mut left = false;
                                let mut right = false;
                                if column > 0 {
                                    if self.grid.cells[row - 1][column - 1].cell_type
                                        != CellType::Sand
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Water
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Oil
                                        && self.grid.cells[row - 1][column - 1].cell_type
                                            != CellType::Wood
                                    {
                                        down_left = true;
                                    }
                                    if self.grid.cells[row][column - 1].cell_type != CellType::Sand
                                        && self.grid.cells[row][column - 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row][column - 1].cell_type
                                            != CellType::Water
                                        && self.grid.cells[row][column - 1].cell_type
                                            != CellType::Oil
                                        && self.grid.cells[row][column - 1].cell_type
                                            != CellType::Wood
                                    {
                                        left = true;
                                    }
                                }
                                if column < self.grid.columns - 1 {
                                    if self.grid.cells[row - 1][column + 1].cell_type
                                        != CellType::Sand
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Water
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Oil
                                        && self.grid.cells[row - 1][column + 1].cell_type
                                            != CellType::Wood
                                    {
                                        down_right = true;
                                    }
                                    if self.grid.cells[row][column + 1].cell_type != CellType::Sand
                                        && self.grid.cells[row][column + 1].cell_type
                                            != CellType::Wall
                                        && self.grid.cells[row][column + 1].cell_type
                                            != CellType::Water
                                        && self.grid.cells[row][column + 1].cell_type
                                            != CellType::Oil
                                        && self.grid.cells[row][column + 1].cell_type
                                            != CellType::Wood
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
                                            CellType::Oil;
                                        self.grid.cells[row - 1][column - 1].updated = true;
                                    } else if down_right {
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row - 1][column + 1].cell_type;
                                        self.grid.cells[row - 1][column + 1].cell_type =
                                            CellType::Oil;
                                        self.grid.cells[row - 1][column + 1].updated = true;
                                    }
                                } else if left || right {
                                    if right {
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row][column + 1].cell_type;
                                        self.grid.cells[row][column + 1].cell_type = CellType::Oil;
                                        self.grid.cells[row][column + 1].updated = true;
                                    } else if left {
                                        self.grid.cells[row][column].cell_type =
                                            self.grid.cells[row][column - 1].cell_type;
                                        self.grid.cells[row][column - 1].cell_type = CellType::Oil;
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
                                if self.grid.cells[row][column - 1].cell_type != CellType::Sand
                                    && self.grid.cells[row][column - 1].cell_type != CellType::Wall
                                    && self.grid.cells[row][column - 1].cell_type != CellType::Water
                                    && self.grid.cells[row][column - 1].cell_type != CellType::Oil
                                    && self.grid.cells[row][column - 1].cell_type != CellType::Wood
                                {
                                    left = true;
                                }
                            }
                            if column < self.grid.columns - 1 {
                                if self.grid.cells[row][column + 1].cell_type != CellType::Sand
                                    && self.grid.cells[row][column + 1].cell_type != CellType::Wall
                                    && self.grid.cells[row][column + 1].cell_type != CellType::Water
                                    && self.grid.cells[row][column + 1].cell_type != CellType::Oil
                                    && self.grid.cells[row][column + 1].cell_type != CellType::Wood
                                {
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
                                self.grid.cells[row][column].cell_type =
                                    self.grid.cells[row][column + 1].cell_type;
                                self.grid.cells[row][column + 1].cell_type = CellType::Oil;
                                self.grid.cells[row][column + 1].updated = true;
                            } else if left {
                                self.grid.cells[row][column].cell_type =
                                    self.grid.cells[row][column - 1].cell_type;
                                self.grid.cells[row][column - 1].cell_type = CellType::Oil;
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
    }

    pub fn clear(&mut self) {
        for row in 0..self.grid.rows {
            for column in 0..self.grid.columns {
                self.grid.cells[row][column].cell_type = CellType::Empty;
            }
        }
    }
}
