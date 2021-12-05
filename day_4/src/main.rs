use std::fmt;

#[derive(Debug, Clone, Copy)]
struct Board {
    cells: [(usize, bool); 25],
    winning: bool,
}

const LENGTH: usize = 5;

impl Board {
    pub fn new(nums: &[usize]) -> Self {
        let mut cells = [(0, false); 25];
        nums.iter().enumerate().for_each(|(i, n)| cells[i].0 = *n);

        Board {
            cells,
            winning: false,
        }
    }

    fn get_index(&self, num: &usize) -> Option<usize> {
        self.cells.iter().position(|(n, _)| num == n)
    }

    pub fn mark_cell(&mut self, num: &usize) {
        if let Some(index) = self.get_index(num) {
            self.cells[index].1 = true;
            self.winning = self.winning
                || self.check_winning_mark_row(index / LENGTH)
                || self.check_winning_mark_col(index % LENGTH);
        }
    }
    fn check_winning_mark_row(&self, i: usize) -> bool {
        let mut marked: bool = true;
        for j in 0..LENGTH {
            marked = marked && self.cells[i * LENGTH + j].1
        }
        marked
    }

    fn check_winning_mark_col(&self, i: usize) -> bool {
        let mut marked: bool = true;
        for j in 0..LENGTH {
            marked = marked && self.cells[j * LENGTH + i].1;
        }
        marked
    }

    pub fn get_winning(&self) -> bool {
        self.winning
    }

    pub fn get_unmarked(&self) -> Vec<usize> {
        let mut unmarked = vec![];
        self.cells
            .iter()
            .filter(|(n, m)| !m)
            .for_each(|c| unmarked.push(c.0));
        unmarked
    }
}

fn parse_input(numbers: &str, boards: &str) -> (Vec<usize>, Vec<Board>) {
    let chosen_numbers: Vec<usize> = numbers
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|row| row.split_whitespace().map(|n| n.parse::<usize>().unwrap()))
        .flatten()
        .collect::<Vec<usize>>()
        .chunks(25)
        .map_while(|chunk| match chunk {
            v if v.len() == 25 => Some(Board::new(v)),
            _ => None,
        })
        .collect();

    (chosen_numbers, boards)
}

fn score_p1(chosen_numbers: &Vec<usize>, mut boards: Vec<Board>) {
    for chosen_num in chosen_numbers.iter() {
        for board in boards.iter_mut() {
            board.mark_cell(chosen_num);
            if board.get_winning() {
                println!(
                    "{}",
                    board.get_unmarked().iter().sum::<usize>() * chosen_num
                );
                return;
            }
        }
    }
}

fn score_p2(chosen_numbers: &Vec<usize>, mut boards: Vec<Board>) {
    for chosen_num in chosen_numbers.iter() {
        for board in boards.iter_mut() {
            board.mark_cell(chosen_num);
        }
        if boards.len() == 1 && boards.first().unwrap().get_winning() {
            println!(
                "{}",
                boards.first().unwrap().get_unmarked().iter().sum::<usize>() * chosen_num
            );
        }

        boards.retain(|b| !b.get_winning())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..LENGTH {
            for j in 0..LENGTH {
                let b = self.cells[j + LENGTH * i];
                if b.0 > 9 {
                    write!(f, "{} ", b.0);
                } else {
                    write!(f, "{}  ", b.0);
                }
            }
            write!(f, "\n");
        }
        write!(f, "\nWinning: {}\n", self.get_winning())
    }
}

fn main() {
    let (numbers, boards) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let (chosen_numbers, mut boards) = parse_input(numbers, boards);

    score_p1(&chosen_numbers, boards.clone());
    score_p2(&chosen_numbers, boards.clone());
}

#[test]
fn test_input_a() {
    let (numbers, boards) =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
    \n\
    22 13 17 11  0\n\
     8  2 23  4 24\n\
    21  9 14 16  7\n\
     6 10  3 18  5\n\
     1 12 20 15 19\n\
    \n\
     3 15  0  2 22\n\
     9 18 13 17  5\n\
    19  8  7 25 23\n\
    20 11 10 24  4\n\
    14 21 16 12  6\n\
    \n\
    14 21 17 24  4\n\
    10 16 15  9 19\n\
    18  8 23 26 20\n\
    22 11 13  6  5\n\
     2  0 12  3  7"
            .split_once("\n\n")
            .unwrap();
    let (chosen_numbers, mut boards) = parse_input(numbers, boards);
    for chosen_num in chosen_numbers.iter() {
        for (ii, board) in boards.iter_mut().enumerate() {
            board.mark_cell(chosen_num);
            if board.get_winning() {
                assert_eq!(
                    board.get_unmarked().iter().sum::<usize>() * chosen_num,
                    4512
                );
                return;
            }
        }
    }
    println!("hej")
}
