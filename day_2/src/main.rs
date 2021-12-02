fn calculate_position_pt1(contents: Vec<(&str, usize)>) -> usize {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;

    for (dir, amt) in contents {
        match dir {
            "forward" => horizontal += amt,
            "down" => depth += amt,
            _ => depth -= amt,
        }
    }
    return horizontal * depth;
}

fn calculate_position_pt2(contents: Vec<(&str, usize)>) -> usize {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    for (dir, amt) in contents {
        match dir {
            "forward" => {
                horizontal += amt;
                depth += amt * aim
            }
            "down" => {
                aim += amt
            }
            _ => {
                aim -= amt
            }
        }
    }

    return horizontal * depth;
}
fn main() {
    let contents: Vec<(&str, usize)> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let line_parts = l.split(' ').collect::<Vec<&str>>();
            return (
                line_parts.first().unwrap().clone(),
                line_parts[1].parse::<usize>().unwrap(),
            );
        })
        .collect::<Vec<(&str, usize)>>();

    println!("{:?}", calculate_position_pt1(contents.clone()));
    println!("{:?}", calculate_position_pt2(contents));
}

#[test]
fn feature() {
    let contents = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".lines()
    .map(|l| {
        let line_parts = l.split(' ').collect::<Vec<&str>>();
        return (
            line_parts.first().unwrap().clone(),
            line_parts[1].parse::<usize>().unwrap(),
        );
    })
    .collect::<Vec<(&str, usize)>>();
    println!("{:?}", calculate_position_pt2(contents));

    
}