use std::cmp::Ordering;
use std::error::Error;
use std::fs;

use serde::Deserialize;

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

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(untagged)]
enum Packet {
    Number(u64),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::Number(a), Self::List(b)) => vec![Self::Number(*a)].cmp(b),
            (Self::List(a), Self::Number(b)) => a.cmp(&vec![Self::Number(*b)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        return sum_ordered_packet_indices(contents);
    } else {
        return ordered_divider_indices(contents);
    }
}

fn ordered_divider_indices(contents: &str) -> Result<usize, Box<dyn Error>> {
    let mut packets: Vec<Packet> = contents
        .split("\n")
        .filter(|l| *l != "")
        .map(|l| serde_json::from_str::<Packet>(l).unwrap())
        .collect();

    let divider1 = serde_json::from_str::<Packet>("[[2]]").unwrap();
    let divider2 = serde_json::from_str::<Packet>("[[6]]").unwrap();
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort();

    let ind1 = packets.binary_search(&divider1).unwrap() + 1;
    let ind2 = packets.binary_search(&divider2).unwrap() + 1;

    Ok(ind1 * ind2)
}

fn sum_ordered_packet_indices(contents: &str) -> Result<usize, Box<dyn Error>> {
    let mut result = 0;
    for (i, packets) in contents.split("\n\n").enumerate() {
        let mut parsed_packets = packets
            .lines()
            .map(|l| serde_json::from_str::<Packet>(l).unwrap());

        let first_packet = parsed_packets.next().unwrap();
        let second_packet = parsed_packets.next().unwrap();

        if first_packet < second_packet {
            result += 1 + i;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn p1() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,10]";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(13, result);
    }

    #[test]
    fn p2() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[6]
[[8,7,6]]

[[]]
[[[]]]";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(6, result);
    }

    #[test]
    fn p3() {
        let input = "[[1],[2,3,4]]
[[1],4]";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(1, result);
    }

    #[test]
    fn p4() {
        let input = "[[1],[2,3,4]]
[[1],[4]]";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(0, result);
    }

    #[test]
    fn p5() {
        let input = "[[1],[2,3,4]]
[[1],2]";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(1, result);
    }

    #[test]
    fn p6() {
        let input = "[[[1]],[2,3,4]]
[[1],2]";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(1, result);
    }

    #[test]
    fn p9() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,10]";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(140, result);
    }
    // #[test]
    // fn p2() {
    //     let input = "Sabqponm
    //     abcryxxl
    //     accszExk
    //     acctuvwj
    //     abdefghi";

    //     let p = 2;
    //     let result = solve(input, p).unwrap_or_else(|err| {
    //         println!("failed to unwrap test result, {err}");
    //         return -1;
    //     });

    //     assert_eq!(29, result);
    // }
}
