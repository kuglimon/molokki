fn line_to_number(line: &str) -> u32 {
    let mut numbers = line.chars().filter(|c| c.is_digit(10));
    let start = numbers.nth(0).unwrap_or('0');
    let end = numbers.nth_back(0).unwrap_or(start);

    start.to_digit(10).unwrap() * 10 + end.to_digit(10).unwrap()
}

pub fn solve() {
    println!("Solving day 1 puzzles");

    let input = String::from_utf8_lossy(include_bytes!("inputs/day_1.txt"));
    let result: u32 = input.lines().map(line_to_number).sum();

    println!("  1. result: {}", result);
}
