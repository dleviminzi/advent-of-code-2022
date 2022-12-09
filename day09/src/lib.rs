use std::fs;
use std::error::Error;
use std::collections::HashSet;
use nom::{character::complete::alpha1, IResult};

pub struct Config {
    file_path: String,
    problem_number: i32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("must provide input file_path and which problem to solve")
        }
        let file_path = args[1].clone();
        let problem_number = args[2].clone().parse::<i32>().unwrap_or_default();

        Ok(Config { file_path, problem_number })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = solve(&contents, config.problem_number)?;
    println!("result -> {result}");
    Ok(())
}

fn solve(contents: &str, problem_number: i32) -> Result<usize, Box<dyn Error>> {

    if problem_number == 1 {
        return tail_position_count(contents)
    } else {
        return tail_position_count_10(contents)
    }

}

#[derive(Hash)] #[derive(PartialEq)] #[derive(Eq)] #[derive(Debug)] #[derive(Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn tail_position_count_10(contents: &str) -> Result<usize, Box<dyn Error>> {
    let mut rope: Vec<Pos> = vec![Pos{x: 0, y: 0}; 10];
    let mut tail_positions: HashSet<Pos> = HashSet::from([Pos{x: 0, y: 0}]);


    for line in contents.lines() {
        let (amt_raw, dir) = parse(line).unwrap();
        let amt = amt_raw.trim().parse::<i32>().unwrap();

        match dir {
            "U" => {
                for _j in 0..amt {
                    rope.last_mut().unwrap().y += 1;

                    for index in (0..rope.len()-1).rev() {
                        rope[index] = new_position(&rope[index], &rope[index+1]);
                    }

                    tail_positions.insert(Pos{x: rope[0].x, y: rope[0].y});
                }
            },
            "D" => {
                for _j in 0..amt {
                    rope.last_mut().unwrap().y -= 1;

                    for index in (0..rope.len()-1).rev() {
                        rope[index] = new_position(&rope[index], &rope[index+1]);
                    }

                    tail_positions.insert(Pos{x: rope[0].x, y: rope[0].y});
                }
            },
            "L" => {
                for _i in 0..amt {
                    rope.last_mut().unwrap().x -= 1;

                    for index in (0..rope.len()-1).rev() {
                        rope[index] = new_position(&rope[index], &rope[index+1]);
                    }

                    tail_positions.insert(Pos{x: rope[0].x, y: rope[0].y});
                }
            },
            "R" => {
                for _i in 0..amt {
                    rope.last_mut().unwrap().x += 1;

                    for index in (0..rope.len()-1).rev() {
                        rope[index] = new_position(&rope[index], &rope[index+1]);
                    }

                    tail_positions.insert(Pos{x: rope[0].x, y: rope[0].y});
                }
            },
            _ => { }        
        }
    }

    Ok(tail_positions.len())
}

fn tail_position_count(contents: &str) -> Result<usize, Box<dyn Error>> {
    let mut tail_position = Pos{x: 0, y: 0};
    let mut head_position = Pos{x: 0, y: 0};
    let mut tail_positions: HashSet<Pos> = HashSet::from([Pos{x: 0, y: 0}]);


    for line in contents.lines() {
        let (amt_raw, dir) = parse(line).unwrap();
        let amt = amt_raw.trim().parse::<i32>().unwrap();

        match dir {
            "U" => {
                for _j in 0..amt {
                    head_position.y += 1;
                    tail_position = new_position(&tail_position, &head_position);
                    tail_positions.insert(Pos{x: tail_position.x, y: tail_position.y});
                }
            },
            "D" => {
                for _j in 0..amt {
                    head_position.y -= 1;
                    tail_position = new_position(&tail_position, &head_position);
                    tail_positions.insert(Pos{x: tail_position.x, y: tail_position.y});
                }
            },
            "L" => {
                for _i in 0..amt {
                    head_position.x -= 1;
                    tail_position = new_position(&tail_position, &head_position);
                    tail_positions.insert(Pos{x: tail_position.x, y: tail_position.y});
                }
            },
            "R" => {
                for _i in 0..amt {
                    head_position.x += 1; 
                    tail_position = new_position(&tail_position, &head_position);
                    tail_positions.insert(Pos{x: tail_position.x, y: tail_position.y});
                }
            },
            _ => Err("unknown direction {dir}")?
        }
    }

    Ok(tail_positions.len())
}

fn new_position(tail_position: &Pos, head_position: &Pos) -> Pos {
    let mut next_pos = tail_position.clone();

    if distance(tail_position, head_position) > 2.0 {
        if head_position.x > tail_position.x && head_position.y > tail_position.y {
            next_pos.x += 1;
            next_pos.y += 1;
            return next_pos;
        } else if head_position.x < tail_position.x && head_position.y < tail_position.y {
            next_pos.x -= 1;
            next_pos.y -= 1;
            return next_pos;
        } else if head_position.x > tail_position.x && head_position.y < tail_position.y {
            next_pos.x += 1;
            next_pos.y -= 1;
            return next_pos;
        } else {
            next_pos.x -= 1;
            next_pos.y += 1;
            return next_pos;
        }
    }
    else if head_position.x - tail_position.x == 2 {
        next_pos.x += 1;
        return next_pos;
    } else if head_position.x - tail_position.x == -2 {
        next_pos.x -= 1;
        return next_pos;
    } else if head_position.y - tail_position.y == 2 {
        next_pos.y += 1;
        return next_pos;
    } else if head_position.y - tail_position.y == -2 {
        next_pos.y -= 1;
        return next_pos;
    }

    return tail_position.clone();
}

fn distance(tail_position: &Pos, head_position: &Pos) -> f32 {
    (((head_position.x-tail_position.x).abs().pow(2) + (head_position.y-tail_position.y).abs().pow(2)) as f32).sqrt()
}

fn parse(i: &str) -> IResult<&str, &str> {
    alpha1(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(13, result);
    }

    #[test]
    fn p2() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(1, result);
    }

    #[test]
    fn p3() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

        let p = 2;
        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(36, result);
    }

    #[test]
    fn p4() {
        let input = "R 10";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(2, result);
    }

}
