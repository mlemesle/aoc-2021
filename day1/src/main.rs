use lib;

fn solve_part(levels_to_process: Vec<u32>) {
    let count = levels_to_process
        .windows(2)
        .filter(|&levels| levels[0] < levels[1])
        .count();
    println!("There are {} level increases", count);
}

fn part1() {
    let levels_to_process = lib::input::<u32>("input.txt").collect::<Vec<u32>>();
    solve_part(levels_to_process);
}

fn part2() {
    let levels_to_process = lib::input::<u32>("input.txt")
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|levels| levels.iter().sum())
        .collect::<Vec<u32>>();
    solve_part(levels_to_process);
}

fn main() {
    part1();
    part2();
}
