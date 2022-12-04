use std::fs;
use std::error::Error;


pub struct Config {
    file_path: String,
    problem_number: i32,
}

struct Section {
    open: i32,
    close: i32
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
        return check_containment(contents)
    } else {
        return check_overlap(contents)
    }

}

fn get_sections(line: &str) -> Result<Vec<Section>, Box<dyn Error>> {
    let parts: Vec<&str> = line.split(",").collect();

    if parts.len() < 2 {
        Err("does not have two sections of coverage {line}")?
    }

    let first_sections: Vec<&str>  = parts[0].split("-").collect();
    let second_sections: Vec<&str> = parts[1].split("-").collect();

    let section_one = Section{open: first_sections[0].parse::<i32>().unwrap(), close: first_sections[1].parse::<i32>().unwrap()};
    let section_two = Section{open: second_sections[0].parse::<i32>().unwrap(), close: second_sections[1].parse::<i32>().unwrap()};

    Ok(vec![section_one, section_two])
}

fn check_overlap(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut result = 0;

    for line in contents.lines() {
        let sections = get_sections(line)?;

        if sections[0].open >= sections[1].open && sections[0].open <= sections[1].close {
            result += 1
        } else if sections[1].open >= sections[0].open && sections[1].open <= sections[0].close {
            result += 1
        } else if sections[0].close <= sections[1].close && sections[0].close >= sections[1].open {
            result += 1
        } else if sections[1].close <= sections[0].close && sections[1].close >= sections[0].open {
            result += 1
        }

    }

    Ok(result)
}

fn check_containment(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut result = 0;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(",").collect();

        if parts.len() < 2 {
            Err("does not have two sections of coverage {line}")?
        }

        let first_sections: Vec<&str>  = parts[0].split("-").collect();
        let second_sections: Vec<&str> = parts[1].split("-").collect();

        // check if sections are covered by the other sections 
        if first_sections[0].parse::<i32>().unwrap() >= second_sections[0].parse::<i32>().unwrap() && 
            first_sections[1].parse::<i32>().unwrap() <= second_sections[1].parse::<i32>().unwrap() { 
            result += 1;
        } else if second_sections[0].parse::<i32>().unwrap() >= first_sections[0].parse::<i32>().unwrap() && 
            second_sections[1].parse::<i32>().unwrap() <= first_sections[1].parse::<i32>().unwrap() {
            result += 1;
        }
    }

    Ok(result)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n3-14,14-80";   
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(2, result);
    }

    #[test]
    fn p2() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n3-14,14-80";   
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(5, result);
    }
}
