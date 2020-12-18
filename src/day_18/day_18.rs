#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day18() {
    let input = input_loader::read_input("src/day_18/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 18] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 18] Result for part 2: {}", part_2_result);
}

#[derive(Debug, Clone)]
struct Token {
    kind: String,
    value: String,
}

#[derive(Debug)]
struct Lexer {
    input: String,
    character: char,
    current: usize,
    tokens: Vec<Token>,
}

fn peek(lexer: &Lexer) -> char {
    return match lexer.input.chars().nth(lexer.current + 1) {
        Some(v) => v,
        None => 'E',
    };
}

fn advance_lexer(lexer: &mut Lexer) {
    lexer.current += 1;
    lexer.character = match lexer.input.chars().nth(lexer.current) {
        Some(v) => v,
        None => 'E',
    };
}

fn calc_tokens(tokens: &Vec<Token>) -> i64 {
    let mut total: i64 = 0;
    for (idx, token) in tokens.iter().enumerate() {
        if idx == 0 {
            total = tokens[idx].value.parse::<i64>().unwrap();
        }

        if token.value == "*" {
            total *= tokens[idx + 1].value.parse::<i64>().unwrap();
        } else if token.value == "+" {
            total += tokens[idx + 1].value.parse::<i64>().unwrap();
        }
    }
    return total;
}

fn parse_parentheses(lexer: &mut Lexer) -> Token {
    // Keep walking until we reach the end of the parentheses
    let mut tmp_lexer = Lexer {
        input: lexer.input[lexer.current + 1..].to_string(),
        character: lexer.input[lexer.current + 1..]
            .to_string()
            .chars()
            .nth(0)
            .unwrap(),
        current: 0,
        tokens: vec![],
    };

    while tmp_lexer.character != ')' {
        parse_next_token(&mut tmp_lexer);
    }

    lexer.current += tmp_lexer.current;
    // Skip over the next parenthesis.
    advance_lexer(lexer);
    return Token {
        value: calc_tokens(&tmp_lexer.tokens).to_string(),
        kind: "Number".to_string(),
    };
}

fn parse_next_token(lexer: &mut Lexer) {
    match lexer.character {
        'E' => {
            // Treat this as EOF.
            return;
        }
        '(' => {
            let token = parse_parentheses(lexer);
            lexer.tokens.push(token);
            advance_lexer(lexer);
        }
        ')' => panic!("parse_next_token should not see end parentheses"),
        ' ' => {
            advance_lexer(lexer);
        }
        '+' => {
            lexer.tokens.push(Token {
                value: "+".to_string(),
                kind: "Operator".to_string(),
            });
            advance_lexer(lexer);
        }
        '*' => {
            lexer.tokens.push(Token {
                value: "*".to_string(),
                kind: "Operator".to_string(),
            });
            advance_lexer(lexer);
        }
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            let mut digits = String::from(lexer.character);
            while peek(lexer).is_digit(10) {
                advance_lexer(lexer);
                digits.push(lexer.character);
            }
            lexer.tokens.push(Token {
                value: digits,
                kind: "Number".to_string(),
            });
            advance_lexer(lexer);
        }
        _ => {}
    }
}

fn part_1(input: &str) -> i64 {
    let mut total: i64 = 0;
    for line in input.split("\n") {
        let mut lexer = Lexer {
            input: line.to_string(),
            character: line.chars().nth(0).unwrap(),
            current: 0,
            tokens: vec![],
        };

        while lexer.current <= line.chars().count() - 1 {
            parse_next_token(&mut lexer);
        }

        total += calc_tokens(&lexer.tokens);
    }

    return total;
}

fn parse_value(value: &str) -> i64 {
    return value.parse().unwrap();
}

fn calc_tokens_part_2(tokens: &mut Vec<Token>) -> i64 {
    let mut new_tokens_clone: Vec<Token> = Vec::new();
    let mut cursor: usize = 0;
    while cursor < tokens.len() {
        if tokens[cursor].value == "+" {
            let prev = parse_value(&new_tokens_clone.last().unwrap().value);
            let next = parse_value(&tokens[cursor + 1].value);

            let last_index = new_tokens_clone.len() - 1;
            new_tokens_clone[last_index] = Token {
                kind: "Number".to_string(),
                value: (prev + next).to_string(),
            };
            cursor += 2;
            continue;
        }

        new_tokens_clone.push(tokens[cursor].clone());
        cursor += 1;
    }

    let mut total: i64 = 0;
    for (idx, token) in new_tokens_clone.iter().enumerate() {
        if idx == 0 {
            total = new_tokens_clone[idx].value.parse::<i64>().unwrap();
        }

        if token.value == "*" {
            total *= new_tokens_clone[idx + 1].value.parse::<i64>().unwrap();
        } else if token.value == "+" {
            panic!("Iffy");
        }
    }
    return total;
}

fn parse_parentheses_part_2(lexer: &mut Lexer) -> Token {
    // Keep walking until we reach the end of the parentheses
    let mut tmp_lexer = Lexer {
        input: lexer.input[lexer.current + 1..].to_string(),
        character: lexer.input[lexer.current + 1..]
            .to_string()
            .chars()
            .nth(0)
            .unwrap(),
        current: 0,
        tokens: vec![],
    };

    while tmp_lexer.character != ')' {
        parse_next_token_part_2(&mut tmp_lexer);
    }

    lexer.current += tmp_lexer.current;
    // Skip over the next parenthesis.
    advance_lexer(lexer);
    return Token {
        value: calc_tokens_part_2(&mut tmp_lexer.tokens).to_string(),
        kind: "Number".to_string(),
    };
}

fn parse_next_token_part_2(lexer: &mut Lexer) {
    match lexer.character {
        'E' => {
            // Treat this as EOF.
            return;
        }
        '(' => {
            let token = parse_parentheses_part_2(lexer);
            lexer.tokens.push(token);
            advance_lexer(lexer);
        }
        ')' => panic!("parse_next_token should not see end parentheses"),
        ' ' => {
            advance_lexer(lexer);
        }
        '+' => {
            lexer.tokens.push(Token {
                value: "+".to_string(),
                kind: "Operator".to_string(),
            });
            advance_lexer(lexer);
        }
        '*' => {
            lexer.tokens.push(Token {
                value: "*".to_string(),
                kind: "Operator".to_string(),
            });
            advance_lexer(lexer);
        }
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            let mut digits = String::from(lexer.character);
            while peek(lexer).is_digit(10) {
                advance_lexer(lexer);
                digits.push(lexer.character);
            }
            lexer.tokens.push(Token {
                value: digits,
                kind: "Number".to_string(),
            });
            advance_lexer(lexer);
        }
        _ => {}
    }
}

fn part_2(input: &str) -> i64 {
    let mut total: i64 = 0;
    for line in input.split("\n") {
        let mut lexer = Lexer {
            input: line.to_string(),
            character: line.chars().nth(0).unwrap(),
            current: 0,
            tokens: vec![],
        };

        while lexer.current <= line.chars().count() - 1 {
            parse_next_token_part_2(&mut lexer);
        }

        total += calc_tokens_part_2(&mut lexer.tokens);
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(51, part_1("1 + (2 * 3) + (4 * (5 + 6))"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            23340,
            part_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
