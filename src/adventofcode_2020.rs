use std::fs;

use ndarray::{Array2, Axis, ArrayView, Dim};

use regex::Regex;
use std::collections::HashMap;


fn read_day01_input() -> Vec<i32> {
    // read day 01 input from file
    let day01_input_str = fs::read_to_string("src/day01.txt")
        .expect("Something went wrong reading the file");

    let mut day01_input = Vec::new();

    let lines = day01_input_str.split("\r\n");
    for line in lines {
        let value = line.parse::<i32>().unwrap();
        day01_input.push(value);
    }

    day01_input
}

pub fn day01() {
    println!("\n2020 - Day 01");
    println!("Task: Find the two entries that sum to 2020 \
              and then multiply those two numbers together.");

    let input = read_day01_input();

    let input_len = input.len();
    for i1 in 0..input_len {
        let v1 = input[i1];
        for i2 in i1 + 1..input_len {
            let v2 = input[i2];
            if v1 + v2 == 2020 {
                println!("{} + {} = {} → {} x {} = {}", v1, v2, v1 + v2, v1, v2, v1 * v2);
            }
        }
    }

    println!("Task: Repeat the task of day 01, but now three \
             instead of two values must add up to 2020 \
             (e.g., 979 + 366 + 675 = 2020 → 979 * 366 * 675 = 241861950).");

    for i1 in 0..input_len {
        let v1 = input[i1];
        for i2 in i1 + 1..input_len {
            let v2 = input[i2];
            for i3 in i2..input_len {
                let v3 = input[i3];
                if v1 + v2 + v3 == 2020 {
                    println!("{v1} + {v2} + {v3} = {sum} → {v1} x {v2} x {v3} = {product}",
                             v1 = v1, v2 = v2, v3 = v3, sum = v1 + v2 + v3, product = v1 * v2 * v3);
                }
            }
        }
    }
}

pub fn day02() {
    println!("\n2020 - Day 02");

    // read day 02 input from file
    let input_str = fs::read_to_string("src/day02.txt")
        .expect("Something went wrong reading the file");

    // regex to parse one line of the day 02 input
    let re = Regex::new(r"(?m)(?P<num1>[0-9]*)-(?P<num2>[0-9]*) (?P<char>[a-z]): (?P<password>[a-z]*)").unwrap();

    let mut total = 0;
    let mut task1_counter = 0;
    let mut task2_counter = 0;

    let lines = input_str.split("\r\n");
    for line in lines {
        total += 1;
        match re.captures(line) {
            Some(x) => {
                let num1 = x.name("num1").map_or(-1, |m| m.as_str().parse::<i32>().unwrap());
                let num2 = x.name("num2").map_or(-1, |m| m.as_str().parse::<i32>().unwrap());
                let char = x.name("char").map_or("", |m| m.as_str());
                let password = String::from(x.name("password").map_or("", |m| m.as_str()));

                let occurrences = password.matches(char).count() as i32;
                let task1_valid = (occurrences >= num1) && (occurrences <= num2);
                if task1_valid {
                    task1_counter += 1;
                }

                let task2_valid: bool;
                let password_len = password.len() as i32;
                if (num1 > password_len) || (num2 > password_len) {
                    panic!()
                } else {
                    let i1 = (num1 - 1) as usize;
                    let i2 = (num1) as usize;
                    let pos1_char = &password[i1..i2];

                    let i1 = (num2 - 1) as usize;
                    let i2 = (num2) as usize;
                    let pos2_char = &password[i1..i2];

                    task2_valid = ((pos1_char == char) && (pos2_char != char)) ||
                        (pos1_char != char) && (pos2_char == char);
                }

                if task2_valid {
                    task2_counter += 1;
                }
            }
            None => unreachable!()
        }
    }

    println!("Task 1: {} passwords of {} are valid.", task1_counter, total);
    println!("Task 2: {} passwords of {} are valid.", task2_counter, total);
}

fn day03_tree_finder(input_str: &String, x_inc: i32, y_inc: i32) -> i32 {
    let mut width = 0;
    for char in input_str.chars() {
        if char == '\r' || char == '\n' {
            break;
        }
        width += 1;
    }

    let mut x = 0;

    let lines = input_str.split("\r\n");
    let mut y = 0;
    let mut y_next = y + y_inc;

    let mut tree_counter = 0;
    for line in lines {
        if y != y_next {
            // skip the row
            y += 1;
            continue;
        }

        y_next = y + y_inc;
        y += 1;

        x += x_inc;
        x = x % width;

        let x1 = x as usize;
        let x2 = (x + 1) as usize;
        let char = &line[x1..x2];
        if char == "#" {
            tree_counter += 1;
        }
    }

    tree_counter
}

pub fn day03() {
    println!("\n2020 - Day 03");

    // read day 03 input from file
    let input_str = fs::read_to_string("src/day03.txt")
        .expect("Something went wrong reading the file");

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut tree_mult = 1i64;
    for slope in slopes.iter() {
        let trees = day03_tree_finder(&input_str, slope.0, slope.1);
        tree_mult *= trees as i64;
        println!("{} trees", trees);
    }

    println!("trees multiplied = {}", tree_mult);
}

pub fn day04() {
    println!("\n2020 - Day 04");
}

fn get_edges(tile: &Array2<i8>) -> [ArrayView<i8, Dim<[usize; 1]>>; 8] {
    let s1 = tile.slice(s![0, ..]);
    let s2 = tile.slice(s![9, ..]);
    let s3 = tile.slice(s![.., 0]);
    let s4 = tile.slice(s![.., 9]);

    let mut s5 = tile.slice(s![0, ..]);
    s5.invert_axis(Axis(0));

    let mut s6 = tile.slice(s![9, ..]);
    s6.invert_axis(Axis(0));

    let mut s7 = tile.slice(s![.., 0]);
    s7.invert_axis(Axis(0));

    let mut s8 = tile.slice(s![.., 9]);
    s8.invert_axis(Axis(0));

    let tile_edges = [s1, s2, s3, s4, s5, s6, s7, s8];
    tile_edges
}

fn compare_edges(edges1: [ArrayView<i8, Dim<[usize; 1]>>; 8], edges2: [ArrayView<i8, Dim<[usize; 1]>>; 8]) -> (bool, usize) {
    for (i, e1) in edges1.iter().enumerate() {
        for (j, e2) in edges2.iter().enumerate() {
            if e1.eq(&e2) {
                return (true, i);
            }
        }
    }

    return (false, 9);
}

pub fn day20() {
    println!("\n2020 - Day 20");

    let tile_size = 10;

    // load and parse tiles
    let input_str = fs::read_to_string("src/day20.txt")
        .expect("Something went wrong reading the file");

    let mut tiles = Vec::new();
    let mut tile_ids = Vec::new();

    let lines = input_str.split("\r\n");
    let mut line_counter = 0;
    for line in lines {
        if line == "\r\n" {
            line_counter += 1;
            continue;
        }

        if line_counter % 12 == 0 {
            let mut tile = Array2::<i8>::zeros((tile_size, tile_size));
            tile[[0, 0]] = 10i8;
            tiles.push(tile);

            let tile_id = line[5..9].parse::<i32>().unwrap();
            tile_ids.push(tile_id);
            line_counter += 1;
            continue;
        }

        let tiles_len = tiles.len() - 1;
        let tile = &mut tiles[tiles_len];

        let mut x = 0;
        for char in line.chars() {
            let mut v = 0i8;
            if char == '.' {
                v = 0i8;
            } else if char == '#' {
                v = 1i8;
            }

            let y = (line_counter % 12) - 1;
            tile[[y, x]] = v;
            x += 1;
        }

        line_counter += 1;
    }

    println!("Tile IDs:\n{:?}", tile_ids);

    let map_size = tiles.len() as f64;
    let map_size = map_size.sqrt() as usize;
    println!("Number of tiles: {} → map size: {size} x {size}", tiles.len(), size = map_size);

    let mut compatible_tiles: HashMap<i32, Vec<(usize, i32, usize)>> = HashMap::new();
    let mut corner_tile_ids = Vec::new();

    let mut mult_tile_ids = 1;
    for (i, tile1) in tiles.iter().enumerate() {
        let tile1_id = tile_ids[i];
        let edges1 = get_edges(&tile1);
        let mut comp_tiles = Vec::new();

        for (j, tile2) in tiles.iter().enumerate() {
            let tile2_id = tile_ids[j];
            if tile1_id == tile2_id {
                continue;
            }

            let edges2 = get_edges(&tile2);
            let (matching, e1) = compare_edges(edges1, edges2);
            if matching {
                comp_tiles.push((j, tile2_id, e1));
            }
        }

        if comp_tiles.len() == 2 {
            mult_tile_ids *= tile1_id as i64;
            println!("Tile {} has only two compatible tiles ({:?}).", tile1_id, comp_tiles);
            corner_tile_ids.push(tile1_id);
        }

        compatible_tiles.insert(tile1_id, comp_tiles);
    }

    println!("Corner tiles: {:?}. Multiplied tile IDs: {}", corner_tile_ids, mult_tile_ids);

    println!("{:?}", compatible_tiles);


    // initialize image of tiles
    let mut tile_map = Array2::<i32>::zeros((map_size, map_size));

    // pick first corner tile as the starting point
    let first_corner_id = corner_tile_ids.get(0).unwrap();
    let first_corner = compatible_tiles.get(first_corner_id).unwrap();

    if (first_corner[0].2 == 0 && first_corner[1].2 == 2) ||
        (first_corner[0].2 == 2 && first_corner[1].2 == 0) {
        // top left
        tile_map[[0, 0]] = *first_corner_id;
    } else if (first_corner[0].2 == 0 && first_corner[1].2 == 3) ||
        (first_corner[0].2 == 3 && first_corner[1].2 == 0) {
        // top right
        tile_map[[0, map_size-1]] = *first_corner_id;
    } else if (first_corner[0].2 == 1 && first_corner[1].2 == 3) ||
        (first_corner[0].2 == 3 && first_corner[1].2 == 1) {
        // bottom right
        tile_map[[map_size-1, map_size-1]] = *first_corner_id;
    } else if (first_corner[0].2 == 1 && first_corner[1].2 == 2)
        || (first_corner[0].2 == 2 && first_corner[1].2 == 1) {
        // bottom left
        tile_map[[0, map_size-1]] = *first_corner_id;
    }
}