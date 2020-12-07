use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day7() {
    let input = input_loader::read_input("src/day_7/input.txt");

    let part_1_result = part_1(&input, "shiny gold bag");
    println!("[DAY 7] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input, "shiny gold bag");
    println!("[DAY 7] Result for part 2: {}", part_2_result);
}

#[derive(Debug)]
struct Bag {
    name: String,
    children: Vec<Child>,
}

#[derive(Debug)]
struct Child {
    name: String,
    capacity: i64,
}

fn create_map(input: &str) -> HashMap<String, Bag> {
    let mut map: HashMap<String, Bag> = HashMap::new();

    for line in input.split("\n") {
        let parts = line.trim_end().split("contain").collect::<Vec<&str>>();
        let bag_name = parts[0].trim();
        let children = parts[1].trim();
        if children.trim().contains("no other bags") {
            map.insert(
                bag_name.replace("bags", "bag").replace(".", ""),
                Bag {
                    name: bag_name.replace("bags", "bag").replace(".", ""),
                    children: Vec::new(),
                },
            );
            continue;
        }

        let mut children_vec: Vec<Child> = Vec::new();

        for child in children.split(", ") {
            let capacity = match child.get(0..1) {
                Some(v) => v,
                None => panic!("Could not grab the capacity"),
            };
            let child_name = match child.get(2..) {
                Some(v) => v,
                None => panic!("Could not grab the capacity"),
            };

            children_vec.push(Child {
                name: child_name.replace("bags", "bag").replace(".", ""),
                capacity: capacity.parse::<i64>().unwrap(),
            });
        }

        map.insert(
            bag_name.replace("bags", "bag").replace(".", ""),
            Bag {
                name: bag_name.replace("bags", "bag").replace(".", ""),
                children: children_vec,
            },
        );
    }

    return map;
}

fn can_hold_bag(map: &HashMap<String, Bag>, current: &str, bag: &str) -> bool {
    let b = match map.get(current) {
        Some(v) => v,
        None => panic!("Failed to get {}", current),
    };

    // Check if any of the children can hold it.
    if b.children.iter().any(|x| x.name == bag) {
        return true;
    }

    // Recursively check the children of each child.
    return b.children.iter().any(|x| can_hold_bag(map, &x.name, bag));
}

fn cost(map: &HashMap<String, Bag>, current: &str) -> i64 {
    let b = match map.get(current) {
        Some(v) => v,
        None => panic!("Failed to get {}", current),
    };

    let mut total = 0;
    for child in b.children.iter() {
        total += child.capacity;
        total += child.capacity * cost(map, &child.name);
    }
    return total;
}

fn part_1(input: &str, bag: &str) -> i64 {
    let map = create_map(input);

    return map
        .iter()
        .filter(|(_, b)| can_hold_bag(&map, &b.name, bag))
        .count() as i64;
}

fn part_2(input: &str, bag: &str) -> i64 {
    let map = create_map(input);

    let total = cost(&map, bag);
    println!("Jox: {:?}", total);
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .to_string();
        assert_eq!(4, part_1(&input, "shiny gold bag"));
    }

    #[test]
    fn test_part_2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .to_string();
        assert_eq!(126, part_2(&input, "shiny gold bag"));
    }
}
