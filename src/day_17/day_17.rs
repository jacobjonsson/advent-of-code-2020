use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day17() {
    let input = input_loader::read_input("src/day_17/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 17] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 17] Result for part 2: {}", part_2_result);
}

type Grid = HashMap<(i32, i32, i32), bool>;

fn step(grid: &Grid) -> Grid {
    let mut new_grid = HashMap::new();
    let x_keys = grid.keys().map(|x| x.0);
    let y_keys = grid.keys().map(|x| x.1);
    let z_keys = grid.keys().map(|x| x.2);

    for x in (x_keys.clone().min().unwrap() - 1)..(x_keys.clone().max().unwrap() + 2) {
        for y in (y_keys.clone().min().unwrap() - 1)..(y_keys.clone().max().unwrap() + 2) {
            for z in (z_keys.clone().min().unwrap() - 1)..(z_keys.clone().max().unwrap() + 2) {
                let v = match grid.get(&(x, y, z)) {
                    Some(v) => v,
                    None => &false,
                };

                let mut an = 0;

                for dx in [-1, 0, 1].iter() {
                    for dy in [-1, 0, 1].iter() {
                        for dz in [-1, 0, 1].iter() {
                            if *dx == 0 && *dy == 0 && *dz == 0 {
                                continue;
                            } else {
                                let v1 = match grid.get(&(x + dx, y + dy, z + dz)) {
                                    Some(v) => v,
                                    None => &false,
                                };
                                if *v1 {
                                    an += 1;
                                }
                            }
                        }
                    }
                }

                if *v {
                    if an == 2 || an == 3 {
                        new_grid.insert((x, y, z), true);
                    } else {
                        new_grid.insert((x, y, z), false);
                    }
                } else {
                    if an == 3 {
                        new_grid.insert((x, y, z), true);
                    } else {
                        new_grid.insert((x, y, z), false);
                    }
                }
            }
        }
    }

    return new_grid;
}

fn get_sum(grid: &Grid) -> i32 {
    let mut ans = 0;
    for (_, v) in grid {
        if *v {
            ans += 1;
        }
    }
    return ans;
}

fn part_1(input: &str) -> i32 {
    let mut grid: Grid = HashMap::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32, 0), ch == '#');
        }
    }

    let mut sum = 0;
    for _ in 1..7 {
        grid = step(&grid);
        sum = get_sum(&grid);
    }

    return sum;
}

type GridPart2 = HashMap<(i32, i32, i32, i32), bool>;

fn step_part_2(grid: &GridPart2) -> GridPart2 {
    let mut new_grid: GridPart2 = HashMap::new();
    let x_keys = grid.keys().map(|x| x.0);
    let y_keys = grid.keys().map(|x| x.1);
    let z_keys = grid.keys().map(|x| x.2);
    let w_keys = grid.keys().map(|x| x.3);

    for x in (x_keys.clone().min().unwrap() - 1)..(x_keys.clone().max().unwrap() + 2) {
        for y in (y_keys.clone().min().unwrap() - 1)..(y_keys.clone().max().unwrap() + 2) {
            for z in (z_keys.clone().min().unwrap() - 1)..(z_keys.clone().max().unwrap() + 2) {
                for w in (w_keys.clone().min().unwrap() - 1)..(w_keys.clone().max().unwrap() + 2) {
                    let v = match grid.get(&(x, y, z, w)) {
                        Some(v) => v,
                        None => &false,
                    };

                    let mut an = 0;

                    for dx in [-1, 0, 1].iter() {
                        for dy in [-1, 0, 1].iter() {
                            for dz in [-1, 0, 1].iter() {
                                for dw in [-1, 0, 1].iter() {
                                    if *dx == 0 && *dy == 0 && *dz == 0 && *dw == 0 {
                                        continue;
                                    } else {
                                        let v1 = match grid.get(&(x + dx, y + dy, z + dz, w + dw)) {
                                            Some(v) => v,
                                            None => &false,
                                        };
                                        if *v1 {
                                            an += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if *v {
                        if an == 2 || an == 3 {
                            new_grid.insert((x, y, z, w), true);
                        } else {
                            new_grid.insert((x, y, z, w), false);
                        }
                    } else {
                        if an == 3 {
                            new_grid.insert((x, y, z, w), true);
                        } else {
                            new_grid.insert((x, y, z, w), false);
                        }
                    }
                }
            }
        }
    }

    return new_grid;
}

fn get_sum_part_2(grid: &GridPart2) -> i32 {
    let mut ans = 0;
    for (_, v) in grid {
        if *v {
            ans += 1;
        }
    }
    return ans;
}

fn part_2(input: &str) -> i32 {
    let mut grid: GridPart2 = HashMap::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32, 0, 0), ch == '#');
        }
    }

    let mut sum = 0;
    for _ in 1..7 {
        grid = step_part_2(&grid);
        sum = get_sum_part_2(&grid);
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            112,
            part_1(
                ".#.
..#
###"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            848,
            part_2(
                ".#.
..#
###"
            )
        );
    }
}
