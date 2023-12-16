use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let mut patterns = vec![];

    for pattern in input.trim().split("\n\n") {
        let mut rows = vec![];
        let mut char_cols: HashMap<usize, Vec<_>> = HashMap::new();

        for row in pattern.split("\n") {
            let mut row_binary = vec![];

            for (col, ch) in row.chars().enumerate() {
                let ch = match ch {
                    '.' => "0",
                    '#' => "1",
                    _ => panic!("unknown character: {ch:#?}"),
                };

                row_binary.push(ch);
                char_cols.entry(col).and_modify(|char_col| char_col.push(ch)).or_insert(vec![ch]);
            }

            rows.push(u32::from_str_radix(&row_binary.join(""), 2).unwrap());
        }

        let cols = (0..(*char_cols.keys().max().unwrap() + 1))
            .into_iter()
            .map(|col| u32::from_str_radix(&char_cols[&col].join(""), 2).unwrap())
            .collect();
        patterns.push(Pattern { rows, cols });
    }

    patterns
}

fn find_symmetry(
    pattern: &Pattern,
    prev_answer: (Option<usize>, Option<usize>),
) -> Option<(Option<usize>, Option<usize>)> {
    let mut row = 0;
    while row < pattern.rows.len() - 1 {
        let mirror_half_size = (row + 1).min(pattern.rows.len() - row - 1);

        let mut reflected = true;
        for offset in 0..mirror_half_size {
            if pattern.rows[row - offset] != pattern.rows[row + offset + 1] {
                reflected = false;
                break;
            }
        }

        if reflected {
            // task uses 1-indexes
            let candidate = (Some(row + 1), None);
            if candidate != prev_answer {
                return Some(candidate);
            }
        }
        row += 1;
    }

    let mut col = 0;
    while col < pattern.cols.len() - 1 {
        let mirror_half_size = (col + 1).min(pattern.cols.len() - col - 1);

        let mut reflected = true;
        for offset in 0..mirror_half_size {
            if pattern.cols[col - offset] != pattern.cols[col + offset + 1] {
                reflected = false;
                break;
            }
        }

        if reflected {
            // same as above, adjust for 1-indexed task
            let candidate = (None, Some(col + 1));
            if candidate != prev_answer {
                return Some(candidate);
            }
        }
        col += 1;
    }

    None
}

fn symmetry_summary(symmetry: (Option<usize>, Option<usize>)) -> usize {
    match symmetry {
        (Some(row), None) => 100 * row,
        (None, Some(col)) => col,
        _ => unreachable!(),
    }
}

fn p1(patterns: &Vec<Pattern>) -> usize {
    patterns.iter().map(|p| symmetry_summary(find_symmetry(p, (None, None)).unwrap())).sum()
}

fn find_unsmudged_symmetry(pattern: &Pattern) -> Option<(Option<usize>, Option<usize>)> {
    let prev_answer = find_symmetry(pattern, (None, None)).unwrap();

    for smudge_row in 0..pattern.rows.len() {
        for smudge_col in 0..pattern.cols.len() {
            let mut unsmudged_pattern = pattern.clone();

            // we use bitwise xor to flip a corresponding bit located at (smudge_row, smudge_col)
            let row_bit = pattern.cols.len() - smudge_col - 1;
            let new_row_number = unsmudged_pattern.rows[smudge_row] ^ (1 << row_bit);
            unsmudged_pattern.rows[smudge_row] = new_row_number;

            let col_bit = pattern.rows.len() - smudge_row - 1;
            let new_col_number = unsmudged_pattern.cols[smudge_col] ^ (1 << col_bit);
            unsmudged_pattern.cols[smudge_col] = new_col_number;

            match find_symmetry(&unsmudged_pattern, prev_answer) {
                None => continue,
                Some(new_symmetry) => {
                    return Some(new_symmetry);
                }
            }
        }
    }

    None
}

fn p2(patterns: &Vec<Pattern>) -> usize {
    // if candidate != prev_answer {

    let mut sum = 0;
    for pattern in patterns {
        let unsmudged_symmetry = find_unsmudged_symmetry(pattern).unwrap();
        sum += symmetry_summary(unsmudged_symmetry);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    static TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn p1_test() {
        let test_patterns = parse_input(TEST_INPUT);
        assert_eq!(
            test_patterns.iter().map(|p| find_symmetry(p, (None, None)).unwrap()).collect::<Vec<_>>(),
            vec![(None, Some(5)), (Some(4), None)]
        );
        assert_eq!(p1(&test_patterns), 405);

        let patterns = parse_input(&fs::read_to_string("../inputs/d13").unwrap());
        assert_eq!(p1(&patterns), 34993);
    }

    #[test]
    fn p2_test() {
        let test_patterns = parse_input(TEST_INPUT);
        assert_eq!(p2(&test_patterns), 400);

        let patterns = parse_input(&fs::read_to_string("../inputs/d13").unwrap());
        assert_eq!(p2(&patterns), 29341);
    }
}
