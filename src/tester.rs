use std::fs::{self, File};
use std::io::Read;

use crate::executor::{self, Executor};

pub struct Tester {
    executor: Executor,
    dir: String,
}

impl Tester {
    pub fn new(executor: Executor, dir: String) -> Self {
        Tester {
            executor: executor,
            dir: dir,
        }
    }
    pub fn test(&self) {
        let mut results = vec![];
        let problems = fetch_path_from_directory(&self.dir);
        for problem in problems {
            let script = self.build_run_script(problem.1);
            let res = self.executor.run(script);
            results.push(res);
        }
        println!("{:?}", results);
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
        let tester = Tester::new(executor, "./testcase/abc249_a/input".to_string());
        tester.test();
    }
}
