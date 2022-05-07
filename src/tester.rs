use std::collections::BTreeMap;
use std::fs;

use anyhow::Error;

use crate::executor::Executor;
use crate::types::ResultCase;

pub struct Tester {
    input_dir: String,
    output_dir: String,
    pub results: BTreeMap<u32, ResultCase>,
}

impl Tester {
    pub fn new(input_dir: String, output_dir: String) -> Self {
        Tester {
            input_dir: input_dir,
            output_dir: output_dir,
            results: BTreeMap::new(),
        }
    }
    pub fn test(&mut self, execute_program_path: &str) -> Result<(), Error> {
        let inputs = fetch_path_from_directory(&self.input_dir);
        let expected = fetch_path_from_directory(&self.output_dir);

        for ((idx, input_path), (_, expected_path)) in inputs.into_iter().zip(expected) {
            let run_script = format!("{} < {}", execute_program_path, input_path);
            let actual = Executor::run(&run_script).expect("Compile Error detected");
            let expected =
                fs::read_to_string(expected_path).expect("Something went wrong reading the file");
            let result_case = ResultCase {
                id: idx,
                expected: expected,
                actual: actual.1,
            };

            let _ = &self.results.insert(idx, result_case);
        }
        Ok(())
    }
}

fn fetch_path_from_directory(directory: &str) -> Vec<(u32, String)> {
    let mut paths = fs::read_dir(directory)
        .unwrap()
        .map(|result| result.unwrap().path().display().to_string())
        .collect::<Vec<String>>();
    paths.sort();
    let ret = paths
        .iter()
        .enumerate()
        .map(|(idx, path)| ((idx + 1) as u32, path.clone()))
        .collect();
    ret
}
