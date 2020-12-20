use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day20() {
    let input = input_loader::read_input("src/day_20/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 20] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 20] Result for part 2: {}", part_2_result);
}

#[derive(Debug)]
struct Tile {
    id: i64,
    borders: Vec<String>,
    neighbors: Vec<i64>,
}

// --------A-------
// -              -
// -              -
// D              B
// -              -
// -              -
// --------C-------

fn parse_id(line: &str) -> i64 {
    return line.split(" ").collect::<Vec<&str>>()[1]
        .replace(":", "")
        .to_string()
        .parse::<i64>()
        .unwrap();
}

fn parse_borders(lines: &Vec<String>) -> Vec<String> {
    let mut border: Vec<String> = Vec::new();
    border.push(lines.first().unwrap().clone());
    border.push(lines.last().unwrap().clone());
    let mut c = String::new();
    let mut d = String::new();
    for line in lines {
        c.push(line.chars().nth(0).unwrap());
        d.push(line.chars().last().unwrap());
    }
    border.push(c);
    border.push(d);
    return border;
}

fn border_match(border: &str, borders: &Vec<String>) -> bool {
    for br in borders {
        if br == border {
            return true;
        }
    }
    return false;
}

fn part_1(input: &str) -> i64 {
    let mut tiles: Vec<Tile> = Vec::new();
    for group in input.split("\n\n") {
        let mut id = 0;
        let mut lines: Vec<String> = Vec::new();

        for line in group.split("\n") {
            if line.contains("Tile") {
                id = parse_id(line)
            } else {
                lines.push(line.to_string())
            }
        }
        tiles.push(Tile {
            id: id,
            borders: parse_borders(&lines),
            neighbors: Vec::new(),
        })
    }

    let mut match_map: HashMap<i64, i64> = HashMap::new();
    for (idx, tile) in tiles.iter().enumerate() {
        let mut num_of_matches = 0;
        for border in &tile.borders {
            for (inner_idx, inner_tile) in tiles.iter().enumerate() {
                if idx == inner_idx {
                    continue;
                }

                if border_match(border, &inner_tile.borders)
                    || border_match(
                        &border.chars().rev().collect::<String>(),
                        &inner_tile.borders,
                    )
                {
                    num_of_matches += 1;
                }
            }
        }
        match_map.insert(tile.id, num_of_matches);
    }

    let mut ans: i64 = 1;
    for (id, num_of_matches) in match_map {
        if num_of_matches == 2 {
            ans *= id;
        }
    }

    return ans;
}

fn part_2(input: &str) -> i64 {
    let mut tiles: Vec<Tile> = Vec::new();
    for group in input.split("\n\n") {
        let mut id = 0;
        let mut lines: Vec<String> = Vec::new();

        for line in group.split("\n") {
            if line.contains("Tile") {
                id = parse_id(line)
            } else {
                lines.push(line.to_string())
            }
        }
        tiles.push(Tile {
            id: id,
            borders: parse_borders(&lines),
            neighbors: Vec::new(),
        })
    }

    let mut tile_connections: HashMap<i64, Vec<i64>> = HashMap::new();
    for (idx, tile) in tiles.iter().enumerate() {
        for border in &tile.borders {
            for (inner_idx, inner_tile) in tiles.iter().enumerate() {
                if idx == inner_idx {
                    continue;
                }

                if border_match(border, &inner_tile.borders)
                    || border_match(
                        &border.chars().rev().collect::<String>(),
                        &inner_tile.borders,
                    )
                {
                    if tile_connections.contains_key(&tile.id) {
                        tile_connections
                            .get_mut(&tile.id)
                            .unwrap()
                            .push(inner_tile.id);
                    } else {
                        tile_connections.insert(tile.id, vec![inner_tile.id]);
                    }
                }
            }
        }
    }

    let mut edges: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut inner_edges: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut inners: HashMap<i64, Vec<i64>> = HashMap::new();
    for (k, v) in tile_connections.iter() {
        if v.len() == 2 {
            edges.insert(*k, v.clone());
        } else if v.len() == 3 {
            inner_edges.insert(*k, v.clone());
        } else if v.len() == 4 {
            inners.insert(*k, v.clone());
        }
    }
    println!("Edges: {:?}", edges);
    println!("\nInner edges: {:?}", inner_edges);
    println!("\nInners: {:?}", inners);

    return 100;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> String {
        return "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
            .to_string();
    }

    #[test]
    fn test_part_1() {
        assert_eq!(20899048083289, part_1(&get_data()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(273, part_2(&get_data()));
    }
}
