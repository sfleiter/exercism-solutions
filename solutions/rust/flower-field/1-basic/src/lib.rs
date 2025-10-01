const EMPTY: u8 = b' ';
const FLOWER: u8 = b'*';

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }
    let mut result: Vec<String> = Vec::with_capacity(garden.len());
    for l in 0..garden.len() {
        let mut result_line = String::with_capacity(garden[l].len());
        for c in 0..garden[l].len() {
            let cell = garden[l].as_bytes()[c];
            result_line.push(
                if  cell == EMPTY { calculate_empty_field_value(garden, l, c) }
                    else { cell as char } );
        }
        result.push(result_line);
    }
    result
}

fn calculate_empty_field_value(garden: &[&str], line: usize, column: usize) -> char {
    let mut value: u8 = 0;
    for nl in line as isize - 1..=line as isize + 1 {
        for nc in column as isize - 1..=column as isize + 1 {
            value += match (nl, nc) {
                (-1, _) => 0,
                (_, -1) => 0,
                (_, _) if nl as usize >= garden.len() || nc as usize >= garden[line].len() => 0,
                (_, _) if garden[nl as usize].as_bytes()[nc as usize] == FLOWER => 1,
                (_, _) => 0,
            }
        }
    }
    if 0 == value { ' ' } else { (b'0' + value) as char }
}
