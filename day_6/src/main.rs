/*
Each day, a 0 becomes a 6 and adds a new 8 to the end of the list,
while each other number decreases by 1 if it was present at the start of the day.
*/

fn step(mut store: [usize; 9]) -> [usize; 9] {
    match store[..] {
        [a, b, c, d, e, f, g, h, i] => [b, c, d, e, f, g, h + a, i, a],
        _ => panic!("panic at the disco"),
    }
}

fn simulation(days: usize, mut store: [usize; 9]) -> usize {
    for _ in 0..days {
        store = step(store);
    }
    store.iter().sum()
}

fn main() {
    let mut initial_state: Vec<i32> = include_str!("../input.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut store: [usize; 9] = [0; 9];
    initial_state.iter().for_each(|f| store[*f as usize] += 1);

    println!("p1: {}", simulation(80, store.clone()));
    println!("p2: {}", simulation(256, store))
}
