use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

static ALL_DIRECTIONS: [Direction; 4] = [Left, Right, Up, Down];

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        let ch = match self {
            Left => "<",
            Right => ">",
            Up => "^",
            Down => "v",
        };
        ch.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    pub fn walk(&self, direction: Direction) -> Option<Pos> {
        match direction {
            Left => {
                if self.col >= 1 {
                    Some(Pos { row: self.row, col: self.col - 1 })
                } else {
                    None
                }
            }
            Right => Some(Pos { row: self.row, col: self.col + 1 }),
            Up => {
                if self.row >= 1 {
                    Some(Pos { row: self.row - 1, col: self.col })
                } else {
                    None
                }
            }
            Down => Some(Pos { row: self.row + 1, col: self.col }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    rows: Vec<Vec<usize>>,
}

impl Map {
    pub fn max_row(&self) -> usize {
        self.rows.len()
    }

    pub fn max_col(&self) -> usize {
        self.rows[0].len()
    }

    pub fn heat_loss(&self, pos: &Pos) -> usize {
        self.rows[pos.row][pos.col]
    }

    pub fn next_moves(&self, pos: &Pos, direction: &Direction, steps: usize) -> Vec<(Pos, Direction)> {
        let mut banned_directions = vec![direction.opposite()];
        if steps == 3 {
            banned_directions.push(*direction);
        }
        let banned_directions = banned_directions.into_iter().collect::<HashSet<_>>();

        ALL_DIRECTIONS
            .into_iter()
            .filter(|d| !banned_directions.contains(d))
            .filter_map(|d| {
                pos.walk(d).and_then(|new_pos| {
                    if new_pos.row < self.max_row() && new_pos.col < self.max_col() {
                        Some((new_pos, d))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn next_moves2(&self, pos: &Pos, direction: &Direction, steps: usize) -> Vec<(Pos, Direction)> {
        let banned_directions = if steps == 10 {
            vec![direction.opposite(), *direction]
        } else if steps < 4 {
            ALL_DIRECTIONS.into_iter().filter(|d| d != direction).collect::<Vec<_>>()
        } else {
            vec![direction.opposite()]
        };
        let banned_directions = banned_directions.into_iter().collect::<HashSet<_>>();

        ALL_DIRECTIONS
            .into_iter()
            .filter(|d| !banned_directions.contains(d))
            .filter_map(|d| {
                pos.walk(d).and_then(|new_pos| {
                    if new_pos.row < self.max_row() && new_pos.col < self.max_col() {
                        Some((new_pos, d))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn visualize(&self, directions: &Vec<Direction>) {
        let mut rows: Vec<Vec<String>> = self.rows.iter().map(|r| r.iter().map(|d| d.to_string()).collect()).collect();

        let mut pos = Pos { row: 0, col: 0 };
        for direction in directions {
            pos = pos.walk(*direction).unwrap();
            rows[pos.row][pos.col] = direction.to_string();
        }

        for row in rows {
            println!("{}", row.concat());
        }
    }
}

pub fn parse_input(input: &str) -> Map {
    let mut rows = vec![];

    for row in input.trim().split("\n") {
        let row = row.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect();
        rows.push(row);
    }

    Map { rows }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Pos,
    heat_loss: usize,
    direction: Direction,
    steps: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss).map(|ordering| ordering.reverse())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn dijkstra(map: &Map, p2: bool) -> (Vec<usize>, usize) {
    let start = Pos { row: 0, col: 0 };
    let target = Pos { row: map.max_row() - 1, col: map.max_col() - 1 };
    // we can arrive to each block facing various directions and having made 0 to 3 steps in this direction
    // those are possible states we need to examine, so we a key of (position, direction, remaining_steps)
    // instead of just position to figure out which states we have already considered
    let mut visited: HashSet<(Pos, Direction, usize)> = HashSet::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State { pos: start, heat_loss: 0, direction: Right, steps: 0 });
    heap.push(State { pos: start, heat_loss: 0, direction: Down, steps: 0 });

    // in part 2, only state with >= 4 steps in a direction can count for reaching target,
    // so we need to remember the number of steps and direction to differentiate between different states
    let mut heat_losses = HashMap::new();
    for direction in ALL_DIRECTIONS {
        heat_losses.insert((start, direction, 0), 0);
    }

    while let Some(State { pos, heat_loss, direction, steps }) = heap.pop() {
        let next_moves_fun = if p2 { Map::next_moves2 } else { Map::next_moves };
        for (neighbour, new_direction) in next_moves_fun(map, &pos, &direction, steps + 1) {
            let steps = if new_direction == direction { steps + 1 } else { 0 };

            if !visited.contains(&(neighbour, new_direction, steps)) {
                let new_heat_loss = heat_loss + map.heat_loss(&neighbour);
                let new_state = State { pos: neighbour, heat_loss: new_heat_loss, direction: new_direction, steps };

                match heat_losses.entry((neighbour, new_direction, steps)) {
                    Entry::Occupied(mut entry) => {
                        if new_heat_loss < *entry.get() {
                            entry.insert(new_heat_loss);
                            heap.push(new_state);
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(new_heat_loss);
                        heap.push(new_state);
                    }
                };
            }
        }

        visited.insert((pos, direction, steps));
    }

    let mut target_distances = vec![];
    for direction in ALL_DIRECTIONS {
        for steps in (if p2 { 3 } else { 0 })..=10 {
            if let Some(d) = heat_losses.get(&(target, direction, steps)) {
                target_distances.push(*d);
            }
        }
    }
    return (target_distances.clone(), *target_distances.iter().min().unwrap());
}

fn p1(map: &Map) -> usize {
    dijkstra(map, false).1
}

fn p2(map: &Map) -> usize {
    dijkstra(map, true).1
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    static TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    static TEST_INPUT2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn p1_test() {
        let test_map = parse_input(TEST_INPUT);
        assert_eq!(p1(&test_map), 102);

        let map = parse_input(&fs::read_to_string("../inputs/d17").unwrap());
        let start_time = std::time::Instant::now();
        assert_eq!(p1(&map), 886);
        dbg!(std::time::Instant::now() - start_time);
    }

    #[test]
    fn p2_test() {
        let test_map = parse_input(TEST_INPUT);
        assert_eq!(p2(&test_map), 94);

        let test_map2 = parse_input(TEST_INPUT2);
        assert_eq!(p2(&test_map2), 71);

        let map = parse_input(&fs::read_to_string("../inputs/d17").unwrap());
        let start_time = std::time::Instant::now();
        assert_eq!(p2(&map), 1055);
        dbg!(std::time::Instant::now() - start_time);
    }
}
