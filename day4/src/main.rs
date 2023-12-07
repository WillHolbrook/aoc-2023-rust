use std::collections::HashSet;

use regex::Regex;

fn main() {
    let sample_input1 = include_str!("../data/sample1.txt");
    let input = include_str!("../data/input.txt");

    assert_eq!(part1(sample_input1), 13);
    part1(input);

    assert_eq!(part2(sample_input1), 30);
    part2(input);
}

fn part1(input: &str) -> u32 {
    let pattern = Regex::new(r"^.*?: (.*) \| (.*)$").unwrap();

    let winnings = input
        .lines()
        .map(|row| pattern.captures(row).unwrap())
        .map(|capture| {
            (
                capture.get(1).unwrap().as_str(),
                capture.get(2).unwrap().as_str(),
            )
        })
        .map(|(wins, guesses)| {
            (
                wins.split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<HashSet<_>>(),
                guesses
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<HashSet<_>>(),
            )
        })
        .map(|(wins, guesses)| wins.intersection(&guesses).cloned().collect::<HashSet<_>>())
        .map(|combined_set| {
            if combined_set.is_empty() {
                0
            } else {
                2_u32.pow(combined_set.len() as u32 - 1)
            }
        })
        .sum::<u32>();

    println!("{:?}", winnings);

    winnings
}

fn part2(input: &str) -> u32 {
    let pattern = Regex::new(r"^.*?: (.*) \| (.*)$").unwrap();

    let cards = input
        .lines()
        .map(|row| pattern.captures(row).unwrap())
        .map(|capture| {
            (
                capture.get(1).unwrap().as_str(),
                capture.get(2).unwrap().as_str(),
            )
        })
        .map(|(wins, guesses)| {
            (
                wins.split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<HashSet<_>>(),
                guesses
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<HashSet<_>>(),
            )
        })
        .map(|(wins, guesses)| wins.intersection(&guesses).cloned().collect::<HashSet<_>>())
        .map(|combined_set| combined_set.len())
        .collect::<Vec<_>>();

    let mut num_of_cards: Vec<u32> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        for j in 1..=*card {
            num_of_cards[i + j] += num_of_cards[i];
        }
    }
    let total_num_of_cards = num_of_cards.iter().sum();

    println!("{:?}", total_num_of_cards);

    total_num_of_cards
}
