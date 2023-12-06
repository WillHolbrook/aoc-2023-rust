use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    usize,
};

fn main() {
    let sample_input1 = include_str!("../data/sample1.txt");
    let input = include_str!("../data/input.txt");

    assert_eq!(part1(sample_input1), 4361);
    part1(input);

    assert_eq!(part2(sample_input1), 467835);
    part2(input);
}

fn part1(input: &str) -> u32 {
    let char_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    let is_neighboring_symbol = generate_neighboring_symbol_func(&char_grid);
    for (i, row) in char_grid.iter().enumerate() {
        let mut current_num_str: String = "".to_string();
        let mut num_adjecent_to_symbol = false;
        for (j, current_char) in row.iter().enumerate() {
            if current_char.is_numeric() {
                current_num_str += &current_char.to_string();

                if is_neighboring_symbol((i, j)) {
                    num_adjecent_to_symbol = true;
                }
            } else {
                if num_adjecent_to_symbol {
                    total += current_num_str.parse::<u32>().unwrap();
                }
                current_num_str = "".to_string();
                num_adjecent_to_symbol = false
            }
        }
        if num_adjecent_to_symbol {
            total += current_num_str.parse::<u32>().unwrap();
        }
    }

    println!("{}", total);
    total
}

fn generate_neighboring_symbol_func(
    char_grid: &Vec<Vec<char>>,
) -> impl Fn((usize, usize)) -> bool + '_ {
    let width = char_grid[0].len();
    let height = char_grid.len();

    move |(i, j): (usize, usize)| -> bool {
        for test_row in char_grid.iter().take(min(i + 1, height - 1) + 1).skip(max(i.saturating_sub(1), 0)) {
            for my_j in max(j.saturating_sub(1), 0)..=min(j + 1, width - 1) {
                if !test_row[my_j].is_numeric() && test_row[my_j] != '.' {
                    return true;
                }
            }
        }
        false
    }
}

fn part2(input: &str) -> u32 {
    let char_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let get_neighboring_gears = generate_neighboring_gear_func(&char_grid);
    let mut gear_to_num_set: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (i, row) in char_grid.iter().enumerate() {
        let mut gear_set: HashSet<(usize, usize)> = HashSet::new();
        let mut current_num_str: String = "".to_string();

        for (j, current_char) in row.iter().enumerate() {
            if current_char.is_numeric() {
                current_num_str += &current_char.to_string();

                for gear in get_neighboring_gears((i, j)) {
                    gear_set.insert(gear);
                }
            } else {
                if !gear_set.is_empty() {
                    let current_num = current_num_str.parse::<u32>().unwrap();
                    for gear in gear_set.iter() {
                        let opt_list = gear_to_num_set.get_mut(gear);
                        if let Some(list) = opt_list {
                            list.push(current_num)
                        } else {
                            gear_to_num_set.insert(*gear, vec![current_num]);
                        }
                    }
                }
                gear_set = HashSet::new();
                current_num_str = "".to_string();
            }
        }
        if !gear_set.is_empty() {
            let current_num = current_num_str.parse::<u32>().unwrap();
            for gear in gear_set.iter() {
                let opt_list = gear_to_num_set.get_mut(gear);
                if let Some(list) = opt_list {
                    list.push(current_num)
                } else {
                    gear_to_num_set.insert(*gear, vec![current_num]);
                }
            }
        }
    }

    let gear_to_num_set = gear_to_num_set
        .into_iter()
        .filter(|(_, list)| list.len() == 2)
        .map(|(_, list)| list.into_iter().product::<u32>())
        .sum();

    println!("{}", gear_to_num_set);
    gear_to_num_set
}

fn generate_neighboring_gear_func(
    char_grid: &Vec<Vec<char>>,
) -> impl Fn((usize, usize)) -> Vec<(usize, usize)> + '_ {
    let width = char_grid[0].len();
    let height = char_grid.len();

    move |(i, j): (usize, usize)| -> Vec<(usize, usize)> {
        let mut star_locations = Vec::new();

        for (my_i, row) in char_grid.iter().enumerate().take(min(i + 1, height - 1) + 1).skip(max(i.saturating_sub(1), 0)) {
            for my_j in max(j.saturating_sub(1), 0)..=min(j + 1, width - 1) {
                if row[my_j] == '*' {
                    star_locations.push((my_i, my_j));
                }
            }
        }
        star_locations
    }
}
