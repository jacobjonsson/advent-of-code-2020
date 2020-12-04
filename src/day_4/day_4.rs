extern crate time;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use time::Instant;

pub fn day4() {
    let start = Instant::now();
    let lines = read_input();

    let mut valid_lines = 0;

    for line in lines.iter() {
        let groups: Vec<&str> = line.split(" ").collect();

        if !line.contains("byr:")
            || !line.contains("iyr:")
            || !line.contains("eyr:")
            || !line.contains("hgt:")
            || !line.contains("hcl:")
            || !line.contains("ecl:")
            || !line.contains("pid:")
        {
            continue;
        }

        let mut valid = true;
        'inner: for group in groups.iter() {
            let g: Vec<&str> = group.split(":").collect();
            if g[0] == "byr" {
                if g[1].len() != 4 {
                    valid = false;
                    break 'inner;
                }

                let year = g[1].parse::<i32>().unwrap();
                if year < 1920 || year > 2002 {
                    valid = false;
                    break 'inner;
                }

                continue;
            }
            if g[0] == "iyr" {
                if g[1].len() != 4 {
                    valid = false;
                    break 'inner;
                }

                let year = g[1].parse::<i32>().unwrap();
                if year < 2010 || year > 2020 {
                    valid = false;
                    break 'inner;
                }

                continue;
            }
            if g[0] == "eyr" {
                if g[1].len() != 4 {
                    valid = false;
                    break 'inner;
                }

                let year = g[1].parse::<i32>().unwrap();
                if year < 2020 || year > 2030 {
                    valid = false;
                    break 'inner;
                }

                continue;
            }
            if g[0] == "hgt" {
                if !g[1].contains("cm") && !g[1].contains("in") {
                    valid = false;
                    break 'inner;
                }

                if g[1].contains("cm") {
                    let s = g[1].replace("cm", "");
                    let h = s.parse::<i32>().unwrap();
                    if h < 150 || h > 193 {
                        valid = false;
                        break 'inner;
                    }
                }

                if g[1].contains("in") {
                    let s = g[1].replace("in", "");
                    let h = s.parse::<i32>().unwrap();
                    if h < 59 || h > 76 {
                        valid = false;
                        break 'inner;
                    }
                }

                continue;
            }
            if g[0] == "hcl" {
                if !g[1].contains("#") {
                    valid = false;
                    break 'inner;
                }

                let c = g[1].replace("#", "");
                if c.len() != 6 {
                    valid = false;
                    break 'inner;
                }
                continue;
            }
            if g[0] == "ecl" {
                if g[1] != "amb"
                    && g[1] != "blu"
                    && g[1] != "brn"
                    && g[1] != "gry"
                    && g[1] != "grn"
                    && g[1] != "hzl"
                    && g[1] != "oth"
                {
                    valid = false;
                    break 'inner;
                }
                continue;
            }
            if g[0] == "pid" {
                if g[1].len() != 9 {
                    valid = false;
                    break 'inner;
                }

                let ok = g[1].parse::<f64>().is_ok();
                if !ok {
                    valid = false;
                    break 'inner;
                }
                continue;
            }
        }

        if valid {
            valid_lines += 1;
        }
    }

    println!("Found {} valid passports", valid_lines);

    println!(
        "Took {} seconds to complete",
        start.elapsed().as_seconds_f32()
    );
}

fn read_input() -> Vec<String> {
    let f = File::open("src/day_4/input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut line_counter = 0;
    let mut lines = Vec::new();

    lines.push("".to_string());

    for line in file.lines() {
        let l = line.unwrap();

        if l.is_empty() {
            line_counter += 1;
            continue;
        }

        if lines.len() <= line_counter {
            lines.push("".to_string());
        }

        lines[line_counter] = format!("{} {}", lines[line_counter], l).trim().to_string();
    }

    return lines;
}
