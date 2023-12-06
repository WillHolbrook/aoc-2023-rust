fn main() {    
    let sample_input1 = include_str!("../data/sample1.txt");
    let sample_input2 = include_str!("../data/sample2.txt");
    let input = include_str!("../data/input.txt");



    assert_eq!(part1(sample_input1), 8);
    part1(input);

    assert_eq!(part2(sample_input2), 281);
    part2(input);
}

fn part1(input: &str) -> u32 {
    let games: Vec<_> = input.lines().collect();
    for game in games {
        first, second = game.split(":");

    };
    return 0;
}

fn part2(input: &str) -> u32 {
    return 0;
}