use std::fs::read_to_string;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    let file = read_to_string("./input/4").unwrap();

    let mut visualizer: Vec<Vec<char>> = file.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut map = file.lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let mut part1 = 0;

    for i in 0..map.iter().len() {
        for j in 0..map[i].len() {
            if map[i][j] {
                if visit_neighbours(&map, j,i) < 4 {
                    part1 += 1;
                    visualizer[i][j] = '#';
                }
            }
            // visit neighbours
        }
    }

    let mut part2 = 0;
    let mut removed_neighbours = u64::MAX;
    while removed_neighbours != 0 {
        removed_neighbours = 0;
        for i in 0..map.iter().len() {
            for j in 0..map[i].len() {
                if map[i][j] {
                    if visit_neighbours(&map, j,i) < 4 {
                        removed_neighbours += 1;
                        visualizer[i][j] = '#';
                        map[i][j] = false;
                    }
                }
                // visit neighbours
            }
            print_visualizer(&visualizer);
            sleep(Duration::from_millis(1));
            print!("\x1Bc");
            visualizer = visualizer.iter().map(|l| l.iter().map(|c| {
                if *c == '#' {
                    '.'
                } else { *c }
            }).collect()).collect();
        }
        part2 += removed_neighbours;
    }
    
    print_visualizer(&visualizer);
    println!("Part 1 ANS: {}", part1);
    println!("Part 2 ANS: {}", part2);

}

fn print_visualizer(visualizer: &Vec<Vec<char>>) {
    println!("{}", visualizer.iter().map(|v| v.iter().collect::<String>() + "\n").collect::<String>());
}
fn visit_neighbours(vec: &Vec<Vec<bool>>, cur_x: usize, cur_y: usize) -> usize {
    let mut counter = 0;

    for i in (cur_y.saturating_sub(1))..(cur_y+2).min(vec.len()) {
        for j in (cur_x.saturating_sub(1))..(cur_x+2).min(vec[i].len()) {
            if j == cur_x && i == cur_y { continue; }
            if vec[i][j] {
                counter += 1;
            }
        }
    }

    counter
}