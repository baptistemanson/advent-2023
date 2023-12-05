use crate::data3::INPUT;

type Input = Vec<Vec<char>>;
struct Problem {
    dim_x: usize,
    dim_y: usize,
    input: Input,
}
// too high 6991823
// too high 7823990
pub fn pb1() {
    let expanded_matrix = expand(INPUT);
    let adjacent_symbol = flag_symbol(&expanded_matrix);
    //display_bool(&adjacent_symbol, 0);
    dbg!(sum(&expanded_matrix, &adjacent_symbol));
}

fn sum(matrix: &Problem, symbols: &Vec<Vec<bool>>) -> u64 {
    let mut sum = 0;
    let mut curr: u64 = 0;
    let mut had_flag = false;
    for x in 0..matrix.dim_x {
        for y in 0..matrix.dim_y {
            let elem = matrix.input[x][y];
            if elem == '.' || is_symbol(elem) {
                if had_flag {
                    sum += curr;
                }
                curr = 0;
                had_flag = false;
                continue;
            }
            had_flag = had_flag || symbols[x][y];
            if elem.is_digit(10) {
                curr = curr * 10 + elem.to_digit(10).unwrap() as u64;
                continue;
            }
        }
    }
    sum
}

fn expand(map: &str) -> Problem {
    let mut input: Vec<Vec<char>> = map
        .lines()
        .map(|l| {
            let l = format!("..{}..", l);
            l.chars().collect::<Vec<char>>()
        })
        .collect();
    let empty_line: Vec<char> = (0..input[0].len()).map(|_i| '.').collect();
    input.insert(0, empty_line.clone());
    input.push(empty_line);
    Problem {
        dim_x: input.len(),
        dim_y: input[0].len(),
        input,
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn flag_symbol(input: &Problem) -> Vec<Vec<bool>> {
    let mut output = vec![vec![false; input.dim_y]; input.dim_x];
    for x in 0..input.dim_x - 1 {
        for y in 0..input.dim_y - 1 {
            if is_symbol(input.input[x][y]) {
                for i in x - 1..=x + 1 {
                    for j in y - 1..=y + 1 {
                        output[i][j] = true;
                    }
                }
            }
        }
    }
    output
}

pub fn pb2() {
    let matrix = expand(INPUT); // instead of expand, we could simply check boundaries
    let mut gears = vec![vec![(0, 0); matrix.dim_y]; matrix.dim_x];
    for x in 1..matrix.dim_x - 1 {
        let mut number: u32 = 0;
        let mut number_len = 0;
        for y in 1..matrix.dim_y - 1 {
            if matrix.input[x][y].is_digit(10) {
                number = number * 10 + matrix.input[x][y].to_digit(10).unwrap();
                number_len += 1;
            } else {
                if number_len != 0 {
                    // we have read a number, we will look around for gears
                    for i in x - 1..=x + 1 {
                        for j in y - number_len - 1..=y {
                            if matrix.input[i][j] == '*' {
                                gears[i][j] = merge(gears[i][j], number)
                            }
                        }
                    }
                }
                number = 0;
                number_len = 0;
            }
        }
    }
    dbg!(gears
        .iter()
        .map(|l| l.iter().filter(|c| c.0 == 2).map(|c| c.1).sum::<u32>())
        .sum::<u32>());
}

fn merge((count, acc): (u32, u32), curr: u32) -> (u32, u32) {
    if count == 0 {
        (1, curr)
    } else if count == 1 {
        (2, curr * acc)
    } else {
        (3, 0)
    }
}
