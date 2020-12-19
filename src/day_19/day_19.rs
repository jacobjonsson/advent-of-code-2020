extern crate regex;
use regex::Regex;

use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day19() {
    let input = input_loader::read_input("src/day_19/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 19] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 19] Result for part 2: {}", part_2_result);
}

type RuleMap = HashMap<String, String>;

fn get_tree(num: &str, rules: &RuleMap) -> String {
    let rule = rules.get(num).unwrap();

    if Regex::new("\".\"").unwrap().is_match(rule) {
        return rule.replace("\"", "").to_string();
    } else {
        let parts = rule.split(" | ").collect::<Vec<&str>>();
        let mut res = vec![];
        for part in parts {
            let rxs = part
                .split(" ")
                .map(|n| get_tree(n, rules))
                .collect::<Vec<String>>();

            res.push(rxs.join(""))
        }
        return format!("(?:{})", res.join("|"));
    }
}

fn part_1(input: &str) -> i32 {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules = parts[0];
    let strings = parts[1].split("\n").collect::<Vec<&str>>();
    let mut rule_map: RuleMap = HashMap::new();

    for rule in rules.split("\n") {
        let parts = rule.split(": ").collect::<Vec<&str>>();
        rule_map.insert(parts[0].to_string(), parts[1].trim().to_string());
    }

    let mut matches = 0;
    let re = get_tree("0", &rule_map);
    for string in strings {
        let m = match Regex::new(&re).unwrap().find(string) {
            Some(v) => v,
            None => continue,
        };

        if m.start() == 0 && m.end() == string.len() {
            matches += 1;
        }
    }

    return matches;
}

fn match_line(
    line: &str,
    st: usize,
    ed: usize,
    rule: &str,
    dp: &mut HashMap<(usize, usize, String), bool>,
    r: &HashMap<String, Vec<Vec<String>>>,
    c: &HashMap<String, String>,
) -> bool {
    let key = (st, ed, rule.to_string());
    if dp.contains_key(&key) {
        return *dp.get(&key).unwrap();
    }

    let mut ret = false;
    if c.contains_key(rule) {
        ret = &line[st..ed] == c.get(rule).unwrap();
    } else {
        for option in r.get(rule).unwrap() {
            if match_list(line, st, ed, option, dp, r, c) {
                ret = true;
            }
        }
    }

    dp.insert(key, ret);
    return ret;
}

fn match_list(
    line: &str,
    st: usize,
    ed: usize,
    rules: &Vec<String>,
    dp: &mut HashMap<(usize, usize, String), bool>,
    r: &HashMap<String, Vec<Vec<String>>>,
    c: &HashMap<String, String>,
) -> bool {
    if st == ed && rules.len() == 0 {
        return true;
    } else if st == ed {
        return false;
    } else if rules.len() == 0 {
        return false;
    }

    let mut ret = false;
    for i in (st + 1)..(ed + 1) {
        if match_line(line, st, i, &rules[0], dp, r, c)
            && match_list(line, i, ed, &rules[1..].to_vec(), dp, r, c)
        {
            ret = true;
        }
    }
    return ret;
}

fn part_2(input: &str) -> i32 {
    let mut c: HashMap<String, String> = HashMap::new();
    let mut r: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for line in input.split("\n") {
        if line.contains(":") {
            let name = line.split(": ").collect::<Vec<&str>>()[0];
            let mut rules = line.split(": ").collect::<Vec<&str>>()[1];
            if name == "8" {
                rules = "42 | 42 8"
            } else if name == "11" {
                rules = "42 31 | 42 11 31"
            }

            if rules.contains("\"") {
                c.insert(name.to_string(), rules.replace("\"", ""));
            } else {
                let mut rxs: Vec<Vec<String>> = vec![];
                for option in rules.split(" | ") {
                    let mut tmp = vec![];
                    for rule in option.split(" ") {
                        tmp.push(rule.to_string());
                    }
                    rxs.push(tmp);
                }
                r.insert(name.to_string(), rxs);
            }
        }
    }

    let strings = input.split("\n\n").collect::<Vec<&str>>()[1]
        .split("\n")
        .collect::<Vec<&str>>();
    let mut ans = 0;
    for line in strings {
        let mut dp: HashMap<(usize, usize, String), bool> = HashMap::new();
        if match_line(line, 0, line.len(), "0", &mut dp, &r, &c) {
            ans += 1;
        }
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            2,
            part_1(
                "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            12,
            part_2(
                "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            )
        );
    }
}
