use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;

#[derive(Debug, Default, Clone)]
struct PacketFragment {
    is_num: bool,
    num: u32,
    packet_fragments: Vec<PacketFragment>,
}

impl PacketFragment {
    pub fn new(p: &str) -> Result<Self, Box<dyn Error>> {
        Ok(PacketFragment {
            is_num: true,
            num: p.parse::<u32>().unwrap(),
            packet_fragments: vec![],
        })
    }
}

#[derive(Debug, Default, Clone)]
struct Packet {
    packet_fragments: Vec<PacketFragment>,
}

impl Packet {
    pub fn new(p: &str) -> Result<Self, Box<dyn Error>> {
        let mut frag_map: HashMap<i32, PacketFragment> = HashMap::new();
        let mut pckt = Packet::default();
        let mut curr_pckt_frag = PacketFragment::default();

        let p_mod = p
            .replace(",", " ")
            .replace("[", " [ ")
            .replace("]", " ] ")
            .replace("  ", " ");

        let mut depth = 0;
        for c in p_mod.split(" ") {
            match c {
                "[" => {
                    depth += 1;
                    curr_pckt_frag = PacketFragment::default();
                    frag_map.insert(depth, curr_pckt_frag.clone());
                }
                "]" => {
                    depth -= 1;
                    if depth != 0 {
                        let prev_pckt_frag = frag_map.get_mut(&depth).unwrap();
                        prev_pckt_frag.packet_fragments.push(curr_pckt_frag);
                        curr_pckt_frag = prev_pckt_frag.clone();
                    }
                }
                _ => {
                    if c.len() < 1 {
                        continue;
                    }
                    curr_pckt_frag
                        .packet_fragments
                        .push(PacketFragment::new(c).unwrap());
                }
            }
        }

        pckt.packet_fragments = curr_pckt_frag.packet_fragments;

        Ok(pckt)
    }
}

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = solve(&contents, config.problem_number)?;
    println!("result -> {result}");
    Ok(())
}

fn solve(contents: &str, problem_number: i32) -> Result<i32, Box<dyn Error>> {
    if problem_number == 1 {
        return ordered_packet_indices(contents);
    } else {
    }
    Ok(1)
}

fn ordered_packet_indices(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut packets: VecDeque<Packet> = contents
        .split("\n")
        .filter(|p| *p != "")
        .map(|raw_packet| Packet::new(raw_packet).unwrap())
        .collect();

    let mut index = 1;
    let mut result = 0;
    while !packets.is_empty() {
        let pckt1 = packets.pop_front().unwrap();
        let pckt2 = packets.pop_front().unwrap();

        // dbg!(&pckt1, &pckt2);
        // dbg!(index);
        if correct_packet_order(&pckt1.packet_fragments, &pckt2.packet_fragments) {
            result += index;
        }

        index += 1;
    }

    Ok(result)
}

// TODO: take packet fragments
fn correct_packet_order(pckt1: &Vec<PacketFragment>, pckt2: &Vec<PacketFragment>) -> bool {
    let mut p1_iter = pckt1.iter().peekable();
    let mut p2_iter = pckt2.iter().peekable();

    while !p1_iter.peek().is_none() && !p2_iter.peek().is_none() {
        let mut pf1 = p1_iter.next().unwrap();
        let mut pf2 = p2_iter.next().unwrap();

        if pf1.is_num && pf2.is_num {
            if pf1.num > pf2.num {
                return false;
            }
        } else if pf1.is_num {
            if pf2.packet_fragments.is_empty() {
                return false;
            } else if pf2.packet_fragments[0].is_num && pf1.num > pf2.packet_fragments[0].num {
                return false;
            } else {
                while !pf2.is_num {
                    if pf2.packet_fragments.is_empty() {
                        return false;
                    }
                    pf2 = &pf2.packet_fragments[0];
                }
                if pf1.num > pf2.num {
                    return false;
                }
            }
        } else if pf2.is_num {
            if pf1.packet_fragments.is_empty() {
                continue;
            } else if pf1.packet_fragments[0].is_num && pf1.packet_fragments[0].num > pf2.num {
                return false;
            } else {
                while !pf1.is_num {
                    if pf1.packet_fragments.is_empty() {
                        return false;
                    }
                    pf1 = &pf1.packet_fragments[0];
                }
                if pf1.num > pf2.num {
                    return false;
                }
            }
        } else {
            if !correct_packet_order(&pf1.packet_fragments, &pf2.packet_fragments) {
                return false;
            }
        }
    }

    if p2_iter.peek().is_none() && !p1_iter.peek().is_none() {
        return false;
    }

    return true;
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
    fn p7() {
        let input = "[[[[1]]],[2,3,4]]
[[1],2]";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(0, result);
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
