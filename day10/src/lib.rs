use std::fs;
use std::error::Error;
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
    println!("result ->\n{result}");
    Ok(())
}

fn solve(contents: &str, problem_number: i32) -> Result<String, Box<dyn Error>> {

    if problem_number == 1 {
        return sum_signal_strength(contents);
    } else {
        return draw(contents);
    }

}

fn draw(contents: &str) -> Result<String, Box<dyn Error>> {
    let cycle_values: Vec<i32> = calc_cycle_values(contents);
    let mut screen = vec!['.'; cycle_values.len()+1];

    for (i, val) in cycle_values.iter().enumerate() {
        let pixel = i as i32 % 40;
        if pixel == *val-1 || pixel == *val || pixel == *val+1 {
            screen[i] = '#';
        }
    }

    let img: String = screen.chunks(40).map(|row| row.iter().collect::<String>() + "\n").collect();

    Ok(img)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Instr {
    cycles: i32,
    amt: i32,
}

fn sum_signal_strength(contents: &str) -> Result<String, Box<dyn Error>> {
    let cycle_value: Vec<i32> = calc_cycle_values(contents);

    let mut result = 0;
    let mut i = 19;

    while i < cycle_value.len() {
        result += (i+1) as i32 * cycle_value[i];
        i += 40;
    }

    let res_str = result.to_string();

    Ok(res_str)
}

fn calc_cycle_values(contents: &str) -> Vec<i32> {
    let mut x_value: i32 = 1;
    let mut cycle_value: Vec<i32> = Vec::new();
    let mut instructions: Vec<Instr> = Vec::new();

    for (instr_num, line) in contents.lines().enumerate() {
        if instr_num > 0 {
            cycle_value.push(x_value);
        }

        for instruction in &mut instructions {
            instruction.cycles -= 1;
            if instruction.cycles == 0 {
                x_value += instruction.amt;
            }
        }


        let (amt_raw, instr) = parse(line).unwrap();
        let amt = amt_raw.trim().parse::<i32>().unwrap_or(0);
        if instr == "noop" {
            continue;
        } else {
            instructions.push(Instr { cycles: 1, amt })
        }
        cycle_value.push(x_value);
    }

    return cycle_value;
}

fn parse(i: &str) -> IResult<&str, &str> {
    alpha1(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return "".to_string() 
        });

        assert_eq!("13140", result);
    }

    #[test]
    fn p3() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return "".to_string()
        });

        assert_eq!("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
", result);
    }
}
