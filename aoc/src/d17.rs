use std::collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet};

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

static ALL_DIRECTIONS: [Direction; 4] = [Left, Right, Up, Down];

impl Direction {
    fn opposite(&self) -> Direction {
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
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn walk(&self, direction: Direction) -> Option<Pos> {
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
    fn max_row(&self) -> usize {
        self.rows.len()
    }

    fn max_col(&self) -> usize {
        self.rows[0].len()
    }

    fn heat_loss(&self, pos: &Pos) -> usize {
        self.rows[pos.row][pos.col]
    }

    fn next_moves(&self, pos: &Pos, prev_directions: &Vec<Direction>) -> Vec<(Pos, Direction)> {
        let banned_directions = get_banned_directions(prev_directions);

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

    fn visualize(&self, directions: &Vec<Direction>) {
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

fn get_banned_directions(prev_directions: &Vec<Direction>) -> HashSet<Direction> {
    if prev_directions.is_empty() {
        HashSet::new()
    } else {
        let mut banned = HashSet::new();
        banned.insert(prev_directions.last().unwrap().opposite());

        if prev_directions.len() >= 3 {
            let last_three_directions = prev_directions.iter().rev().take(3).collect::<HashSet<_>>();
            if last_three_directions.len() == 1 {
                for d in last_three_directions.into_iter() {
                    banned.insert(*d);
                }
            }
        }

        banned
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Distance {
    pos: Pos,
    heat_loss: usize,
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss).map(|ordering| ordering.reverse())
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
struct State {
    parent: Pos,
    prev_directions: Vec<Direction>,
    heat_loss: usize,
}

struct Route {
    prev_directions: Vec<Direction>,
    heat_loss: usize,
}

fn dijkstra(map: &Map) -> (HashMap<Pos, State>, usize) {
    let start = Pos { row: 0, col: 0 };
    let target = Pos { row: map.max_row() - 1, col: map.max_col() - 1 };
    let mut visited: HashSet<Pos> = HashSet::new();

    let mut distances: BinaryHeap<Distance> = BinaryHeap::new();
    distances.push(Distance { pos: start, heat_loss: 0 });

    let mut states = HashMap::new();
    states.insert(start, State { parent: start, prev_directions: vec![], heat_loss: 0 });

    while let Some(Distance { pos, .. }) = distances.pop() {
        // if pos == target {
        //     let heat_loss = states.get(&target).unwrap().heat_loss;
        //     return (states, heat_loss);
        // }

        if visited.contains(&pos) {
            // println!("VISITED {:?}", pos);
            continue;
        }

        // cloning state here to avoid keeping it borrowed during modification via entry API below
        let state = states.get(&pos).unwrap().clone();
        let heat_loss = state.heat_loss;
        // TODO: keep all possible routes (limited to 3) to the current node with corresponding costs
        // and consider all possible neighbours for each of those with corresponding costs
        for (neighbour, direction) in map.next_moves(&pos, &state.prev_directions) {
            if !visited.contains(&neighbour) {
                let new_heat_loss = heat_loss + map.heat_loss(&neighbour);

                let mut new_prev_directions = state.prev_directions.clone();
                new_prev_directions.push(direction);

                let possible_new_state =
                    State { parent: pos, prev_directions: new_prev_directions, heat_loss: new_heat_loss };
                let possible_new_distance = Distance { pos: neighbour, heat_loss: new_heat_loss };

                match states.entry(neighbour) {
                    Entry::Occupied(mut entry) => {
                        if new_heat_loss <= entry.get().heat_loss {
                            // dbg!(("optimized", entry.get().heat_loss, new_heat_loss));

                            entry.insert(possible_new_state);
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(possible_new_state);
                    }
                };

                distances.push(possible_new_distance);
            }
        }

        visited.insert(pos);
    }

    let heat_loss = states.get(&target).unwrap().heat_loss;
    return (states, heat_loss);
}

fn p1(map: &Map) -> usize {
    dijkstra(map).1
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

    #[test]
    fn p1_test() {
        let test_map = parse_input(TEST_INPUT);
        // dbg!(test_map.next_moves(&Pos { row: 0, col: 0 }, &vec![]));
        // dbg!(test_map.next_moves(&Pos { row: 1, col: 1 }, &vec![]));
        // dbg!(test_map.next_moves(&Pos { row: 1, col: 1 }, &vec![Right, Right, Right]));

        let (states, heat_loss) = dijkstra(&test_map);
        let end_state = states.get(&Pos { row: test_map.max_row() - 1, col: test_map.max_col() - 1 }).unwrap();
        test_map.visualize(&end_state.prev_directions);
        dbg!(&end_state);
        dbg!(heat_loss);

        let map = parse_input(&fs::read_to_string("../inputs/d17").unwrap());
    }
}
