pub fn is_neighbour(number: u32, k: usize, j: usize) -> bool {
    let len = number.to_string().len();
    if k + 1 == j {
        return true;
    }

    let mut start = 0;
    if k <= len - 1 {
        start = 0;
    } else {
        start = k - len + 1;
    }

    if start != 0 && j == start - 1 {
        return true;
    }

    if j >= start && j <= start + len - 1 {
        return true;
    }

    false
}

pub fn get_numbers(input: &str, i: usize, j: usize, numbers: &mut Vec<u32>) {
    let mut number = 0;
    let line = input.lines().nth(i).unwrap();

    for (k, char) in line.chars().enumerate() {
        if char.is_numeric() {
            number = number * 10 + char.to_digit(10).unwrap();
        }

        if (k + 1 == line.len() || !line.chars().nth(k + 1).unwrap().is_numeric()) && number != 0 {
            if is_neighbour(number, k, j) {
                numbers.push(number);
            }
            number = 0;
        }
    }
}

pub fn get_gear_ratio(input: &str, i: usize, j: usize) -> u32 {
    let mut product = 0;
    let mut numbers: Vec<u32> = Vec::new();

    get_numbers(input, i, j, &mut numbers);
    get_numbers(input, i + 1, j, &mut numbers);
    get_numbers(input, i - 1, j, &mut numbers);

    if numbers.len() == 2 {
        product = numbers[0] * numbers[1];
    }

    return product;
}