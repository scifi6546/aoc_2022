use super::Problem;
use std::cmp::max;
pub const P_08: Problem = Problem {
    number: 8,
    problem_a: a,
    print_problem_a_output: true,
    problem_a_output: Some("1698"),
    problem_b: b,
    problem_b_output: Some("672280"),
    print_problem_b_output: true,
};
fn make_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .split_whitespace()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
fn a(input: &str) -> String {
    let grid = make_grid(input);

    let mut num_visible = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let height = grid[x][y];
            let mut visible = true;
            let left = (0..y)
                .map(|y_n| grid[x][y_n] < height)
                .fold(true, |acc, x| acc && x);
            let right = (y + 1..grid[x].len())
                .map(|y_n| grid[x][y_n] < height)
                .fold(true, |acc, x| acc && x);
            let up = (0..x)
                .map(|x_n| grid[x_n][y] < height)
                .fold(true, |acc, x| acc && x);
            let down = (x + 1..grid.len())
                .map(|x_n| grid[x_n][y] < height)
                .fold(true, |acc, x| acc && x);
            if left || right || up || down {
                num_visible += 1;
            }
        }
    }
    num_visible.to_string()
}
fn b(input: &str) -> String {
    let grid = make_grid(input);
    let mut set_grid = grid.clone();
    let mut max_score = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let height = grid[x][y];

            let mut max_dist_left = x;
            for (count, x) in (0..x).rev().enumerate() {
                if grid[x][y] >= height {
                    max_dist_left = count + 1;
                    break;
                }
            }
            let mut max_dist_right = grid.len() - x - 1;
            for x_n in (x + 1..grid.len()) {
                if grid[x_n][y] >= height {
                    max_dist_right = x_n - x;
                    break;
                }
            }
            let mut max_dist_up = y;
            for (count, y) in (0..y).rev().enumerate() {
                if grid[x][y] >= height {
                    max_dist_up = count + 1;
                    break;
                }
            }
            let mut max_dist_down = grid[x].len() - y - 1;
            for y_n in (y + 1..grid[x].len()) {
                if grid[x][y_n] >= height {
                    max_dist_down = y_n - y;
                    break;
                }
            }
            let this_score = max_dist_left * max_dist_right * max_dist_up * max_dist_down;
            max_score = max(max_score, this_score);

            set_grid[x][y] = this_score as u32;
        }
    }

    max_score.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
    30373
    25512
    65332
    33549
    35390
        "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "21")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "8");
    }
}
