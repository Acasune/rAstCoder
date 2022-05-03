use run_script::{types::ScriptResult, ScriptOptions};
use std::process::{Command, Stdio};

pub struct Executor {
    source_program_path: String,
    execute_program_path: String,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            source_program_path: "./playground/main.rs".to_string(),
            execute_program_path: "./playground/a.out".to_string(),
        }
    }
    pub fn build(&self) -> ScriptResult<(i32, String, String)> {
        run_script::run_script!(
            r#"
            cargo build --release --offline --quiet --manifest-path=./playground/Cargo.toml
            cp ./playground/target/release/main ./playground/a.out
             "#
        )
    }
    pub fn run(&self, script: String) -> String {
        let (a, b, c) = run_script::run_script!(script).unwrap();
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::executor;

    use super::*;
    #[test]
    fn test_run_testcase() {
        let executor = Executor::new();
        // executor.run();
    }
}
