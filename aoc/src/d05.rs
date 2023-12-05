use std::collections::HashMap;
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
    source: Category,
    destination: Category,
    source_range_start: u64,
    source_range_end: u64,
    destination_range_start: u64,
    destination_range_end: u64,
    length: u64,
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
                            source,
                            destination,
                            source_range_start,
                            destination_range_start,
                            length,
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
}
