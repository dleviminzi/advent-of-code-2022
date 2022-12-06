use std::fs;
use std::collections::HashSet;
use std::error::Error;


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

fn solve(contents: &str, problem_number: i32) -> Result<i32, Box<dyn Error>> {

    if problem_number == 1 {
        return packet_start_index(contents, 4);
    } else {
        return packet_start_index(contents, 14);
    }

}

fn packet_start_index(contents: &str, distinct_count: usize) -> Result<i32, Box<dyn Error>> {

    let mut c_set = HashSet::new();
    let mut left: usize = 0;

    for (right, c) in contents.chars().enumerate() {

        if right - left >= distinct_count {
            return Ok(right as i32)
        }

        while c_set.contains(&c) {
            let l_char = &contents.chars().nth(left).unwrap();
            c_set.remove(l_char);
            left += 1;
        }

        c_set.insert(c);

    }

    Err("no packet start index found")?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(5, result);
    }


    #[test]
    fn p2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(7, result);
    }

    #[test]
    fn p3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(6, result);
    }

    #[test]
    fn p4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(10, result);
    }

    #[test]
    fn p5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(11, result);
    }

    #[test]
    fn p6() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(23, result);
    }


    #[test]
    fn p7() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(19, result);
    }

    #[test]
    fn p8() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(23, result);
    }

    #[test]
    fn p9() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(29, result);
    }

    #[test]
    fn p10() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 0
        });

        assert_eq!(26, result);
    }
}
