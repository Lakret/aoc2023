use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Galaxy {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
pub struct Image {
    galaxies: Vec<Galaxy>,
    expanded_rows: HashSet<usize>,
    expanded_cols: HashSet<usize>,
}

pub fn parse_input(input: &str) -> Image {
    let rows = input.trim().split("\n").collect::<Vec<_>>();

    let mut expanded_rows = (0..rows.len()).into_iter().collect::<HashSet<_>>();
    let mut expanded_cols = (0..rows[0].len()).into_iter().collect::<HashSet<_>>();
    let mut galaxies = vec![];

    for (row, cells) in rows.into_iter().enumerate() {
        for (col, ch) in cells.trim().chars().enumerate() {
            match ch {
                '#' => {
                    galaxies.push(Galaxy { row, col });
                    expanded_rows.remove(&row);
                    expanded_cols.remove(&col);
                }
                _ => (),
            }
        }
    }

    Image { galaxies, expanded_rows, expanded_cols }
}

pub fn distances(image: &Image, factor: usize) -> (HashMap<(usize, usize), usize>, usize) {
    let mut sum = 0;
    let mut pairwise = HashMap::new();

    for id1 in 0..image.galaxies.len() {
        for id2 in (id1 + 1)..image.galaxies.len() {
            let &Galaxy { row: row1, col: col1 } = &image.galaxies[id1];
            let &Galaxy { row: row2, col: col2 } = &image.galaxies[id2];

            let mut dist = row2.abs_diff(row1) + col2.abs_diff(col1);

            let walked_rows = row1.min(row2)..=row2.max(row1);
            for expanded_row in &image.expanded_rows {
                if walked_rows.contains(expanded_row) {
                    dist += factor;
                }
            }

            let walked_cols = col1.min(col2)..=col2.max(col1);
            for expanded_col in &image.expanded_cols {
                if walked_cols.contains(expanded_col) {
                    dist += factor;
                }
            }

            sum += dist;
            pairwise.insert((id1, id2), dist);
        }
    }

    (pairwise, sum)
}

pub fn p1(image: &Image) -> (HashMap<(usize, usize), usize>, usize) {
    distances(image, 1)
}

pub fn p2(image: &Image) -> (HashMap<(usize, usize), usize>, usize) {
    distances(image, 1000000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    static TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn parse_input_test() {
        let test_image = parse_input(TEST_INPUT);
        assert_eq!(&test_image.galaxies[0], &Galaxy { row: 0, col: 3 });
        assert_eq!(&test_image.galaxies[2], &Galaxy { row: 2, col: 0 });
        assert_eq!(&test_image.galaxies[5], &Galaxy { row: 6, col: 9 });
        assert_eq!(&test_image.galaxies[8], &Galaxy { row: 9, col: 4 });

        assert_eq!(test_image.expanded_rows, [3, 7].into_iter().collect::<HashSet<_>>());
        assert_eq!(test_image.expanded_cols, [2, 5, 8].into_iter().collect::<HashSet<_>>());
    }

    #[test]
    fn p1_test() {
        let test_image = parse_input(TEST_INPUT);
        let (pairwise, sum) = p1(&test_image);
        assert_eq!(pairwise.len(), 36);
        assert_eq!(pairwise.get(&(4, 8)), Some(&9));
        assert_eq!(pairwise.get(&(0, 6)), Some(&15));
        assert_eq!(pairwise.get(&(2, 5)), Some(&17));
        assert_eq!(pairwise.get(&(7, 8)), Some(&5));
        assert_eq!(pairwise.get(&(0, 1)), Some(&6));
        assert_eq!(pairwise.get(&(0, 2)), Some(&6));
        assert_eq!(sum, 374);

        let (_pairwise, sum) = distances(&test_image, 9);
        assert_eq!(sum, 1030);

        let (_pairwise, sum) = distances(&test_image, 99);
        assert_eq!(sum, 8410);

        let image = parse_input(&fs::read_to_string("../inputs/d11").unwrap());
        let (_pairwise, sum) = p1(&image);
        assert_eq!(sum, 9233514);
    }

    #[test]
    fn p2_test() {
        let test_image = parse_input(TEST_INPUT);
        let (_pairwise, sum) = distances(&test_image, 9);
        assert_eq!(sum, 1030);

        let (_pairwise, sum) = distances(&test_image, 99);
        assert_eq!(sum, 8410);

        let image = parse_input(&fs::read_to_string("../inputs/d11").unwrap());
        let (_pairwise, sum) = p2(&image);
        assert_eq!(sum, 363293506944);
    }
}
