use std::collections::HashMap;

fn main() {
    let sample_input1 = include_str!("../data/sample1.txt");
    let sample_input2 = include_str!("../data/sample2.txt");
    let input = include_str!("../data/input.txt");

    assert_eq!(part1(sample_input1), 142);
    part1(input);

    assert_eq!(part2(sample_input2), 281);
    part2(input);
}

fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;
    for line in lines {
        total += left_most_number_numeric(line) * 10;
        total += right_most_number_numeric(line);
    }
    println!("{}", total);
    total
}

fn left_most_number_numeric(line: &str) -> u32 {
    for letter in line.chars() {
        if letter.is_numeric() {
            return letter.to_digit(10).unwrap();
        }
    }
    panic!()
}

fn right_most_number_numeric(line: &str) -> u32 {
    return left_most_number_numeric(line.chars().rev().collect::<String>().as_str());
}

fn part2(input: &str) -> u32 {
    let written_number_to_value = gen_dict();

    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;
    for line in lines {
        total += left_most_number(line, &written_number_to_value) * 10;
        total += right_most_number(line, &written_number_to_value);
    }
    println!("{}", total);
    total
}

fn gen_dict() -> HashMap<&'static str, u32> {
    let mut written_number_to_value: HashMap<&str, u32> = HashMap::new();
    let tuples = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for (k, v) in tuples {
        written_number_to_value.insert(k, v);
    }
    written_number_to_value
}

fn left_most_number(line: &str, written_number_to_value: &HashMap<&str, u32>) -> u32 {
    number_helper(line, line.char_indices(), written_number_to_value)
}

fn right_most_number(line: &str, written_number_to_value: &HashMap<&str, u32>) -> u32 {
    number_helper(line, line.char_indices().rev(), written_number_to_value)
}

fn number_helper(
    line: &str,
    iterator: impl Iterator<Item = (usize, char)>,
    written_number_to_value: &HashMap<&str, u32>,
) -> u32 {
    for (i, letter) in iterator {
        if letter.is_numeric() {
            return letter.to_digit(10).unwrap();
        } else {
            for (number, value) in written_number_to_value {
                if line[i..].starts_with(number) {
                    return *value;
                }
            }
        }
    }
    panic!()
}

fn _number_helper_bool2(
    line: &str,
    left_most: bool,
    written_number_to_value: &HashMap<&str, u32>,
) -> u32 {
    let iterator: Box<dyn Iterator<Item = _>> = match left_most {
        true => Box::new(line.char_indices()),
        false => Box::new(line.char_indices().rev()),
    };

    for (i, letter) in iterator {
        if letter.is_numeric() {
            return letter.to_digit(10).unwrap();
        } else {
            for (number, value) in written_number_to_value {
                if line[i..].starts_with(number) {
                    return *value;
                }
            }
        }
    }
    panic!()
}
