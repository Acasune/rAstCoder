use std::collections::BTreeMap;
use std::fs;

use anyhow::Error;

use crate::executor::Executor;
use crate::types::ResultCase;

pub struct Tester {
    executor: Executor,
    input_dir: String,
    output_dir: String,
    pub results: BTreeMap<u32, ResultCase>,
}

impl Tester {
    pub fn new(executor: Executor, input_dir: String, output_dir: String) -> Self {
        Tester {
            executor: executor,
            input_dir: input_dir,
            output_dir: output_dir,
            results: BTreeMap::new(),
        }
    }
    pub fn test(&mut self) -> Result<(), Error> {
        let inputs = fetch_path_from_directory(&self.input_dir);
        let expected = fetch_path_from_directory(&self.output_dir);
        for ((idx, input_path), (_, expected_path)) in inputs.into_iter().zip(expected) {
            let script = build_run_script(input_path.clone());
            let actual = self.executor.run(script).expect("Compile Error detected");
            let expected =
                fs::read_to_string(expected_path).expect("Something went wrong reading the file");
            let result_case = ResultCase {
                id: idx,
                expected: expected,
                actual: actual.1,
            };

            &self.results.insert(idx, result_case);
        }
        Ok(())
    }
}

fn build_run_script(path_test_data: String) -> String {
    format!(r#"./playground/a.out < {}"#, path_test_data)
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

    use super::*;
    #[test]
    fn test_run_testcase() {
        // let executor = Executor::new();
        // let mut tester = Tester::new(
        //     // executor,
        //     "./testcase/abc249_a/input".to_string(),
        //     "./testcase/abc249_a/output".to_string(),
        // );
        // tester.test();
    }
}
