const GRID_SIZE: usize = 300;

const fn power_level(x: usize, y: usize, serial_number: usize) -> isize {
    ((((x + 10) * y + serial_number) * (x + 10) / 100) % 10) as isize - 5
}

fn compute_power_level_table(serial_number: usize) -> [[isize; GRID_SIZE]; GRID_SIZE] {
    // I(x, y) = i(x, y) + I(x, y - 1) + I(x - 1, y) - I(x - 1, y - 1)
    let mut table = [[0; GRID_SIZE]; GRID_SIZE];

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            table[x][y] = power_level(x, y, serial_number);

            if x > 0 {
                table[x][y] += table[x - 1][y];
            }

            if y > 0 {
                table[x][y] += table[x][y - 1];
            }

            if x > 0 && y > 0 {
                table[x][y] -= table[x - 1][y - 1]
            }
        }
    }

    table
}

fn find_max_fixed_square(
    table: &[[isize; GRID_SIZE]; GRID_SIZE],
    size: usize,
) -> Option<((usize, usize), isize)> {
    let mut max_square = None;

    for x in 0..GRID_SIZE - size {
        for y in 0..GRID_SIZE - size {
            let total_power_level =
                table[x][y] + table[x + size][y + size] - table[x][y + size] - table[x + size][y];

            max_square = match max_square {
                Some((_, max_power_level)) if total_power_level <= max_power_level => max_square,
                _ => Some(((x + 1, y + 1), total_power_level)),
            }
        }
    }

    max_square
}

pub fn part1(serial_number: usize) -> Option<((usize, usize), isize)> {
    let table = compute_power_level_table(serial_number);

    find_max_fixed_square(&table, 3)
}

pub fn part2(serial_number: usize) -> Option<((usize, usize), usize, isize)> {
    let table = compute_power_level_table(serial_number);

    let mut max_square = None;

    for size in 1..=GRID_SIZE {
        max_square = match (max_square, find_max_fixed_square(&table, size)) {
            (Some((_, _, max_power_level)), Some((_, current_total_power_level)))
                if current_total_power_level <= max_power_level =>
            {
                max_square
            }

            (_, Some((current_max_square, current_total_power_level))) => {
                Some((current_max_square, size, current_total_power_level))
            }

            (_, None) => max_square,
        }
    }

    max_square
}

#[cfg(test)]
mod tests {
    use super::*;

    const SERIAL_NUMBER: usize = 2866;

    #[test]
    fn power_level_works() {
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(18), Some(((33, 45), 29)));
        assert_eq!(part1(42), Some(((21, 61), 30)));
        assert_eq!(part1(SERIAL_NUMBER), Some(((20, 50), 30)));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(18), Some(((90, 269), 16, 113)));
        assert_eq!(part2(42), Some(((232, 251), 12, 119)));
        assert_eq!(part2(SERIAL_NUMBER), Some(((238, 278), 9, 88)));
    }
}
