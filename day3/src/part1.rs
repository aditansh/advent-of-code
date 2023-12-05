pub fn is_punctuation(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

pub fn check_line(input: &str, number: String, i: usize, j: usize) -> bool {
    let line = input.lines().nth(i).unwrap();
    let len = line.len();

    // if j == number.len() - 1 {
    //     print!("{} {} {}\n", line, number, j);
    // }

    // print!("{} {} {}\n", line, number, j);
    if j >= number.len() && is_punctuation(line.chars().nth(j - number.len()).unwrap()) {
        return true;
    }

    if j + 1 < len && is_punctuation(line.chars().nth(j + 1).unwrap()) {
        return true;
    }

    let mut x = j;
    if j <= number.len() - 1 {
        x = 0;
    } else {
        x = j - number.len() + 1;
    }

    for k in x..j + 1 {
        if k < len && k > 0 && is_punctuation(line.chars().nth(k).unwrap()) {
            return true;
        }
    }

    false
}

pub fn is_part(input: &str, number: u32, i: usize, j: usize) -> bool {
    if i > 0 && check_line(input, number.to_string(), i - 1, j) {
        return true;
    }

    if i < input.lines().count() - 1 && check_line(input, number.to_string(), i + 1, j) {
        return true;
    }

    check_line(input, number.to_string(), i, j)
}
