use std::collections::HashMap;

fn main() {
    let sample_input1 = include_str!("../data/sample1.txt");
    let sample_input2 = include_str!("../data/sample2.txt");
    let input = include_str!("../data/input.txt");

    part1(sample_input1);
    part1(input);

    part2(sample_input2);
    part2(input);
}

fn part1(input: &str) {
    let lines : Vec<&str> = input.lines().collect();
    let mut total = 0;
    for line in lines {
        total += left_most_number_numeric(line) * 10;
        total += right_most_number_numeric(line);
    }
    println!("{}", total);
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

fn part2(input: &str) {
    let mut written_number_to_value: HashMap<&str, u32> = HashMap::new();
    // written_number_to_value.insert("zero", 0);
    written_number_to_value.insert("one", 1);
    written_number_to_value.insert("two", 2);
    written_number_to_value.insert("three", 3);
    written_number_to_value.insert("four", 4);
    written_number_to_value.insert("five", 5);
    written_number_to_value.insert("six", 6);
    written_number_to_value.insert("seven", 7);
    written_number_to_value.insert("eight", 8);
    written_number_to_value.insert("nine", 9);

    let written_number_to_value = &written_number_to_value;

    let lines : Vec<&str> = input.lines().collect();
    let mut total = 0;
    for line in lines {
        total += left_most_number(line, written_number_to_value) * 10;
        total += right_most_number(line, written_number_to_value);
    }
    println!("{}", total);
}

fn left_most_number(line: &str, written_number_to_value: &HashMap<&str, u32>) -> u32 {
    for (i, letter) in line.char_indices() {
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

fn right_most_number(line: &str, written_number_to_value: &HashMap<&str, u32>) -> u32 {
    for (i, letter) in line.char_indices().rev() {
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

// fn number_helper(line: &str, left_most: bool, written_number_to_value: &HashMap<&str, u32>) -> u32 {
//     let iterator: std::iter::Rev<std::str::CharIndices<'_>>= match left_most {
//         true => line.char_indices().into_iter(),
//         false => line.char_indices().rev().into_iter(),
//     };
//     let a = line.char_indices().rev().into_iter();
//     for (i, letter) in iterator {
//         if letter.is_numeric() {
//             return letter.to_digit(10).unwrap();
//         } else {
//             for (number, value) in written_number_to_value {
//                 if line[i..].starts_with(number) {
//                     return *value;
//                 }
//             }
//         }
//     }
//     panic!()
// }