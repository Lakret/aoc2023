use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Distance {
    pos: Pos,
    heat_loss: usize,
    direction: Direction,
    prev_directions: Vec<Direction>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Route {
    parent: Pos,
    prev_directions: Vec<Direction>,
    heat_loss: usize,
}

fn dijkstra(map: &Map) -> (Vec<usize>, usize) {
    let start = Pos { row: 0, col: 0 };
    let target = Pos { row: map.max_row() - 1, col: map.max_col() - 1 };
    let mut visited: HashSet<(Pos, Direction, usize)> = HashSet::new();

    let mut heap: BinaryHeap<Distance> = BinaryHeap::new();
    heap.push(Distance { pos: start, heat_loss: 0, direction: Right, prev_directions: vec![] });
    heap.push(Distance { pos: start, heat_loss: 0, direction: Down, prev_directions: vec![] });

    let mut distances_by_direction = HashMap::new();
    for direction in ALL_DIRECTIONS {
        distances_by_direction.insert((start, direction), 0);
    }

    while let Some(Distance { pos, heat_loss, direction, prev_directions }) = heap.pop() {
        let remaining_steps =
            3 - prev_directions.iter().rev().take(3).take_while(|prev_dir| **prev_dir == direction).count();

        if visited.contains(&(pos, direction, remaining_steps)) {
            // println!("VISITED {:?}", pos);
            continue;
        }

        // let prev_dist = distances_by_direction.get(&(pos, direction));
        // if prev_dist.is_some() && heat_loss > *prev_dist.unwrap() {
        //     continue;
        // }

        // cloning state here to avoid keeping it borrowed during modification via entry API below
        for (neighbour, new_direction) in map.next_moves(&pos, &prev_directions) {
            let remaining_steps =
                3 - prev_directions.iter().rev().take(3).take_while(|prev_dir| **prev_dir == new_direction).count();

            if !visited.contains(&(neighbour, new_direction, remaining_steps)) {
                let new_heat_loss = heat_loss + map.heat_loss(&neighbour);

                let mut new_prev_directions = prev_directions.clone();
                new_prev_directions.push(new_direction);

                // let possible_new_route =
                //     Route { parent: pos, prev_directions: new_prev_directions, heat_loss: new_heat_loss };
                let possible_new_distance = Distance {
                    pos: neighbour,
                    heat_loss: new_heat_loss,
                    direction: new_direction,
                    prev_directions: new_prev_directions,
                };

                match distances_by_direction.entry((neighbour, new_direction)) {
                    Entry::Occupied(mut entry) => {
                        // let neighbour_routes = entry.get_mut();
                        // if neighbour_routes.iter().any(|r| new_heat_loss <= r.heat_loss) {
                        // neighbour_routes.insert(possible_new_route);
                        // distances_by_direction = ;
                        // }
                        if new_heat_loss < *entry.get() {
                            // dbg!(("optimized", entry.get().heat_loss, new_heat_loss));
                            // entry.insert(possible_new_route);
                            entry.insert(new_heat_loss);
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(new_heat_loss);
                    }
                };

                heap.push(possible_new_distance);
            }
        }

        let remaining_steps =
            3 - prev_directions.iter().rev().take(3).take_while(|prev_dir| **prev_dir == direction).count();
        visited.insert((pos, direction, remaining_steps));
    }

    // dbg!(&distances_by_direction);
    let mut target_distances = vec![];
    for direction in ALL_DIRECTIONS {
        if let Some(d) = distances_by_direction.get(&(target, direction)) {
            target_distances.push(*d);
        }
    }
    return (target_distances.clone(), *target_distances.iter().min().unwrap());
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

        let (target_distances, heat_loss) = dijkstra(&test_map);
        // test_map.visualize(states.prev_directions);
        dbg!(&target_distances);
        dbg!(heat_loss);
        let start_time = std::time::Instant::now();
        assert_eq!(p1(&test_map), 102);
        dbg!(std::time::Instant::now() - start_time);

        let map = parse_input(&fs::read_to_string("../inputs/d17").unwrap());
        assert_eq!(p1(&map), 886);
    }
}
