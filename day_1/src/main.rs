fn count_measurements_pt1(measurements: &Vec<usize>) -> usize {
    let mut total: usize = 0;
    for (i, measurement) in measurements.iter().enumerate() {
        total += match i {
            0 => 0,
            _ => {
                if measurement > &measurements[i - 1] {1} else {0}
            }
        }
    }
    total
}

fn count_measurements_pt2(measurements: &Vec<usize>) -> usize {
    let mut total: usize = 0;
    let mut previous: usize = measurements[0..3].iter().fold(0usize, |sum, m| sum + m);

    for window in measurements.windows(3) {
        let current = window.iter().sum();
        if current > previous {
            total += 1;
        } 
        previous = current;
    }
    total
}

fn main() {
    let contents = include_str!("../input.txt");

    let parsed_contents: Vec<usize> = contents
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    println!("part1: {}", count_measurements_pt1(&parsed_contents));
    println!("part2: {}", count_measurements_pt2(&parsed_contents));
}
