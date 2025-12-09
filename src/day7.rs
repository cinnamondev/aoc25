use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::thread::sleep;
use std::time::Duration;

struct Beam {
    start: usize,
    end: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum Node {
    EMPTY,
    EMITTER,
    SPLITTER
}
impl TryFrom<char> for Node {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Node::EMPTY),
            '^' => Ok(Node::SPLITTER),
            'S' => Ok(Node::EMITTER),
            _ => Err(()),
        }
    }
}

pub fn run() {
    let str = read_to_string("input/7").unwrap();

    let mut grid = str.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
    let starting_x = grid[0].iter().enumerate().find(|(i, node)| **node == 'S').unwrap().0;
    grid[1][starting_x] = '|';
    let mut dimensions_per_column: Vec<u64> = (0..grid[0].len()).map(|i| 0).collect();
    let mut total_splits = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' && grid[i-1][j] == '|' {
                // we only need to see how many dimensions are in this column and add it to the l/r. min add is 1.
                let n_dimensions = dimensions_per_column[j];
                dimensions_per_column[j] = 0;
                grid[i][j] = '#';
                let lhs = draw_beams(&mut grid, j-1, i) as u32;
                dimensions_per_column[j-1] += n_dimensions.max(1);
                let rhs = draw_beams(&mut grid, j+1, i) as u32;
                dimensions_per_column[j+1] += n_dimensions.max(1);
                total_splits += 1;
                if lhs != 0 || rhs != 0 {
                    print_grid(&grid, &dimensions_per_column, total_splits);
                }
            }
        }
    }
    //println!("{}", total_splits);
    //println!("{}", dimensions_per_column.iter().sum::<u64>());
}

fn print_grid(vec: &Vec<Vec<char>>, dimensions_counter: &Vec<u64>, total_splits: u32) {
    print!("\x1Bc");
    println!("{}\n Total Dimensions: {} \n Total Splits: {}",
             vec.iter().map(|l| l.iter().collect::<String>() + "\n").collect::<String>(),
        //dimensions_counter,
        dimensions_counter.iter().sum::<u64>(),
        total_splits
    );
    sleep(Duration::from_millis(2));
}
fn draw_beams(vec: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if vec[y][x] == '|' { return false; }

    for j in y..vec.len() {
        if vec[j][x] == '^' { break; }
        vec[j][x] = '|';
    }

    true
}