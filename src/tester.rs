// This file is modeled after:
// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/utils/test_helper.rs

use std::fs::{self, File};
use std::io::Read;

use crate::utils::scanner::Scanner;

pub(crate) struct Tester {
    input: Vec<(String, Vec<u8>)>,
    output: Vec<(String, Vec<u8>)>,
}

impl Tester {
    pub fn new(input_directory: &str, output_directory: &str) -> Tester {
        let input_data = fetch_data_from_directory(input_directory);
        let output_data = fetch_data_from_directory(output_directory);
        Tester {
            input: input_data,
            output: output_data,
        }
    }
    pub fn solve_by_algorithm<F>(self, solution: F)
    where
        F: Fn(&mut Scanner<&[u8], &mut Vec<u8>>),
    {
        for ((input_label, input), (output_label, output)) in
            self.input.into_iter().zip(self.output)
        {
            println!("Test: {} {}", input_label, output_label);
            let mut writer = vec![];
            {
                let mut sc = Scanner::new(&input[..], &mut writer);
                solution(&mut sc);
            }
            assert_eq!(writer, output);
        }
    }
}

fn fetch_data_from_directory(directory: &str) -> Vec<(String, Vec<u8>)> {
    let mut file_names = fs::read_dir(directory)
        .unwrap()
        .map(|result| result.unwrap().path().display().to_string())
        .collect::<Vec<String>>();
    file_names.sort();
    file_names
        .into_iter()
        .map(|file| {
            let data = read_file(&file);
            (file, data)
        })
        .collect()
}

fn read_file(filepath: &str) -> Vec<u8> {
    let mut file = File::open(filepath).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_test_data() {
        let files = fetch_data_from_directory("./assets/DSL_1_A/in");
        assert_eq!("./assets/DSL_1_A/in/DSL_1_A_1.in", files[0].0);
    }
}
