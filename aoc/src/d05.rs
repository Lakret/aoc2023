use std::collections::{HashMap, HashSet};
use Category::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Clone, Copy)]
pub struct RangeMap {
    destination: Category,
    source_range_start: u64,
    source_range_end: u64,
    destination_range_start: u64,
    destination_range_end: u64,
}

impl RangeMap {
    fn to_source_ids_range(&self) -> IdsRange {
        IdsRange {
            start: self.source_range_start,
            end: self.source_range_end,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    seeds: Vec<u64>,
    maps: HashMap<Category, Vec<RangeMap>>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.split("\n");
    let seeds = lines.next().unwrap();
    let seeds = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut maps = HashMap::new();
    for (header, source, destination) in [
        ("seed-to-soil map:", Seed, Soil),
        ("soil-to-fertilizer map:", Soil, Fertilizer),
        ("fertilizer-to-water map:", Fertilizer, Water),
        ("water-to-light map:", Water, Light),
        ("light-to-temperature map:", Light, Temperature),
        ("temperature-to-humidity map:", Temperature, Humidity),
        ("humidity-to-location map:", Humidity, Location),
    ] {
        parse_range_maps(&mut lines, &mut maps, header, source, destination);
    }

    Input { seeds, maps }
}

fn parse_range_maps<'a, 'b>(
    lines: &'b mut impl Iterator<Item = &'a str>,
    maps: &'b mut HashMap<Category, Vec<RangeMap>>,
    expected_header: &str,
    source: Category,
    destination: Category,
) {
    let mut header = "";
    while header == "" {
        header = lines.next().unwrap().trim();
    }

    let mut range_maps = vec![];
    if header == expected_header {
        loop {
            let line = lines.next().unwrap().trim();
            if line == "" {
                break;
            } else {
                match &line
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()[..]
                {
                    &[destination_range_start, source_range_start, length] => {
                        range_maps.push(RangeMap {
                            destination,
                            source_range_start,
                            destination_range_start,
                            source_range_end: source_range_start + length - 1,
                            destination_range_end: destination_range_start + length - 1,
                        });
                    }
                    x => {
                        panic!("unexpected map range definition: {:?}", x);
                    }
                }
            }
        }
    }
    range_maps.sort_by_key(|range_map| range_map.source_range_start);

    maps.insert(source, range_maps);
}

fn seed_to_location(input: &Input, seed: u64) -> u64 {
    let mut curr_category = Seed;
    let mut curr_id = seed;
    let mut dest_id = None;
    let mut dest_category = None;

    while let Some(range_maps) = input.maps.get(&curr_category) {
        dest_id = None;
        dest_category = None;

        for range_map in range_maps.iter() {
            dest_category = Some(range_map.destination);

            if curr_id >= range_map.source_range_start && curr_id <= range_map.source_range_end {
                dest_id = Some(
                    (curr_id - range_map.source_range_start) + range_map.destination_range_start,
                );
                break;
            }
        }

        if dest_id.is_none() {
            dest_id = Some(curr_id);
        }

        curr_id = dest_id.unwrap();
        curr_category = dest_category.unwrap();
    }

    if dest_category == Some(Location) {
        dest_id.unwrap()
    } else {
        panic!(
            "couldn't arrive at location, got stuck at {:?} category",
            dest_category
        )
    }
}

pub fn p1(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .map(|seed| seed_to_location(input, *seed))
        .min()
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IdsRange {
    start: u64,
    end: u64,
}

impl IdsRange {
    fn intersect_with(self: IdsRange, other: IdsRange) -> Vec<IdsRange> {
        // self is outside other => no split
        let res = if (self.start < other.start && self.end < other.start)
            || (self.start > other.end && self.end > other.end)
        {
            vec![self]
        // self intersects other from the end and self's end is contained inside other
        } else if self.start < other.start && self.end <= other.end {
            vec![
                IdsRange {
                    start: self.start,
                    end: other.start - 1,
                },
                IdsRange {
                    start: other.start,
                    end: self.end,
                },
            ]
        // ranges overlap [other.start, self.start, other.end, self.end]
        } else if self.start > other.start && self.start < other.end && self.end > other.end {
            vec![
                IdsRange {
                    start: self.start,
                    end: other.end,
                },
                IdsRange {
                    start: other.end + 1,
                    end: self.end,
                },
            ]
        // self starts in other and continues beyond other
        } else if self.start >= other.start && self.end > other.end {
            vec![
                IdsRange {
                    start: self.start,
                    end: other.end,
                },
                IdsRange {
                    start: other.end + 1,
                    end: self.end,
                },
            ]
            //   self is fully contained in other
        } else if self.start >= other.start && self.end <= other.end {
            vec![self]
        } else {
            panic!(
                "unexpected condition: self = {:?}, other = {:?}",
                self, other
            );
        };

        // println!(
        //     "for self = {:?} and other = {:?} got {:?}",
        //     self, other, res
        // );

        res
    }

    fn to_desintation_range(&self, maps: &[RangeMap]) -> IdsRange {
        IdsRange {
            start: apply_maps(self.start, maps),
            end: apply_maps(self.end, maps),
        }
    }
}

fn apply_maps(value: u64, maps: &[RangeMap]) -> u64 {
    match maps
        .iter()
        .find(|m| value >= m.source_range_start && value <= m.source_range_end)
    {
        Some(m) => value + m.destination_range_start - m.source_range_start,
        None => value,
    }
}

pub fn p2(input: &Input) -> u64 {
    let mut ranges = input
        .seeds
        .chunks(2)
        .map(|chunk| IdsRange {
            start: chunk[0],
            end: chunk[0] + chunk[1] - 1,
        })
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.start);
    dbg!(ranges.len());

    for source_category in [Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity] {
        let maps = input.maps.get(&source_category).unwrap();
        println!("Category: {:?}", source_category);
        dbg!(&ranges);

        let new_ranges =
            split_ranges_based_on_map_ranges(ranges, maps.iter().map(|m| m.to_source_ids_range()));

        dbg!((
            "[{}] # of ranges after splitting = {}",
            source_category,
            new_ranges.len()
        ));
        dbg!(&maps);
        dbg!(&new_ranges);

        // transform new_ranges to destination ranges
        ranges = new_ranges
            .into_iter()
            .map(|range| range.to_desintation_range(maps))
            .collect::<Vec<_>>();

        println!("transformed ranges:");
        dbg!(&ranges);
    }

    ranges.into_iter().map(|r| r.start).min().unwrap()
}

// split the ranges in such a way that each new range will be mapped to the destination via the same map
// or without using any maps
fn split_ranges_based_on_map_ranges(
    ranges: Vec<IdsRange>,
    map_ranges: impl Iterator<Item = IdsRange>,
) -> Vec<IdsRange> {
    let mut new_ranges = ranges;

    for map_source_ids_range in map_ranges {
        let mut new_ranges_updated = HashSet::new();

        for range in &new_ranges {
            for intersection_range in range.intersect_with(map_source_ids_range) {
                new_ranges_updated.insert(intersection_range);
            }
        }

        new_ranges = new_ranges_updated.into_iter().collect();
    }

    new_ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    static TEST_INPUT_RAW: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    ";

    #[test]
    fn p1_test() {
        let test_input = parse_input(TEST_INPUT_RAW);
        assert_eq!(seed_to_location(&test_input, 79), 82);
        assert_eq!(seed_to_location(&test_input, 14), 43);
        assert_eq!(seed_to_location(&test_input, 55), 86);
        assert_eq!(seed_to_location(&test_input, 13), 35);
        assert_eq!(p1(&test_input), 35);

        let input = parse_input(&fs::read_to_string("../inputs/d05").unwrap());
        assert_eq!(p1(&input), 173706076);
    }

    #[test]
    fn p2_test() {
        let test_input = parse_input(TEST_INPUT_RAW);
        assert_eq!(p2(&test_input), 46);

        let input = parse_input(&fs::read_to_string("../inputs/d05").unwrap());
        assert_eq!(p2(&input), 11611182);
    }

    #[test]
    fn split_ranges_based_on_map_ranges_test() {
        let ranges = vec![
            IdsRange { start: 57, end: 69 },
            IdsRange { start: 81, end: 94 },
        ];

        let map_ranges = vec![
            IdsRange { start: 11, end: 52 },
            IdsRange { start: 53, end: 60 },
        ];

        assert_eq!(
            ranges[0].intersect_with(map_ranges[0]),
            vec![IdsRange { start: 57, end: 69 }]
        );

        let mut splitted = split_ranges_based_on_map_ranges(ranges, map_ranges.into_iter());
        splitted.sort_by_key(|r| r.start);
        assert_eq!(
            splitted,
            vec![
                IdsRange { start: 57, end: 60 },
                IdsRange { start: 61, end: 69 },
                IdsRange { start: 81, end: 94 }
            ]
        );
    }
}
