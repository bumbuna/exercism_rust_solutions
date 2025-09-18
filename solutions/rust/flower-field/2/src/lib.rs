struct Cell {
    row: i32,
    col: i32,
    c: u8,
}

struct Garden<'a> {
    rows: i32,
    cols: i32,
    grid: &'a [&'a str],
}

impl Cell {
    fn new(row: i32, col: i32, c: u8) -> Self {
        Cell { row, col, c }
    }
    fn left_neighbour(&self, garden: &Garden) -> Option<Cell> {
        garden.get_cell_at(self.row, self.col - 1)
    }

    fn right_neighbour(&self, garden: &Garden) -> Option<Cell> {
        garden.get_cell_at(self.row, self.col + 1)
    }

    fn top_neighbour(&self, garden: &Garden) -> Option<Cell> {
        garden.get_cell_at(self.row - 1, self.col)
    }

    fn bottom_neighbour(&self, garden: &Garden) -> Option<Cell> {
        garden.get_cell_at(self.row + 1, self.col)
    }

    fn top_left_diagonal_neighbour(&self, garden: &Garden) -> Option<Cell> {
        garden.get_cell_at(self.row - 1, self.col - 1)
    }

    fn top_right_diagonal_neighbour(&self, garden: &Garden) -> Option<Cell> {
        garden.get_cell_at(self.row - 1, self.col + 1)
    }

    fn bottom_left_diagonal_neighbour(&self, garden: &Garden) -> Option<Cell> {
        garden.get_cell_at(self.row + 1, self.col - 1)
    }

    fn bottom_right_diagonal_neighbour(&self, garden: &Garden) -> Option<Cell> {
        garden.get_cell_at(self.row + 1, self.col + 1)
    }

    fn has_flower(&self) -> bool {
        !self.c.to_ascii_uppercase().is_ascii_whitespace()
    }

    fn flowered_neigbours(&self, garden: &Garden) -> String {
        let mut i = 0;
        if let Some(n) = self.left_neighbour(garden)
            && n.has_flower()
        {
            i += 1
        };
        if let Some(n) = self.right_neighbour(garden)
            && n.has_flower()
        {
            i += 1
        };
        if let Some(n) = self.top_neighbour(garden)
            && n.has_flower()
        {
            i += 1
        };
        if let Some(n) = self.bottom_neighbour(garden)
            && n.has_flower()
        {
            i += 1
        };
        if let Some(n) = self.top_left_diagonal_neighbour(garden)
            && n.has_flower()
        {
            i += 1
        };
        if let Some(n) = self.top_right_diagonal_neighbour(garden)
            && n.has_flower()
        {
            i += 1
        };
        if let Some(n) = self.bottom_left_diagonal_neighbour(garden)
            && n.has_flower()
        {
            i += 1
        };
        if let Some(n) = self.bottom_right_diagonal_neighbour(garden)
            && n.has_flower()
        {
            i += 1
        };

        if i == 0 {
            String::from(" ")
        } else {
            i.to_string()
        }
    }
}

impl<'a> Garden<'a> {
    fn new(rows: i32, cols: i32, grid: &'a [&'a str]) -> Self {
        Garden { rows, cols, grid }
    }

    fn execute(&self) -> Vec<String> {
        let mut v = Vec::new();
        for row in 0..self.rows {
            let mut s = String::new();
            for col in 0..self.cols {
                let c = Cell::new(row, col, self.grid[row as usize].as_bytes()[col as usize]);
                if c.has_flower() {
                    s.push('*');
                } else {
                    s.push_str(&c.flowered_neigbours(self));
                }
            }
            v.push(s);
        }
        v
    }

    fn get_cell_at(&self, row: i32, col: i32) -> Option<Cell> {
        if row >= 0 && col >= 0 && row < self.rows && col < self.cols {
            Some(Cell::new(
                row,
                col,
                self.grid[row as usize].as_bytes()[col as usize],
            ))
        } else {
            None
        }
    }
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].len();
    if cols == 0 {
        return vec!["".to_string()];
    }
    let garden = Garden::new(garden.len() as i32, garden[0].len() as i32, garden);
    garden.execute()
}
