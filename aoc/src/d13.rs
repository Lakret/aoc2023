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

fn find_symmetry(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
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
            return (Some(row + 1), None);
        } else {
            row += 1;
        }
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
            return (None, Some(col + 1));
        } else {
            col += 1;
        }
    }

    panic!("didn't find reflection")
}

fn p1(patterns: &Vec<Pattern>) -> usize {
    patterns
        .iter()
        .map(|p| match find_symmetry(p) {
            (Some(row), None) => 100 * row,
            (None, Some(col)) => col,
            _ => unreachable!(),
        })
        .sum()
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
            test_patterns.iter().map(|p| find_symmetry(p)).collect::<Vec<_>>(),
            vec![(None, Some(5)), (Some(4), None)]
        );
        assert_eq!(p1(&test_patterns), 405);

        let patterns = parse_input(&fs::read_to_string("../inputs/d13").unwrap());
        assert_eq!(p1(&patterns), 34993);
    }
}
