use std::collections::HashSet;
use std::error::Error;
use std::fs;

pub struct Config {
    file_path: String,
    problem_number: i128,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("must provide input file_path and which problem to solve");
        }
        let file_path = args[1].clone();
        let problem_number = args[2].clone().parse::<i128>().unwrap_or_default();

        Ok(Config {
            file_path,
            problem_number,
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Sensor {
    coords: Coords,
    nearest_beacon: Beacon,
    md: i128,
}

#[derive(Debug, Default, Clone, Copy)]
struct Beacon {
    coords: Coords,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: i128,
    y: i128,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = solve(&contents, config.problem_number)?;
    println!("result -> {result}");
    Ok(())
}

fn solve(contents: &str, problem_number: i128) -> Result<i128, Box<dyn Error>> {
    if problem_number == 1 {
        let pos_no_beacon = positions_without_beacon(contents);
        return Ok(pos_no_beacon);
    } else {
        return tuning_freq(contents);
    }
}

fn tuning_freq(contents: &str) -> Result<i128, Box<dyn Error>> {
    let sensors: Vec<Sensor> = parse_sensors(contents);
    let s: HashSet<Coords> = sensors.iter().map(|a| a.coords).collect();
    let b: HashSet<Coords> = sensors.iter().map(|a| a.nearest_beacon.coords).collect();

    let mut to_check: HashSet<Coords> = HashSet::new();

    for (i, sensor) in sensors.iter().enumerate() {
        dbg!(i);
        let border = sensor.md + 1;
        let top = sensor.coords.y - border;
        let bottom = sensor.coords.y + border;

        for y in top..bottom + 1 {
            if y < 0 || y > 4000000 {
                continue;
            }

            let wiggle = border - (y - sensor.coords.y).abs();
            let x_right = sensor.coords.x + wiggle;
            let x_left = sensor.coords.x - wiggle;

            if !(x_left < 0 || x_left > 4000000) {
                to_check.insert(Coords { x: x_left, y: y });
            }

            if !(x_right < 0 || x_right > 4000000) {
                to_check.insert(Coords { x: x_right, y: y });
            }
        }
    }

    dbg!(to_check.len());

    for (j, pt) in to_check.iter().enumerate() {
        if s.contains(&pt)
            || b.contains(&pt)
            || pt.y < 0
            || pt.x < 0
            || pt.y > 4000000
            || pt.x > 4000000
        {
            continue;
        }

        let mut in_range = false;
        for s in &sensors {
            if manhattan_distance(*pt, s.coords) <= s.md {
                // point within a sensor's range
                in_range = true;
                break;
            }
        }
        if !in_range {
            return Ok(pt.x * 4000000 + pt.y);
        }
    }

    Ok(0)
}

fn positions_without_beacon(contents: &str) -> i128 {
    let sensors: Vec<Sensor> = parse_sensors(contents);

    let s: HashSet<Coords> = sensors.iter().map(|a| a.coords).collect();
    let b: HashSet<Coords> = sensors.iter().map(|a| a.nearest_beacon.coords).collect();

    let max_md = sensors.iter().map(|s| s.md).max().unwrap();
    let min_x = sensors
        .iter()
        .map(|a| a.coords.x.min(a.nearest_beacon.coords.x))
        .min()
        .unwrap()
        - max_md;
    let max_x = sensors
        .iter()
        .map(|a| a.coords.x.max(a.nearest_beacon.coords.x))
        .max()
        .unwrap()
        + max_md;

    let mut result = 0;
    let y = 2000000;
    for x in min_x..max_x {
        let c = Coords { x, y };
        if s.contains(&c) || b.contains(&c) {
            continue;
        }
        for s in &sensors {
            if manhattan_distance(c, s.coords) <= s.md {
                result += 1;
                break;
            }
        }
    }

    return result;
}

fn parse_sensors(contents: &str) -> Vec<Sensor> {
    return contents
        .lines()
        .map(|l| {
            let mut s = Sensor::default();

            let l = l
                .replace("Sensor at ", "")
                .replace(": closest beacon is at", "");
            let raw_coords: Vec<i128> = l
                .split(&[',', '=', 'x', 'y', ' '][..])
                .filter(|p| *p != "")
                .map(|n| n.parse::<i128>().unwrap())
                .collect();

            s.coords.x = raw_coords[0];
            s.coords.y = raw_coords[1];
            s.nearest_beacon.coords.x = raw_coords[2];
            s.nearest_beacon.coords.y = raw_coords[3];

            s.md = manhattan_distance(s.coords, s.nearest_beacon.coords);

            s
        })
        .collect();
}

fn manhattan_distance(a: Coords, b: Coords) -> i128 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn p1() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(26, result);
    }

    #[test]
    fn p2() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(56000011, result);
    }

    #[test]
    fn p3() {
        let input = "Sensor at x=0, y=11: closest beacon is at x=2, y=11";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(56000011, result);
    }
}
