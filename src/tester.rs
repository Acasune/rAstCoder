use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;

use crate::executor::{self, Executor};
use crate::types::ResultCase;

pub struct Tester {
    executor: Executor,
    input_dir: String,
    output_dir: String,
    pub results: HashMap<u32, ResultCase>,
}

impl Tester {
    pub fn new(executor: Executor, input_dir: String, output_dir: String) -> Self {
        Tester {
            executor: executor,
            input_dir: input_dir,
            output_dir: output_dir,
            results: HashMap::new(),
        }
    }
    pub fn test(&mut self) {
        let inputs = fetch_path_from_directory(&self.input_dir);
        let expected = fetch_path_from_directory(&self.output_dir);
        for ((idx, input_path), (_, expected_path)) in inputs.iter().zip(expected) {
            let script = self.build_run_script(input_path.clone());
            let actual = self.executor.run(script);
            let expected =
                fs::read_to_string(expected_path).expect("Something went wrong reading the file");
            let result_case = ResultCase {
                id: *idx,
                expected: expected,
                actual: actual,
            };

            &self.results.insert(*idx, result_case);
        }
        for (a, b) in &self.results {
            println!("actual: {}", b.actual);
            println!("expected: {}", b.expected);
        }
    }
    fn build_run_script(&self, path_test_data: String) -> String {
        format!(r#"./playground/a.out < {}"#, path_test_data)
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

#[cfg(test)]
mod tests {
    use crate::executor;

    use super::*;
    #[test]
    fn test_run_testcase() {
        let executor = Executor::new();
        let mut tester = Tester::new(
            executor,
            "./testcase/abc249_a/input".to_string(),
            "./testcase/abc249_a/output".to_string(),
        );
        tester.test();
    }
}
