use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day11() {
    let input = input_loader::read_input("src/day_11/input.txt");

    let part_1_result = part_1(&input);
    println!(
        "[DAY 11] Result for part 1: runs: {} and taken seats: {}",
        part_1_result.0, part_1_result.1
    );

    let part_2_result = part_2(&input);
    println!(
        "[DAY 11] Result for part 2: runs: {} and taken seats: {}",
        part_2_result.0, part_2_result.1
    );
}

#[derive(Debug, PartialEq, Clone)]
enum SeatType {
    Floor,
    Seat,
}

#[derive(Debug, PartialEq, Clone)]
enum SeatState {
    Empty,
    Taken,
    NotApplicable,
}

#[derive(Debug, Clone)]
struct Seat {
    row: i32,
    column: i32,
    seat_type: SeatType,
    seat_state: SeatState,
}

type SeatMap = HashMap<String, Seat>;

fn create_seat(character: char, row: i32, column: i32) -> Seat {
    if character == '.' {
        return Seat {
            row: row,
            column: column,
            seat_type: SeatType::Floor,
            seat_state: SeatState::NotApplicable,
        };
    }

    return Seat {
        row: row,
        column: column,
        seat_type: SeatType::Seat,
        seat_state: SeatState::Empty,
    };
}

fn create_seat_map(input: &str) -> SeatMap {
    let mut seat_map: SeatMap = HashMap::new();

    for (i, row) in input.trim_end().split("\n").enumerate() {
        for (j, character) in row.chars().enumerate() {
            seat_map.insert(
                format!("{}-{}", i, j),
                create_seat(character, i as i32, j as i32),
            );
        }
    }

    return seat_map;
}

fn get_adjacent_seats_part_1(row: i32, column: i32, seat_map: &SeatMap) -> Vec<&Seat> {
    let mut seat_keys: Vec<String> = Vec::new();
    seat_keys.push(format!("{}-{}", row - 1, column)); // Up
    seat_keys.push(format!("{}-{}", row, column - 1)); // Left
    seat_keys.push(format!("{}-{}", row, column + 1)); // Right
    seat_keys.push(format!("{}-{}", row + 1, column)); // Down
    seat_keys.push(format!("{}-{}", row - 1, column - 1)); // Up + Left
    seat_keys.push(format!("{}-{}", row - 1, column + 1)); // Up + Right
    seat_keys.push(format!("{}-{}", row + 1, column - 1)); // Down + Left
    seat_keys.push(format!("{}-{}", row + 1, column + 1)); // Down + Right

    let mut seats: Vec<&Seat> = Vec::new();
    for seat_key in seat_keys {
        match seat_map.get(&seat_key) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v)
                }
            }
            None => continue,
        };
    }
    return seats;
}

fn shuffle_map_part_1(seat_map: &mut SeatMap) -> bool {
    let cloned_seat_map = seat_map.clone();
    let keys = cloned_seat_map.keys();

    let mut state_changed = false;
    for key in keys {
        let seat = match cloned_seat_map.get(key) {
            Some(v) => v,
            None => panic!("Failed to get seat for key: {}", key),
        };
        if seat.seat_type == SeatType::Floor {
            continue;
        }

        let adjacent_seats = get_adjacent_seats_part_1(seat.row, seat.column, &cloned_seat_map);
        if seat.seat_state == SeatState::Empty {
            if adjacent_seats
                .iter()
                .all(|seat| seat.seat_state == SeatState::Empty)
            {
                seat_map.insert(
                    key.to_string(),
                    Seat {
                        row: seat.row,
                        column: seat.column,
                        seat_type: SeatType::Seat,
                        seat_state: SeatState::Taken,
                    },
                );
                state_changed = true;
            }
        }
        if seat.seat_state == SeatState::Taken {
            if adjacent_seats
                .iter()
                .filter(|seat| seat.seat_state == SeatState::Taken)
                .count()
                >= 4
            {
                seat_map.insert(
                    key.to_string(),
                    Seat {
                        row: seat.row,
                        column: seat.column,
                        seat_type: SeatType::Seat,
                        seat_state: SeatState::Empty,
                    },
                );
                state_changed = true;
            }
        }
    }
    return state_changed;
}

fn count_taken_seats(seat_map: &SeatMap) -> i32 {
    return seat_map
        .iter()
        .filter(|(_, seat)| seat.seat_state == SeatState::Taken)
        .count() as i32;
}

fn part_1(input: &str) -> (i32, i32) {
    let mut seat_map = create_seat_map(input);

    let mut count = 1;
    loop {
        let state_changed = shuffle_map_part_1(&mut seat_map);
        if !state_changed {
            break;
        }
        count += 1;
    }

    return (count, count_taken_seats(&seat_map));
}

fn get_adjacent_seats_part_2(row: i32, column: i32, seat_map: &SeatMap) -> Vec<&Seat> {
    let mut seats: Vec<&Seat> = Vec::new();
    let mut row_ct = row.clone();
    // UP
    loop {
        row_ct -= 1;
        match seat_map.get(&format!("{}-{}", row_ct, column)) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v);
                    break;
                }
            }
            None => break,
        }
    }
    let mut column_ct = column.clone();
    // Left
    loop {
        column_ct -= 1;
        match seat_map.get(&format!("{}-{}", row, column_ct)) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v);
                    break;
                }
            }
            None => break,
        }
    }
    column_ct = column.clone();
    // Right
    loop {
        column_ct += 1;
        match seat_map.get(&format!("{}-{}", row, column_ct)) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v);
                    break;
                }
            }
            None => break,
        }
    }
    row_ct = row.clone();
    // Down
    loop {
        row_ct += 1;
        match seat_map.get(&format!("{}-{}", row_ct, column)) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v);
                    break;
                }
            }
            None => break,
        }
    }
    row_ct = row.clone();
    column_ct = column.clone();
    // Up + Left
    loop {
        row_ct -= 1;
        column_ct -= 1;
        match seat_map.get(&format!("{}-{}", row_ct, column_ct)) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v);
                    break;
                }
            }
            None => break,
        }
    }
    row_ct = row.clone();
    column_ct = column.clone();
    // Up + Right
    loop {
        row_ct -= 1;
        column_ct += 1;
        match seat_map.get(&format!("{}-{}", row_ct, column_ct)) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v);
                    break;
                }
            }
            None => break,
        }
    }
    row_ct = row.clone();
    column_ct = column.clone();
    // Down + Left
    loop {
        row_ct += 1;
        column_ct -= 1;
        match seat_map.get(&format!("{}-{}", row_ct, column_ct)) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v);
                    break;
                }
            }
            None => break,
        }
    }
    row_ct = row.clone();
    column_ct = column.clone();
    // Down + Right
    loop {
        row_ct += 1;
        column_ct += 1;
        match seat_map.get(&format!("{}-{}", row_ct, column_ct)) {
            Some(v) => {
                if v.seat_type == SeatType::Seat {
                    seats.push(v);
                    break;
                }
            }
            None => break,
        }
    }

    return seats;
}

fn shuffle_map_part_2(seat_map: &mut SeatMap) -> bool {
    let cloned_seat_map = seat_map.clone();
    let keys = cloned_seat_map.keys();

    let mut state_changed = false;
    for key in keys {
        let seat = match cloned_seat_map.get(key) {
            Some(v) => v,
            None => panic!("Failed to get seat for key: {}", key),
        };
        if seat.seat_type == SeatType::Floor {
            continue;
        }

        let adjacent_seats = get_adjacent_seats_part_2(seat.row, seat.column, &cloned_seat_map);
        if seat.seat_state == SeatState::Empty {
            if adjacent_seats
                .iter()
                .all(|seat| seat.seat_state == SeatState::Empty)
            {
                seat_map.insert(
                    key.to_string(),
                    Seat {
                        row: seat.row,
                        column: seat.column,
                        seat_type: SeatType::Seat,
                        seat_state: SeatState::Taken,
                    },
                );
                state_changed = true;
            }
        }
        if seat.seat_state == SeatState::Taken {
            if adjacent_seats
                .iter()
                .filter(|seat| seat.seat_state == SeatState::Taken)
                .count()
                >= 5
            {
                seat_map.insert(
                    key.to_string(),
                    Seat {
                        row: seat.row,
                        column: seat.column,
                        seat_type: SeatType::Seat,
                        seat_state: SeatState::Empty,
                    },
                );
                state_changed = true;
            }
        }
    }
    return state_changed;
}

fn part_2(input: &str) -> (i32, i32) {
    let mut seat_map = create_seat_map(input);

    let mut count = 1;
    loop {
        let state_changed = shuffle_map_part_2(&mut seat_map);
        if !state_changed {
            break;
        }
        count += 1;
    }

    return (count, count_taken_seats(&seat_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .to_string();
        assert_eq!((6, 37), part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .to_string();
        assert_eq!((7, 26), part_2(&input));
    }
}
