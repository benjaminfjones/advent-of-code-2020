/// --- Day 3: Toboggan Trajectory ---
/// With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.
/// 
/// Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:
/// 
/// ..##.......
/// #...#...#..
/// .#....#..#.
/// ..#.#...#.#
/// .#...##..#.
/// ..#.##.....
/// .#.#.#....#
/// .#........#
/// #.##...#...
/// #...##....#
/// .#..#...#.#
/// These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
/// 
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........#.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...##....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
/// You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
/// 
/// The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:
/// 
/// From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
/// 
/// The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:
/// 
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........X.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...#X....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
/// In this example, traversing the map using this slope would cause you to encounter 7 trees.
/// 
/// Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
/// 
/// --- Part Two ---
/// Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.
/// 
/// Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:
/// 
/// Right 1, down 1.
/// Right 3, down 1. (This is the slope you already checked.)
/// Right 5, down 1.
/// Right 7, down 1.
/// Right 1, down 2.
/// In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.
/// 
/// What do you get if you multiply together the number of trees encountered on each of the listed slopes?

extern crate aoc_2020;

use aoc_2020::{util, cylinder::Grid};

pub fn main() {
    let input = util::read_to_string("inputs/d3").unwrap();
    let grid = parse_grid(&input).unwrap();
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut result = 1;
    for (drow, dcol) in slopes.iter() {
        let count = count_trees(&grid, *drow, *dcol);
        println!("Number of tree on slope drow({}), dcol({}): {}", drow, dcol, count);
        result *= count; 
    }
    println!("\nProduct: {}", result);
}

pub fn parse_grid(input: &str) -> Result<Grid<bool>, &'static str> {
    let lines: Vec<&str> = input.trim().split('\n')
        .map(|ln| ln.trim())
        .collect();
    // filter empty rows out?
    let rows = lines.len();
    if rows == 0 {
        return Err("zero rows")
    }
    let cols = lines[0].len();
    let content: Vec<char> = lines.iter()
        .map(|ln| ln.chars().collect::<Vec<char>>())
        .flatten()
        .collect();
    let bool_content: Vec<bool> = content.into_iter()
        .map(|c| if c == '#' { true } else { false })
        .collect();
    println!("XXX rows {} cols {} content len {}", rows, cols, bool_content.len());
    Grid::from_vec(rows, cols, bool_content)
}

/// start at 0,0 on the grid and count trees along the line with slope dcol/drow
pub fn count_trees(grid: &Grid<bool>, drow: usize, dcol: usize) -> usize {
    let mut r = 0;    
    let mut c = 0;    
    let mut count = 0;
    loop {
        // if the match here is successful, we're still on the grid, else we break out of the loop
        if let Some(b) = grid.get(r, c) {
            if *b {
                count += 1;
            }
        } else {
            break;
        }
        r += drow;
        c += dcol;
    }
    count
}

#[cfg(test)]
mod test_d3 {
    use crate::{count_trees, parse_grid};

    const TEST_INPUT: &'static str =
        "..##.......
         #...#...#..
         .#....#..#.
         ..#.#...#.#
         .#...##..#.
         ..#.##.....
         .#.#.#....#
         .#........#
         #.##...#...
         #...##....#
         .#..#...#.#";

    #[test]
    fn test_sample_grid() {
        let grid = parse_grid(TEST_INPUT).unwrap();
        assert_eq!(grid.get(0, 0), Some(&false));
        assert_eq!(grid.get(0, 1), Some(&false));
        assert_eq!(grid.get(0, 2), Some(&true));
        assert_eq!(grid.get(1, 0), Some(&true));
        assert_eq!(grid.get(1, 1), Some(&false));
        
        // test wrap around
        assert_eq!(grid.get(1, 11), Some(&true));
        assert_eq!(grid.get(1, 12), Some(&false));
    }
    
    #[test]
    fn test_count_grid() {
        let grid = parse_grid(TEST_INPUT).unwrap();
        assert_eq!(count_trees(&grid, 1, 3), 7);
        assert_eq!(count_trees(&grid, 1, 1), 2);
        assert_eq!(count_trees(&grid, 1, 0), 3);
    }
}