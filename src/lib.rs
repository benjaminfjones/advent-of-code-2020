pub mod util {
    use std::fs::File;
    use std::io::{self, BufRead, Read};

    /// Return an iterator over lines in a txt file.
    ///
    /// Example:
    /// ```
    /// # use aoc_2020::util::read_lines;
    /// for line in read_lines("inputs/d1").unwrap() {
    ///     if let Ok(ln) = line {
    ///          println!("{}", ln);
    ///     }
    /// }
    /// ```
    pub fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
        let in_file = File::open(path)?;
        Ok(io::BufReader::new(in_file).lines())
    }
    
    /// Read entire file path into a String
    pub fn read_to_string(path: &str) -> io::Result<String> {
        let mut in_file = File::open(path)?;
        let mut result = String::new();
        in_file.read_to_string(&mut result)?;
        Ok(result)
    }
}

pub mod cylinder {
    pub struct Grid<T> {
        rows: usize,
        cols: usize,
        content: Vec<T>,
    }
    
    impl<T> Grid<T> {
        pub fn new(rows: usize, cols: usize, fill: T) -> Self
            where T: Clone {
            let mut content =Vec::new();
            content.resize(rows*cols, fill);
            Grid {
                rows: rows,
                cols: cols,
                content: content,
            }
        }
        
        pub fn from_vec(rows: usize, cols: usize, content: Vec<T>) -> Result<Self, &'static str> {
            if content.len() == rows * cols {
                Ok(Grid {
                    rows: rows,
                    cols: cols,
                    content: content,
                })
            } else {
                Err("rows * cols vs. content length mismatch")
            }
        }
        
        /// Get element from the grid at (row, col), wrapping col modulo the number of columns
        /// on the grid.
        pub fn get(&self, row: usize, col: usize) -> Option<&T> {
            if row < self.rows {
                Some(&self.content[row*self.cols + (col % self.cols)])
            } else {
                None
            }
        }

        /// Set element on the grid at (row, col), wrapping col modulo the number of columns
        /// on the grid.
        pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), ()> {
            if row < self.rows {
                self.content[row*self.cols + (col % self.cols)] = value;
                Ok(())
            } else {
                Err(())
            }
        }
    }

    #[cfg(test)]
    mod test_util {
        use super::*;

        #[test]
        fn test_cylinder() {
            let mut cyl = Grid::new(5, 5, false);
            assert_eq!(cyl.get(0, 0), Some(&false));
            assert_eq!(cyl.get(0,501), Some(&false));
            assert_eq!(cyl.get(501, 0), None);
            
            assert!(cyl.set(1, 1, true).is_ok());
            assert_eq!(cyl.get(1, 1), Some(&true));

            assert!(cyl.set(1, 101, true).is_ok());
            assert_eq!(cyl.get(1, 101), Some(&true));
        }
    }
}
