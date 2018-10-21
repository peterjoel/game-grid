use std::mem;
use std::fmt::{self, Debug, Formatter};
use std::iter;

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Grid<T> {
        assert!(data.len() == width * height, "invalid data size: {}, w={}, h={}", data.len(), width, height);
        Grid { width, height, data }
    }

    pub fn get(&self, x: usize, y: usize) -> T 
    where 
        T: Copy,
    {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        self.data[y * self.width + x]
    }

    pub fn get_ref(&self, x: usize, y: usize) -> &T {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &self.data[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &mut self.data[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        *self.get_mut(x, y) = value;
    }

    pub fn get_neighbours(&self, x: usize, y: usize, res: &mut [T])
    where
        T: Copy,
    {
        if y == 0 {
            if x == 0 {
                res[0] = self.get(x, y + 1);
                res[1] = self.get(x + 1, y);
                res[2] = self.get(x + 1, y + 1);
            } else if x == self.width - 1 {
                res[0] = self.get(x, y + 1);
                res[1] = self.get(x - 1, y);
                res[2] = self.get(x - 1, y + 1);
            }
        } else if y == self.height - 1 {
            if x == 0 {
                res[0] = self.get(x, y - 1);
                res[1] = self.get(x + 1, y);
                res[2] = self.get(x + 1, y - 1);
            } else if x == self.width - 1 {
                res[0] = self.get(x, y - 1);
                res[1] = self.get(x - 1, y);
                res[2] = self.get(x - 1, y - 1);
            }
        } else {
            res[0] = self.get(x, y + 1);
            res[1] = self.get(x, y - 1);
            if x != self.width - 1 {
                res[2] = self.get(x + 1, y);
                res[3] = self.get(x + 1, y + 1);
                res[4] = self.get(x + 1, y - 1);
            }
            if x != 0 {
                res[5] = self.get(x - 1, y);
                res[6] = self.get(x - 1, y + 1);
                res[7] = self.get(x - 1, y - 1);
            }
        }
    }

    pub fn get_neighbours_wrapped(&self, x: usize, y: usize, res: &mut [T])
    where
        T: Copy,
    {
        let y_top = (y + self.height - 1) % self.height;
        let y_bottom = (y + 1) % self.height;
        let x_left = (x + self.width - 1) % self.width;
        let x_right = (x + 1) % self.width;
        res[0] = self.get(x_left, y_top);
        res[1] = self.get(x_left, y);
        res[2] = self.get(x_left, y_bottom);
        res[3] = self.get(x, y_top);
        res[4] = self.get(x, y_bottom);
        res[5] = self.get(x_right, y_top);
        res[6] = self.get(x_right, y);
        res[7] = self.get(x_right, y_bottom);
    }
}

pub struct Game<T> {
    old_grid: Grid<T>,
    grid: Grid<T>,
}

impl<T: Default + Clone> Game<T> {
    pub fn new(width: usize, height: usize) -> Game<T> {
        let grid = Grid::new(width, height, vec![T::default(); width * height]);
        let old_grid = Grid::new(width, height, vec![T::default(); width * height]);
        Game { old_grid, grid }
    }

    pub fn next_turn(&mut self) {
        mem::swap(&mut self.grid, &mut self.old_grid);
    }

    pub fn grid(&self) -> &Grid<T> {
        &self.grid
    }

    pub fn grid_mut(&mut self) -> &mut Grid<T> {
        &mut self.grid
    }

    pub fn old_grid(&self) -> &Grid<T> {
        &self.old_grid
    }
}


impl Debug for Grid<u8> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}\n", iter::repeat("-").take(self.width + 2).collect::<String>());
        for y in 0 .. self.height {
            write!(f, "|");
            for x in 0 .. self.width {
                write!(f, "{}", if self.get(x, y) == 0 { " " } else { "*" });
            }
            write!(f, "|\n", );
        }
        write!(f, "{}", iter::repeat("-").take(self.width + 2).collect::<String>());
        Ok(())
    }
}