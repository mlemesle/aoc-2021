use lib;

fn distance(crab_position: &usize, destination: usize) -> usize {
    if *crab_position > destination {
        *crab_position - destination
    } else {
        destination - *crab_position
    }
}

fn compute_min_fuel(f: fn(Vec<usize>, usize) -> usize) -> usize {
    let crab_positions = lib::input::<String>("input.txt").collect::<Vec<String>>()[0]
        .split(",")
        .into_iter()
        .map(|s| usize::from_str_radix(s, 10).expect(&format!("Error while parsing {}", s)))
        .collect::<Vec<usize>>();
    let (min_crab_pos, max_crab_pos) =
        crab_positions
            .iter()
            .fold((usize::MAX, usize::MIN), |mut positions, &current_crab| {
                if positions.0 > current_crab {
                    positions.0 = current_crab;
                }
                if positions.1 < current_crab {
                    positions.1 = current_crab;
                }
                positions
            });

    (min_crab_pos..max_crab_pos)
        .into_iter()
        .map(|current_pos| f(crab_positions.clone(), current_pos))
        .min()
        .expect("Something went wrong while computing fuel consumption")
}

fn part1() {
    let min_fuel = compute_min_fuel(|crab_positions, current_pos| {
        crab_positions
            .iter()
            .map(|crab_position| distance(crab_position, current_pos))
            .sum::<usize>()
    });

    println!("Minimum fuel consumption is {}", min_fuel);
}

fn part2() {
    let min_fuel = compute_min_fuel(|crab_positions, current_pos| {
        crab_positions
            .iter()
            .map(|crab_position| (1..=distance(crab_position, current_pos)).sum::<usize>())
            .sum::<usize>()
    });

    println!("Minimum fuel consumption is {}", min_fuel);
}

fn main() {
    part1();
    part2();
}
