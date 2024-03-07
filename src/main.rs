use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct City {
    min: f64,
    max: f64,
    count: u32,
    total: f64,
}

fn main() {
    if let Ok(lines) = read_lines("measurements.txt") {
        let mut mapped_vals = HashMap::<String, City>::new();

        for line in lines.flatten() {
            let splitted: Vec<&str> = line.split(';').collect();

            let city_name = splitted[0].to_string();
            let measurement = splitted[1].to_string().parse::<f64>();

            if let Ok(measurement) = measurement {
                if mapped_vals.contains_key(&city_name) {
                    let val = mapped_vals.get_mut(&city_name).unwrap();

                    if measurement < val.min {
                        val.min = measurement;
                    }

                    if measurement > val.max {
                        val.max = measurement;
                    }

                    val.count += 1;
                    val.total += measurement;
                } else {
                    let city = City {
                        min: measurement,
                        max: measurement,
                        count: 1,
                        total: measurement,
                    };
                    mapped_vals.insert(city_name, city);
                }
            }
        }

        for (key, value) in mapped_vals.iter() {
            let avg = value.total / value.count as f64;
            println!(
                "City: {}, Min: {}, Max: {}, Avg: {}",
                key, value.min, value.max, avg
            );
        }
    }
}
