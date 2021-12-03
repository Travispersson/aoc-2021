use std::str;

fn count_ones(i: usize, contents: &Vec<&str>) -> usize {
    let mut ones = 0;
    for l in contents.iter() {
        if l.chars().nth(i).unwrap() == '1' {
            ones += 1
        }
    }
    ones
}
fn count_most_common_bits(contents: &Vec<&str>) -> Vec<usize> {
    let width = contents[0].len();
    let mut most_common_bits: Vec<usize> = Vec::with_capacity(width);
    for i in 0..width {
        let ones = count_ones(i, &contents);
        let zeros = contents.len() - ones;
        if ones > zeros {
            most_common_bits.push(1)
        } else {
            most_common_bits.push(0)
        }
    }

    most_common_bits
}

fn power_consumption(contents: Vec<&str>) -> usize {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for b in count_most_common_bits(&contents) {
        if b == 1 {
            gamma.push('1');
            epsilon.push('0')
        } else {
            gamma.push('0');
            epsilon.push('1')
        }
    }

    usize::from_str_radix(gamma.as_str(), 2).unwrap()
        * usize::from_str_radix(epsilon.as_str(), 2).unwrap()
}

fn find_target<T>(contents: &Vec<&str>, pred: T) -> usize
where
    T: Fn(usize, usize) -> usize,
{
    let mut content = contents.clone();
    let width = content[0].len();

    for i in 0usize..width {
        let ones = count_ones(i, &content);
        let zeros = content.len() - ones;
        let to_match = pred(zeros, ones);

        content = content
            .into_iter()
            .filter(|num| (num.chars().nth(i).unwrap().to_digit(10).unwrap() as usize) == to_match)
            .collect();

        if content.len() == 1 {
            break;
        }
    }
    return usize::from_str_radix(content.first().unwrap(), 2).unwrap();
}

fn life_support_rating(contents: Vec<&str>) -> usize {
    find_target(&contents, |zeros, ones| if ones >= zeros { 1 } else { 0 })
        * find_target(&contents, |zeros, ones| if ones >= zeros { 0 } else { 1 })
}

fn main() {
    let contents: Vec<&str> = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    println!("{:?}", power_consumption(contents.clone()));
    println!("{:?}", life_support_rating(contents));
}
