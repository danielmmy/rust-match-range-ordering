use std::usize;

pub fn match_crescent_ordering(n: usize) -> i8 {
    match n {
        0 => 9,
        1 => 8,
        2 => 7,
        3 => 6,
        4 => 5,
        5 => 4,
        6 => 3,
        7 => 2,
        8 => 1,
        9 => 0,
        _ => -1,
    }
}

pub fn match_decrescent_ordering(n: usize) -> i8 {
    match n {
        9 => 0,
        8 => 1,
        7 => 2,
        6 => 3,
        5 => 4,
        4 => 5,
        3 => 6,
        2 => 7,
        1 => 8,
        0 => 9,
        _ => -1,
    }
}

pub fn match_range_crescent_ordering(n: usize) -> i8 {
    match n {
        0..10 => 0,
        10..20 => 1,
        20..30 => 2,
        30..40 => 3,
        40..50 => 4,
        50..60 => 5,
        60..70 => 6,
        70..80 => 7,
        80..90 => 8,
        90..100 => 9,
        _ => -1,
    }
}

pub fn match_range_decrescent_ordering(n: usize) -> i8 {
    match n {
        90..100 => 9,
        80..90 => 8,
        70..80 => 7,
        60..70 => 6,
        50..60 => 5,
        40..50 => 4,
        30..40 => 3,
        20..30 => 2,
        10..20 => 1,
        0..10 => 0,
        _ => -1,
    }
}

pub fn run_many(numbers: &[usize], f: &dyn Fn(usize) -> i8) -> u64 {
    let mut acc: u64 = 0;
    for n in numbers {
        acc += f(*n) as u64;
    }
    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_crescent_ordering_test() {
        // arrange
        let test_table: Vec<(usize, i8)> = vec![
            (0, 9),
            (1, 8),
            (2, 7),
            (3, 6),
            (4, 5),
            (5, 4),
            (6, 3),
            (7, 2),
            (8, 1),
            (9, 0),
            (10, -1),
        ];

        for (value, expected) in test_table {
            // act
            let res = match_crescent_ordering(value);

            // assert
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn match_decrescent_ordering_test() {
        // arrange
        let test_table: Vec<(usize, i8)> = vec![
            (0, 9),
            (1, 8),
            (2, 7),
            (3, 6),
            (4, 5),
            (5, 4),
            (6, 3),
            (7, 2),
            (8, 1),
            (9, 0),
            (10, -1),
        ];

        for (value, expected) in test_table {
            // act
            let res = match_decrescent_ordering(value);

            // assert
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn match_range_crescent_ordering_test() {
        // arrange
        let test_table: Vec<(usize, i8)> = vec![
            (0, 0),
            (5, 0),
            (10, 1),
            (15, 1),
            (20, 2),
            (25, 2),
            (30, 3),
            (35, 3),
            (40, 4),
            (45, 4),
            (50, 5),
            (55, 5),
            (60, 6),
            (65, 6),
            (70, 7),
            (75, 7),
            (80, 8),
            (85, 8),
            (90, 9),
            (95, 9),
            (100, -1),
        ];

        for (value, expected) in test_table {
            // act
            let res = match_range_crescent_ordering(value);

            // assert
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn match_range_decrescent_ordering_test() {
        // arrange
        let test_table: Vec<(usize, i8)> = vec![
            (0, 0),
            (5, 0),
            (10, 1),
            (15, 1),
            (20, 2),
            (25, 2),
            (30, 3),
            (35, 3),
            (40, 4),
            (45, 4),
            (50, 5),
            (55, 5),
            (60, 6),
            (65, 6),
            (70, 7),
            (75, 7),
            (80, 8),
            (85, 8),
            (90, 9),
            (95, 9),
            (100, -1),
        ];

        for (value, expected) in test_table {
            // act
            let res = match_range_decrescent_ordering(value);

            // assert
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn run_many_test() {
        // arrange
        let data: Vec<usize> = vec![0, 1, 10, 11, 20, 22];

        // act
        let res1 = run_many(&data, &match_range_crescent_ordering);
        let res2 = run_many(&data, &match_range_crescent_ordering);

        // assert
        assert_eq!(res1, 6);
        assert_eq!(res2, 6);
    }
}
