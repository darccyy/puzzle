use crate::GRID_SIZE;

type TileValue = Option<u32>;
type GridArray = [[TileValue; GRID_SIZE]; GRID_SIZE];

pub struct Grid(GridArray);

impl Grid {
    pub fn new() -> Self {
        let mut grid = GridArray::default();

        grid[0][0] = Some(1);
        grid[0][1] = Some(2);
        grid[0][2] = Some(3);
        grid[0][3] = Some(4);
        grid[1][0] = Some(5);
        grid[1][1] = Some(6);
        // grid[1][2] = Some(7);
        grid[1][3] = Some(8);
        grid[2][0] = Some(9);
        grid[2][1] = Some(10);
        grid[2][2] = Some(11);
        grid[2][3] = Some(12);
        grid[3][0] = Some(13);
        grid[3][1] = Some(14);
        grid[3][2] = Some(15);
        grid[3][3] = Some(16);

        Self(grid)
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    row: usize,
    col: usize,
}

pub struct GridTile<'a> {
    pub tile: &'a TileValue,
    pub x: usize,
    pub y: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = GridTile<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < GRID_SIZE {
            let tile = &self.grid.0[self.row][self.col];
            let x = self.col;
            let y = self.row;

            self.col += 1;
            if self.col == GRID_SIZE {
                self.col = 0;
                self.row += 1;
            }

            Some(GridTile { tile, x, y })
        } else {
            None
        }
    }
}