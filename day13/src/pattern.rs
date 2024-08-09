pub struct Pattern {
    pub pattern: Vec<Vec<char>>,
}

impl Pattern {
    pub fn new(new_pattern: Vec<Vec<char>>) -> Self {
        Self {
            pattern: new_pattern,
        }
    }

    pub fn summarize(&self) -> (i8, i8) {
        let rows: i8 = self.pattern.len() as i8;
        let cols: i8 = self.pattern[0].len() as i8;

        // checking for row
        let mut mirror_row: i8 = -1;
        for r in 1..rows {
            let mut upper_bound: i8 = r - 1;
            let mut lower_bound: i8 = r;
            let mut reflection: bool = true;
            'outer: while upper_bound >= 0 && lower_bound < rows {
                for j in 0..cols {
                    if self.pattern[upper_bound as usize][j as usize]
                        != self.pattern[lower_bound as usize][j as usize]
                    {
                        reflection = false;
                        break 'outer;
                    }
                }
                upper_bound -= 1;
                lower_bound += 1;
            }
            if reflection {
                mirror_row = r;
            }
        }

        // checking for column
        let mut mirror_col: i8 = -1;
        for c in 1..cols {
            let mut left_bound: i8 = c - 1;
            let mut right_bound: i8 = c;
            let mut reflection: bool = true;
            'outer: while left_bound >= 0 && right_bound < cols {
                for i in 0..rows {
                    if self.pattern[i as usize][left_bound as usize]
                        != self.pattern[i as usize][right_bound as usize]
                    {
                        reflection = false;
                        break 'outer;
                    }
                }
                left_bound -= 1;
                right_bound += 1;
            }
            if reflection {
                mirror_col = c;
            }
        }

        (mirror_row, mirror_col)
    }
}
