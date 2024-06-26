use sc2pathlib::mapping::map::Map;
use sc2pathlib::path_find;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn rot90(vec: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let new_height = vec.len();
    let new_width = vec[0].len();
    let mut new_arr: Vec<Vec<usize>> = vec![vec![0; new_height]; new_width];
    // Traverse each cycle
    for i in 0..new_width {
        for j in 0..new_height {
            new_arr[i][j] = vec[new_height - 1 - j][i];
        }
    }
    new_arr
}

pub fn read_vec_from_file(file_path: &str) -> Vec<Vec<usize>> {
    let f = BufReader::new(File::open(file_path).unwrap());
    let mut arr = Vec::<Vec<usize>>::new();

    for line in f.lines().map(|x| x.unwrap()) {
        let mut maze_line = vec![];
        for char in line.chars() {
            if !char.is_digit(10) {
                break;
            }
            let value = char.to_digit(10).unwrap() as usize;
            maze_line.push(value)
        }

        arr.push(maze_line);
    }
    rot90(arr)
}

pub fn get_pathfind(file: &str) -> path_find::PathFind {
    let map = read_vec_from_file(file);
    path_find::PathFind::new_internal(map)
}

pub fn get_choke_map() -> Map {
    let grid = read_vec_from_file("tests/choke.txt");
    let grid2 = read_vec_from_file("tests/choke.txt");
    let grid_height = read_vec_from_file("tests/choke_height.txt");
    let reaper_overrides: Vec<Vec<usize>> = Vec::new();
    let map = Map::new(grid, grid2, grid_height, 2, 2, 38, 38, reaper_overrides);
    map
}
