use std::cmp;

pub fn solve() {
    let seeds = include_str!("./seeds.txt");
    let seeds_to_soil = include_str!("./seed-to-soil.txt");
    let soil_to_fert = include_str!("./soil-to-fert.txt");
    let fert_to_water = include_str!("./fert-to-water.txt");
    let water_to_light = include_str!("./water-to-light.txt");
    let light_to_temp = include_str!("./light-to-temp.txt");
    let temp_to_hum = include_str!("./temp-to-hum.txt");
    let hum_to_loc = include_str!("./hum-to-loc.txt");

    let res = part_2(
        seeds,
        seeds_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_hum,
        hum_to_loc,
    );

    dbg!(res);
}

fn part_1(
    seeds: &str,
    seeds_to_soil: &str,
    soil_to_fert: &str,
    fert_to_water: &str,
    water_to_light: &str,
    light_to_temp: &str,
    temp_to_hum: &str,
    hum_to_loc: &str,
) -> u64 {
    let alamanc = Alamanc::parse(
        seeds,
        seeds_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_hum,
        hum_to_loc,
    );

    alamanc.get_min_loc()
}

fn part_2(
    seeds: &str,
    seeds_to_soil: &str,
    soil_to_fert: &str,
    fert_to_water: &str,
    water_to_light: &str,
    light_to_temp: &str,
    temp_to_hum: &str,
    hum_to_loc: &str,
) -> u64 {
    let alamanc = Alamanc::parse(
        seeds,
        seeds_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_hum,
        hum_to_loc,
    );

    alamanc.get_min_loc2()
}

#[derive(PartialEq, Debug, Clone)]
struct MapRange {
    source_start: u64,
    dest_start: u64,
    len: u64,
}

impl MapRange {
    fn from_str(input: &str) -> Self {
        let mut parts = input.split(" ");

        let dest_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_start = parts.next().unwrap().parse::<u64>().unwrap();
        let length = parts.next().unwrap().parse::<u64>().unwrap();

        MapRange {
            source_start,
            dest_start,
            len: length,
        }
    }

    fn get_mapping(&self, source: u64) -> Option<u64> {
        if source >= self.source_start && source < self.source_start + self.len {
            Some(self.dest_start + (source - self.source_start))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct MapRanges(Vec<MapRange>);

impl MapRanges {
    fn from_str(input: &str) -> Self {
        let ranges = input
            .split('\n')
            .map(MapRange::from_str)
            .collect::<Vec<_>>();

        Self(ranges)
    }

    fn get_mapping(&self, source: u64) -> u64 {
        let match_mapping = self
            .0
            .iter()
            .filter_map(|range| range.get_mapping(source))
            .collect::<Vec<_>>();

        assert!(match_mapping.len() <= 1);

        match match_mapping.first() {
            Some(dest) => *dest,
            _ => source,
        }
    }

    fn get_containing_range(&self, source: u64) -> Option<MapRange> {
        self.0
            .iter()
            .find(|range| range.get_mapping(source).is_some())
            .cloned()
    }
}

#[derive(PartialEq, Debug)]
struct Alamanc {
    seeds: Vec<u64>,
    seeds_to_soil: MapRanges,
    soil_to_fert: MapRanges,
    fert_to_water: MapRanges,
    water_to_light: MapRanges,
    light_to_temp: MapRanges,
    temp_to_hum: MapRanges,
    hum_to_loc: MapRanges,
}

impl Alamanc {
    fn parse(
        seeds: &str,
        seeds_to_soil: &str,
        soil_to_fert: &str,
        fert_to_water: &str,
        water_to_light: &str,
        light_to_temp: &str,
        temp_to_hum: &str,
        hum_to_loc: &str,
    ) -> Self {
        let seeds = seeds
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        Self {
            seeds,
            seeds_to_soil: MapRanges::from_str(seeds_to_soil),
            soil_to_fert: MapRanges::from_str(soil_to_fert),
            fert_to_water: MapRanges::from_str(fert_to_water),
            water_to_light: MapRanges::from_str(water_to_light),
            light_to_temp: MapRanges::from_str(light_to_temp),
            temp_to_hum: MapRanges::from_str(temp_to_hum),
            hum_to_loc: MapRanges::from_str(hum_to_loc),
        }
    }

    fn get_loc(&self, seed: u64) -> u64 {
        let soil = self.seeds_to_soil.get_mapping(seed);
        let fert = self.soil_to_fert.get_mapping(soil);
        let water = self.fert_to_water.get_mapping(fert);
        let light = self.water_to_light.get_mapping(water);
        let temp = self.light_to_temp.get_mapping(light);
        let hum = self.temp_to_hum.get_mapping(temp);
        let loc = self.hum_to_loc.get_mapping(hum);

        loc
    }

    fn get_min_loc(&self) -> u64 {
        self.seeds.iter().map(|s| self.get_loc(*s)).min().unwrap()
    }

    fn get_next_range_equal_len(&self, seed: u64) -> u64 {
        let Some(soil_range) = self.seeds_to_soil.get_containing_range(seed) else {return 1};

        let soil = soil_range.get_mapping(seed).unwrap();
        let remining_soil_range = soil_range.dest_start + soil_range.len - soil;

        let Some(fert_range) = self.soil_to_fert.get_containing_range(soil) else {return 1};
        let fret = fert_range.get_mapping(soil).unwrap();
        let remining_fert_range = fert_range.dest_start + fert_range.len - fret;

        let Some(water_range) = self.fert_to_water.get_containing_range(fret) else {return 1};
        let water = water_range.get_mapping(fret).unwrap();
        let remining_water_range = water_range.dest_start + water_range.len - water;

        let Some(light_range) = self.water_to_light.get_containing_range(water) else {return 1};
        let light = light_range.get_mapping(water).unwrap();
        let remining_light_range = light_range.dest_start + light_range.len - light;

        let Some(temp_range) = self.light_to_temp.get_containing_range(light) else {return 1};
        let temp = temp_range.get_mapping(light).unwrap();
        let remining_temp_range = temp_range.dest_start + temp_range.len - temp;

        let Some(hum_range) = self.temp_to_hum.get_containing_range(temp) else {return 1};
        let hum = hum_range.get_mapping(temp).unwrap();
        let remining_hum_range = hum_range.dest_start + hum_range.len - hum;

        let Some(loc_range) = self.hum_to_loc.get_containing_range(hum) else {return 1};
        let loc = loc_range.get_mapping(hum).unwrap();
        let remining_loc_range = loc_range.dest_start + loc_range.len - loc;

        [
            remining_soil_range,
            remining_fert_range,
            remining_water_range,
            remining_light_range,
            remining_temp_range,
            remining_hum_range,
            remining_loc_range,
        ]
        .iter()
        .min()
        .unwrap()
        .clone()
    }

    fn get_min_loc2(&self) -> u64 {
        self.seeds
            .chunks(2)
            .map(|c| {
                dbg!(c);
                let start = c[0];
                let len = c[1];

                let mut cur = start;
                let mut min = u64::MAX;

                while cur < start + len {
                    let cur_loc = self.get_loc(cur);
                    min = cmp::min(cur_loc, min);
                    let jump = self.get_next_range_equal_len(cur);
                    if jump > 1 {
                        dbg!(jump);
                    }
                    cur = cur + jump;
                }

                min
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_range_get_mapping_works() {
        let range = MapRange {
            source_start: 98,
            dest_start: 50,
            len: 2,
        };

        assert_eq!(range.get_mapping(97), None);
        assert_eq!(range.get_mapping(98), Some(50));
        assert_eq!(range.get_mapping(99), Some(51));
        assert_eq!(range.get_mapping(100), None);
    }

    #[test]
    fn map_ranges_get_mapping_works() {
        let ranges = MapRanges(vec![
            MapRange {
                source_start: 98,
                dest_start: 50,
                len: 2,
            },
            MapRange {
                source_start: 50,
                dest_start: 52,
                len: 48,
            },
        ]);

        assert_eq!(ranges.get_mapping(49), 49);
        assert_eq!(ranges.get_mapping(97), 99);
        assert_eq!(ranges.get_mapping(98), 50);
        assert_eq!(ranges.get_mapping(99), 51);
        assert_eq!(ranges.get_mapping(100), 100);
    }

    #[test]
    fn map_range_from_str_works() {
        let input = "50 98 2";
        assert_eq!(
            MapRange::from_str(input),
            MapRange {
                source_start: 98,
                dest_start: 50,
                len: 2
            }
        );
    }

    #[test]
    fn part_1_works() {
        let seeds = "79 14 55 13";

        let seed_to_soil = "50 98 2
52 50 48";

        let soil_to_fertilizer = "0 15 37
37 52 2
39 0 15";

        let fertilizer_to_water = "49 53 8
0 11 42
42 0 7
57 7 4";

        let water_to_light = "88 18 7
18 25 70";

        let light_to_temperature = "45 77 23
81 45 19
68 64 13";

        let temperature_to_humidity = "0 69 1
1 0 69";

        let humidity_to_location = "60 56 37
56 93 4";

        let res = part_1(
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        );

        assert_eq!(res, 35);
    }

    #[test]
    fn part_2_works() {
        let seeds = "79 14 55 13";

        let seed_to_soil = "50 98 2
52 50 48";

        let soil_to_fertilizer = "0 15 37
37 52 2
39 0 15";

        let fertilizer_to_water = "49 53 8
0 11 42
42 0 7
57 7 4";

        let water_to_light = "88 18 7
18 25 70";

        let light_to_temperature = "45 77 23
81 45 19
68 64 13";

        let temperature_to_humidity = "0 69 1
1 0 69";

        let humidity_to_location = "60 56 37
56 93 4";

        let res = part_2(
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        );

        assert_eq!(res, 46);
    }
}
