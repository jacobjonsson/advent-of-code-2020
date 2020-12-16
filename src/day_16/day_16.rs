use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day16() {
    let input = input_loader::read_input("src/day_16/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 16] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 16] Result for part 2: {}", part_2_result);
}

#[derive(Debug, Clone, Hash, Eq)]
struct Rule {
    name: String,
    lower_lower: i64,
    lower_upper: i64,
    upper_lower: i64,
    upper_upper: i64,
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

fn create_rule(input: &str) -> Rule {
    let parts = input.split(":").collect::<Vec<&str>>();
    let bounds = parts[1].trim().split(" or ").collect::<Vec<&str>>();
    let lower_bounds = bounds[0]
        .split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let upper_bounds = bounds[1]
        .split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    return Rule {
        name: String::from(parts[0]),
        lower_lower: lower_bounds[0],
        lower_upper: lower_bounds[1],
        upper_lower: upper_bounds[0],
        upper_upper: upper_bounds[1],
    };
}

fn validate_rule(rule: &Rule, value: i64) -> bool {
    if value < rule.lower_lower || value > rule.upper_upper {
        return false;
    }

    if value > rule.lower_upper && value < rule.upper_lower {
        return false;
    }

    return true;
}

fn part_1(input: &str) -> i64 {
    let rules = input.split("your ticket:").collect::<Vec<&str>>()[0]
        .trim()
        .split("\n")
        .map(create_rule)
        .collect::<Vec<Rule>>();
    let nearby_tickets = input.split("nearby tickets:\n").collect::<Vec<&str>>()[1]
        .split("\n")
        .map(|x| x.split(","))
        .map(|x| x.map(|y| y.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();

    let mut error_rate = 0;
    for ticket in nearby_tickets {
        for value in ticket {
            let valid = rules.iter().any(|rule| validate_rule(rule, value));
            if !valid {
                error_rate += value;
            }
        }
    }

    return error_rate;
}

fn part_2(input: &str) -> i64 {
    let rules = input.split("your ticket:").collect::<Vec<&str>>()[0]
        .trim()
        .split("\n")
        .map(create_rule)
        .collect::<Vec<Rule>>();
    let nearby_tickets = input.split("nearby tickets:\n").collect::<Vec<&str>>()[1]
        .split("\n")
        .map(|x| x.split(","))
        .map(|x| x.map(|y| y.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();

    let mut valid_tickets: Vec<Vec<i64>> = Vec::new();
    for ticket in nearby_tickets {
        let mut valid = true;
        for value in &ticket {
            if !rules.iter().any(|rule| validate_rule(rule, *value)) {
                valid = false;
            }
        }
        if valid {
            valid_tickets.push(ticket.clone());
        }
    }

    let mut positions: HashMap<usize, Vec<&Rule>> = HashMap::new();
    for ticket in &valid_tickets {
        for (idx, value) in ticket.iter().enumerate() {
            let applicable_rules = rules
                .iter()
                .filter(|rule| validate_rule(rule, *value))
                .collect::<Vec<&Rule>>();

            if positions.contains_key(&idx) {
                let applied_rules = positions.get(&idx).unwrap();
                let filtered = applied_rules
                    .iter()
                    .filter(|rule| applicable_rules.iter().any(|inner| inner.name == rule.name))
                    .map(|rule| rule.clone())
                    .collect::<Vec<&Rule>>();
                positions.insert(idx, filtered);
            } else {
                positions.insert(idx, applicable_rules);
            }
        }
    }

    let mut rules_map: HashMap<&Rule, Vec<usize>> = HashMap::new();
    for (p, rxs) in &positions {
        for r in rxs {
            if rules_map.contains_key(r) {
                rules_map.get_mut(r).unwrap().push(*p);
            } else {
                rules_map.insert(r, vec![*p]);
            }
        }
    }

    let mut new_positions = positions.clone();
    for _ in 0..valid_tickets.len() {
        let check = new_positions.clone();
        for (x, rxs) in &check {
            if rxs.len() == 1 {
                // Remove it from other positions since it's only applicable to this position.
                for (i, _) in &positions {
                    if i != x {
                        let filtered = new_positions
                            .get(&i)
                            .unwrap()
                            .iter()
                            .filter(|rule| rule.name != rxs[0].name)
                            .map(|rule| rule.clone())
                            .collect::<Vec<&Rule>>();
                        new_positions.insert(*i, filtered);
                    }
                }
            }
        }
    }

    let my_ticket = input.split("your ticket:\n").collect::<Vec<&str>>()[1]
        .split("\n\nnearby tickets")
        .collect::<Vec<&str>>()[0]
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut total = 1;
    for (idx, value) in my_ticket.iter().enumerate() {
        if new_positions.get(&idx).unwrap()[0]
            .name
            .contains("departure")
        {
            total *= value;
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            71,
            part_1(
                "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            1,
            part_2(
                "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            )
        );
    }
}
