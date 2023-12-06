use std::{collections::HashMap, cmp::max};

fn main() {    
    let sample_input1 = include_str!("../data/sample1.txt");
    let input = include_str!("../data/input.txt");

    assert_eq!(part1(sample_input1), 8);
    part1(input);

    assert_eq!(part2(sample_input1), 2286);
    part2(input);
}

fn gen_limit_dict() -> HashMap<&'static str, u32> {
    let mut written_number_to_value: HashMap<&str, u32> = HashMap::new();
        let tuples = [("red", 12), ("green", 13), ("blue", 14)];

    for (k, v) in tuples {
        written_number_to_value.insert(k, v);
    }
    written_number_to_value
}

fn part1(input: &str) -> u32 {
    let limit_dict = gen_limit_dict();

    let mut total = 0;

    let games: Vec<_> = input.lines().collect();
    for game in games {
        let split = game.split(":").collect::<Vec<_>>();
        let mut valid = true;
        let id = split[0].split(" ").collect::<Vec<_>>()[1].parse::<u32>().unwrap();

        for dice_num in split[1].split(|c| c == ',' || c ==';') {
            let split = dice_num.split(" ").collect::<Vec<_>>();

            let colour = split[2];
            let value = split[1].parse::<u32>().unwrap();

            if limit_dict.get(colour).unwrap() < &value {
                valid = false;
                break;
            }
        }

        if valid {
            total += id;
        }
    };
    println!("{}", total);
    return total;
}

fn gen_max_dict() -> HashMap<&'static str, u32> {
    let mut written_number_to_value: HashMap<&str, u32> = HashMap::new();
        let tuples = [("red", 0), ("green", 0), ("blue", 0)];

    for (k, v) in tuples {
        written_number_to_value.insert(k, v);
    }
    written_number_to_value
}

fn part2(input: &str) -> u32 {
    let max_dict = gen_max_dict();
    let mut total = 0;

    let games: Vec<_> = input.lines().collect();
    for game in games {
        let mut my_max = max_dict.clone();
        let split = game.split(":").collect::<Vec<_>>();

        for dice_num in split[1].split(|c| c == ',' || c ==';') {
            let split = dice_num.split(" ").collect::<Vec<_>>();

            let colour = split[2];
            let value = split[1].parse::<u32>().unwrap();

            my_max.insert(colour, max(value, my_max[colour]));
        }

        let mut game_total = 1;
        for val in my_max.values() {
            game_total *= val;
        }
        total += game_total;

    };
    println!("{}", total);
    return total;
}