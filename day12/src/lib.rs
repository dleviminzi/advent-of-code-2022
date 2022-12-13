use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;

pub struct Config {
    file_path: String,
    problem_number: i32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("must provide input file_path and which problem to solve");
        }
        let file_path = args[1].clone();
        let problem_number = args[2].clone().parse::<i32>().unwrap_or_default();

        Ok(Config {
            file_path,
            problem_number,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    i: usize,
    j: usize,
    depth: i32,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = solve(&contents, config.problem_number)?;
    println!("result -> {result}");
    Ok(())
}

fn solve(contents: &str, problem_number: i32) -> Result<i32, Box<dyn Error>> {
    if problem_number == 1 {
        return shortest_path(contents);
    } else {
        return shorted_viable_path(contents);
    }
}

fn shorted_viable_path(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut grid = make_grid(contents);
    let (starts, target) = find_viable_positions(&mut grid);

    let mut result = i32::MAX;
    for start in starts {
        let cnt = search(
            &mut grid,
            Point {
                i: start.0,
                j: start.1,
                depth: 1,
            },
            target,
        );

        if cnt != -1 && cnt < result {
            result = cnt;
        }
    }

    Ok(result)
}

fn shortest_path(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut grid = make_grid(contents);
    let (start, target) = find_positions(&mut grid);
    let cnt = search(&mut grid, start, target);
    Ok(cnt)
}

fn search(grid: &Vec<Vec<char>>, start: Point, target: Point) -> i32 {
    let mut to_visit = VecDeque::from([start]);
    let mut seen: HashSet<(usize, usize)> = HashSet::from([(start.i, start.j)]);

    while !to_visit.is_empty() {
        let curr_pt = to_visit.pop_front().unwrap();

        if curr_pt.i == target.i && curr_pt.j == target.j {
            return curr_pt.depth - 1;
        }

        // look around
        if !curr_pt.i.checked_sub(1).is_none()
            && is_climbable(grid[curr_pt.i][curr_pt.j], grid[curr_pt.i - 1][curr_pt.j])
            && !seen.contains(&(curr_pt.i - 1, curr_pt.j))
        {
            seen.insert((curr_pt.i - 1, curr_pt.j));
            to_visit.push_back(Point {
                i: curr_pt.i - 1,
                j: curr_pt.j,
                depth: curr_pt.depth + 1,
            })
        }

        if !curr_pt.j.checked_sub(1).is_none()
            && is_climbable(grid[curr_pt.i][curr_pt.j], grid[curr_pt.i][curr_pt.j - 1])
            && !seen.contains(&(curr_pt.i, curr_pt.j - 1))
        {
            seen.insert((curr_pt.i, curr_pt.j - 1));
            to_visit.push_back(Point {
                i: curr_pt.i,
                j: curr_pt.j - 1,
                depth: curr_pt.depth + 1,
            })
        }

        if curr_pt.i + 1 < grid.len()
            && is_climbable(grid[curr_pt.i][curr_pt.j], grid[curr_pt.i + 1][curr_pt.j])
            && !seen.contains(&(curr_pt.i + 1, curr_pt.j))
        {
            seen.insert((curr_pt.i + 1, curr_pt.j));
            to_visit.push_back(Point {
                i: curr_pt.i + 1,
                j: curr_pt.j,
                depth: curr_pt.depth + 1,
            })
        }

        if curr_pt.j + 1 < grid[0].len()
            && is_climbable(grid[curr_pt.i][curr_pt.j], grid[curr_pt.i][curr_pt.j + 1])
            && !seen.contains(&(curr_pt.i, curr_pt.j + 1))
        {
            seen.insert((curr_pt.i, curr_pt.j + 1));
            to_visit.push_back(Point {
                i: curr_pt.i,
                j: curr_pt.j + 1,
                depth: curr_pt.depth + 1,
            })
        }
    }

    return -1;
}

// next can be at most 1 greater than current
fn is_climbable(curr: char, next: char) -> bool {
    if next <= char::from_u32(curr as u32 + 1).unwrap() {
        return true;
    }
    return false;
}

fn make_grid(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_positions(grid: &mut Vec<Vec<char>>) -> (Point, Point) {
    let (mut start, mut target) = ((0, 0), (0, 0));
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, spot) in row.iter_mut().enumerate() {
            if *spot == 'S' {
                *spot = 'a';
                start = (i, j);
            } else if *spot == 'E' {
                *spot = 'z';
                target = (i, j);
            }
        }
    }

    return (
        Point {
            i: start.0,
            j: start.1,
            depth: 1,
        },
        Point {
            i: target.0,
            j: target.1,
            depth: -1,
        },
    );
}

fn find_viable_positions(grid: &mut Vec<Vec<char>>) -> (Vec<(usize, usize)>, Point) {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut target: (usize, usize) = (0, 0);
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, spot) in row.iter_mut().enumerate() {
            if *spot == 'a' {
                starts.push((i, j));
            } else if *spot == 'S' {
                *spot = 'a';
                starts.push((i, j));
            } else if *spot == 'E' {
                *spot = 'z';
                target = (i, j);
            }
        }
    }

    return (
        starts,
        Point {
            i: target.0,
            j: target.1,
            depth: -1,
        },
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn p1() {
        let input = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(31, result);
    }

    #[test]
    fn p2() {
        let input = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";

        let p = 2;
        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1;
        });

        assert_eq!(29, result);
    }
}
