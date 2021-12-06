use itertools::Itertools;
use std::collections::HashMap;

//this whole thing can be done in one for loop fiiiix!?!?!

// part = true  == part1 else part2
fn calculate_overlapping_lines(mut lines: Vec<(i32, i32, i32, i32)>, part: bool) -> usize {
    let mut covered: HashMap<(i32, i32), i32> = HashMap::new();


    // (x0,y0,x1,y1)
    for (x0, y0, x1, y1) in lines {
        let range_x = if x1 > x0 {
            (x0..=x1).collect::<Vec<i32>>()
        } else {
            (x1..=x0).rev().collect::<Vec<i32>>()
        };
        let range_y = if y1 > y0 {
            (y0..=y1).collect::<Vec<i32>>()
        } else {
            (y1..=y0).rev().collect::<Vec<i32>>()
        };

        if x0 == x1 {
            // vertical
            for y in range_y {
                *covered.entry((x0, y)).or_insert(0) += 1;
            }
        } else if y0 == y1 {
            // horizontal
            for x in range_x {
                *covered.entry((x, y0)).or_insert(0) += 1;
            }
        } else if !part {
            // diagonal
            let dx = if (x1 - x0) > 0 { 1 } else { -1 };
            let dy = if (y1 - y0) > 0 { 1 } else { -1 };
            let (mut x, mut y) = (x0, y0);

            *covered.entry((x, y)).or_insert(0) += 1;
            while x != x1 {
                x += dx;
                y += dy;
                *covered.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    covered.values().filter(|v| **v > 1).count()
}
fn main() {
    let mut start_to_end_points: Vec<(i32, i32, i32, i32)> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let points = l
                .split(" -> ")
                .flat_map(|p| p.split(","))
                .flat_map(|point| point.parse::<i32>())
                .collect_tuple()
                .unwrap();
            points
        })
        .collect();

    println!(
        "{}",
        calculate_overlapping_lines(
            start_to_end_points
                .clone()
                .into_iter()
                .filter(|tup| tup.0 == tup.2 || tup.1 == tup.3)
                .collect::<Vec<(i32, i32, i32, i32)>>(),
            true
        )
    );
    println!(
        "{}",
        calculate_overlapping_lines(start_to_end_points.clone(), false)
    );
}
