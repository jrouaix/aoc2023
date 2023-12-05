use rayon::{iter::ParallelIterator, slice::ParallelSlice};

advent_of_code::solution!(5);

type U = u64;

#[derive(Default, Debug)]
struct Data {
  seeds: Vec<Seed>,
  seed_to_soil_map: Vec<Map>,
  soil_to_fertilizer_map: Vec<Map>,
  fertilizer_to_water_map: Vec<Map>,
  water_to_light_map: Vec<Map>,
  light_to_temperature_map: Vec<Map>,
  temperature_to_humidity_map: Vec<Map>,
  humidity_to_location_map: Vec<Map>,
}

#[derive(Debug)]
struct Map {
  source_range_start: U,
  dest_range_start: U,
  range_length: U,
}

fn follow_map(value: U, map: &[Map]) -> U {
  for bind in map {
    if bind.source_range_start <= value && value < bind.source_range_start + bind.range_length {
      return bind.dest_range_start + (value - bind.source_range_start);
    }
  }
  value
}

#[derive(Debug)]
struct Seed(U);
impl Seed {
  fn to_soil(&self, data: &Data) -> Soil {
    Soil(follow_map(self.0, &data.seed_to_soil_map))
  }
}
#[derive(Debug)]
struct Soil(U);
impl Soil {
  fn to_fertilizer(&self, data: &Data) -> Fertilizer {
    Fertilizer(follow_map(self.0, &data.soil_to_fertilizer_map))
  }
}
#[derive(Debug)]
struct Fertilizer(U);
impl Fertilizer {
  fn to_water(&self, data: &Data) -> Water {
    Water(follow_map(self.0, &data.fertilizer_to_water_map))
  }
}
#[derive(Debug)]
struct Water(U);
impl Water {
  fn to_light(&self, data: &Data) -> Light {
    Light(follow_map(self.0, &data.water_to_light_map))
  }
}
#[derive(Debug)]
struct Light(U);
impl Light {
  fn to_temperature(&self, data: &Data) -> Temperature {
    Temperature(follow_map(self.0, &data.light_to_temperature_map))
  }
}
#[derive(Debug)]
struct Temperature(U);
impl Temperature {
  fn to_humidity(&self, data: &Data) -> Humidity {
    Humidity(follow_map(self.0, &data.temperature_to_humidity_map))
  }
}
#[derive(Debug)]
struct Humidity(U);
impl Humidity {
  fn to_location(&self, data: &Data) -> Location {
    Location(follow_map(self.0, &data.humidity_to_location_map))
  }
}
#[derive(Debug)]
struct Location(U);

fn read_data(input: &str) -> Option<Data> {
  let mut data = Data::default();

  let mut current_map: Option<&mut Vec<Map>> = None;

  for line in input.lines() {
    if line.starts_with("seeds:") {
      data.seeds = line.split(": ").nth(1)?.split_whitespace().map(|s| Seed(s.parse().unwrap())).collect();
    }

    if line.contains("seed-to-soil map:") {
      current_map = Some(&mut data.seed_to_soil_map);
    }
    if line.contains("soil-to-fertilizer map:") {
      current_map = Some(&mut data.soil_to_fertilizer_map);
    }
    if line.contains("fertilizer-to-water map:") {
      current_map = Some(&mut data.fertilizer_to_water_map);
    }
    if line.contains("water-to-light map:") {
      current_map = Some(&mut data.water_to_light_map);
    }
    if line.contains("light-to-temperature map:") {
      current_map = Some(&mut data.light_to_temperature_map);
    }
    if line.contains("temperature-to-humidity map:") {
      current_map = Some(&mut data.temperature_to_humidity_map);
    }
    if line.contains("humidity-to-location map:") {
      current_map = Some(&mut data.humidity_to_location_map);
    }

    if let Some(map) = current_map.as_deref_mut() {
      let mut parts = line.split_whitespace();
      if let (Some(dest_range_start), Some(source_range_start), Some(range_length)) = (parts.next(), parts.next(), parts.next()) {
        map.push(Map {
          source_range_start: source_range_start.parse().ok()?,
          dest_range_start: dest_range_start.parse().ok()?,
          range_length: range_length.parse().ok()?,
        });
      }
    }
  }

  Some(data)
}

pub fn part_one(input: &str) -> Option<U> {
  let data = read_data(input)?;
  data
    .seeds
    .iter()
    .map(|seed| {
      seed
        .to_soil(&data)
        .to_fertilizer(&data)
        .to_water(&data)
        .to_light(&data)
        .to_temperature(&data)
        .to_humidity(&data)
        .to_location(&data)
        .0
    })
    .min()
}

pub fn part_two(input: &str) -> Option<U> {
  let data = read_data(input)?;
  data
    .seeds
    .iter()
    .as_slice()
    .par_chunks(2)
    .flat_map_iter(|seeds| {
      let seed = seeds[0].0;
      let range = seeds[1].0;
      (seed..seed + range).map(Seed)
    })
    .map(|seed| {
      seed
        .to_soil(&data)
        .to_fertilizer(&data)
        .to_water(&data)
        .to_light(&data)
        .to_temperature(&data)
        .to_humidity(&data)
        .to_location(&data)
        .0
    })
    .min()
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = r#"seeds: 79 14 55 13

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
  56 93 4"#;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(175622908));
  }

  #[test]
  fn test_part_one_example() {
    let result = part_one(EXAMPLE);
    assert_eq!(result, Some(35));
  }

  #[test]
  #[ignore = "too long"]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(5200543));
  }

  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE);
    assert_eq!(result, Some(46));
  }
}
