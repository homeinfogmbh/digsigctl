use std::collections::HashMap;
use std::fs::read_to_string;

const PROC_MEMINFO: &str = "/proc/meminfo";

pub fn meminfo() -> Result<HashMap<String, usize>, std::io::Error> {
    Ok(read_to_string(PROC_MEMINFO)?
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(key, value)| {
            value
                .trim()
                .split_once(' ')
                .map_or(value.trim().parse::<usize>().ok(), |(value, unit)| {
                    value.trim().parse::<usize>().ok().map(|value| {
                        value
                            * match unit.trim() {
                                "kB" => 1000,
                                unit => panic!("Unknown unit: {unit}"),
                            }
                    })
                })
                .map(|value| (key.trim().to_string(), value))
        })
        .collect())
}
